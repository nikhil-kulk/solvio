use std::collections::{BTreeMap, HashMap};
use std::num::{NonZeroU32, NonZeroU64};

use api::grpc::conversions::{from_grpc_dist, payload_to_proto, proto_to_payloads};
use itertools::Itertools;
use segment::data_types::vectors::{NamedVector, VectorStruct, DEFAULT_VECTOR_NAME};
use segment::types::Distance;
use tonic::Status;

use super::config_diff::CollectionParamsDiff;
use crate::config::{
    default_replication_factor, default_write_consistency_factor, CollectionConfig,
    CollectionParams, WalConfig,
};
use crate::operations::config_diff::{HnswConfigDiff, OptimizersConfigDiff, WalConfigDiff};
use crate::operations::point_ops::PointsSelector::PointIdsSelector;
use crate::operations::point_ops::{
    Batch, FilterSelector, PointIdsList, PointStruct, PointsSelector,
};
use crate::operations::types::{
    CollectionInfo, CollectionStatus, CountResult, LookupLocation, OptimizersStatus,
    RecommendRequest, Record, SearchRequest, UpdateResult, UpdateStatus, VectorParams,
    VectorsConfig,
};
use crate::optimizers_builder::OptimizersConfig;
use crate::shards::remote_shard::CollectionSearchRequest;

impl From<api::grpc::solvio::HnswConfigDiff> for HnswConfigDiff {
    fn from(value: api::grpc::solvio::HnswConfigDiff) -> Self {
        Self {
            m: value.m.map(|v| v as usize),
            ef_construct: value.ef_construct.map(|v| v as usize),
            full_scan_threshold: value.full_scan_threshold.map(|v| v as usize),
            max_indexing_threads: value.max_indexing_threads.map(|v| v as usize),
            on_disk: value.on_disk,
            payload_m: value.payload_m.map(|v| v as usize),
        }
    }
}

impl From<api::grpc::solvio::WalConfigDiff> for WalConfigDiff {
    fn from(value: api::grpc::solvio::WalConfigDiff) -> Self {
        Self {
            wal_capacity_mb: value.wal_capacity_mb.map(|v| v as usize),
            wal_segments_ahead: value.wal_segments_ahead.map(|v| v as usize),
        }
    }
}

impl TryFrom<api::grpc::solvio::CollectionParamsDiff> for CollectionParamsDiff {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::CollectionParamsDiff) -> Result<Self, Self::Error> {
        Ok(Self {
            replication_factor: value
                .replication_factor
                .map(|factor| {
                    NonZeroU32::new(factor)
                        .ok_or_else(|| Status::invalid_argument("`replication_factor` cannot be 0"))
                })
                .transpose()?,
            write_consistency_factor: value
                .write_consistency_factor
                .map(|factor| {
                    NonZeroU32::new(factor).ok_or_else(|| {
                        Status::invalid_argument("`write_consistency_factor` cannot be 0")
                    })
                })
                .transpose()?,
        })
    }
}

impl From<api::grpc::solvio::OptimizersConfigDiff> for OptimizersConfigDiff {
    fn from(value: api::grpc::solvio::OptimizersConfigDiff) -> Self {
        Self {
            deleted_threshold: value.deleted_threshold,
            vacuum_min_vector_number: value.vacuum_min_vector_number.map(|v| v as usize),
            default_segment_number: value.default_segment_number.map(|v| v as usize),
            max_segment_size: value.max_segment_size.map(|v| v as usize),
            memmap_threshold: value.memmap_threshold.map(|v| v as usize),
            indexing_threshold: value.indexing_threshold.map(|v| v as usize),
            flush_interval_sec: value.flush_interval_sec,
            max_optimization_threads: value.max_optimization_threads.map(|v| v as usize),
        }
    }
}

impl From<CollectionInfo> for api::grpc::solvio::CollectionInfo {
    fn from(value: CollectionInfo) -> Self {
        let CollectionInfo {
            status,
            optimizer_status,
            vectors_count,
            indexed_vectors_count,
            points_count,
            segments_count,
            config,
            payload_schema,
        } = value;

        api::grpc::solvio::CollectionInfo {
            status: match status {
                CollectionStatus::Green => api::grpc::solvio::CollectionStatus::Green,
                CollectionStatus::Yellow => api::grpc::solvio::CollectionStatus::Yellow,
                CollectionStatus::Red => api::grpc::solvio::CollectionStatus::Red,
            }
            .into(),
            optimizer_status: Some(match optimizer_status {
                OptimizersStatus::Ok => api::grpc::solvio::OptimizerStatus {
                    ok: true,
                    error: "".to_string(),
                },
                OptimizersStatus::Error(error) => {
                    api::grpc::solvio::OptimizerStatus { ok: false, error }
                }
            }),
            vectors_count: vectors_count as u64,
            indexed_vectors_count: Some(indexed_vectors_count as u64),
            points_count: points_count as u64,
            segments_count: segments_count as u64,
            config: Some(api::grpc::solvio::CollectionConfig {
                params: Some(api::grpc::solvio::CollectionParams {
                    vectors_config: {
                        let config = match config.params.vectors {
                            VectorsConfig::Single(vector_params) => {
                                Some(api::grpc::solvio::vectors_config::Config::Params(
                                    vector_params.into(),
                                ))
                            }
                            VectorsConfig::Multi(vectors_params) => {
                                Some(api::grpc::solvio::vectors_config::Config::ParamsMap(
                                    api::grpc::solvio::VectorParamsMap {
                                        map: vectors_params
                                            .iter()
                                            .map(|(vector_name, vector_param)| {
                                                (vector_name.clone(), vector_param.clone().into())
                                            })
                                            .collect(),
                                    },
                                ))
                            }
                        };
                        Some(api::grpc::solvio::VectorsConfig { config })
                    },
                    shard_number: config.params.shard_number.get(),
                    replication_factor: Some(config.params.replication_factor.get()),
                    on_disk_payload: config.params.on_disk_payload,
                    write_consistency_factor: Some(config.params.write_consistency_factor.get()),
                }),
                hnsw_config: Some(api::grpc::solvio::HnswConfigDiff {
                    m: Some(config.hnsw_config.m as u64),
                    ef_construct: Some(config.hnsw_config.ef_construct as u64),
                    full_scan_threshold: Some(config.hnsw_config.full_scan_threshold as u64),
                    max_indexing_threads: Some(config.hnsw_config.max_indexing_threads as u64),
                    on_disk: config.hnsw_config.on_disk,
                    payload_m: config.hnsw_config.payload_m.map(|v| v as u64),
                }),
                optimizer_config: Some(api::grpc::solvio::OptimizersConfigDiff {
                    deleted_threshold: Some(config.optimizer_config.deleted_threshold),
                    vacuum_min_vector_number: Some(
                        config.optimizer_config.vacuum_min_vector_number as u64,
                    ),
                    default_segment_number: Some(
                        config.optimizer_config.default_segment_number as u64,
                    ),
                    max_segment_size: config.optimizer_config.max_segment_size.map(|x| x as u64),
                    memmap_threshold: config.optimizer_config.memmap_threshold.map(|x| x as u64),
                    indexing_threshold: Some(config.optimizer_config.indexing_threshold as u64),
                    flush_interval_sec: Some(config.optimizer_config.flush_interval_sec),
                    max_optimization_threads: Some(
                        config.optimizer_config.max_optimization_threads as u64,
                    ),
                }),
                wal_config: Some(api::grpc::solvio::WalConfigDiff {
                    wal_capacity_mb: Some(config.wal_config.wal_capacity_mb as u64),
                    wal_segments_ahead: Some(config.wal_config.wal_segments_ahead as u64),
                }),
            }),
            payload_schema: payload_schema
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<Record> for api::grpc::solvio::RetrievedPoint {
    fn from(record: Record) -> Self {
        let vectors = record.vector.map(|vector_struct| vector_struct.into());

        Self {
            id: Some(record.id.into()),
            payload: record.payload.map(payload_to_proto).unwrap_or_default(),
            vectors,
        }
    }
}

impl TryFrom<api::grpc::solvio::RetrievedPoint> for Record {
    type Error = Status;

    fn try_from(retrieved_point: api::grpc::solvio::RetrievedPoint) -> Result<Self, Self::Error> {
        let vectors = match retrieved_point.vectors {
            None => None,
            Some(vectors) => Some(vectors.try_into()?),
        };

        Ok(Self {
            id: retrieved_point.id.unwrap().try_into()?,
            payload: Some(proto_to_payloads(retrieved_point.payload)?),
            vector: vectors,
        })
    }
}

impl TryFrom<i32> for CollectionStatus {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CollectionStatus::Green),
            2 => Ok(CollectionStatus::Yellow),
            3 => Ok(CollectionStatus::Red),
            _ => Err(Status::invalid_argument("Malformed CollectionStatus type")),
        }
    }
}

impl From<api::grpc::solvio::OptimizersConfigDiff> for OptimizersConfig {
    fn from(optimizer_config: api::grpc::solvio::OptimizersConfigDiff) -> Self {
        Self {
            deleted_threshold: optimizer_config.deleted_threshold.unwrap_or_default(),
            vacuum_min_vector_number: optimizer_config
                .vacuum_min_vector_number
                .unwrap_or_default() as usize,
            default_segment_number: optimizer_config.default_segment_number.unwrap_or_default()
                as usize,
            max_segment_size: optimizer_config.max_segment_size.map(|x| x as usize),
            memmap_threshold: optimizer_config.memmap_threshold.map(|x| x as usize),
            indexing_threshold: optimizer_config.indexing_threshold.unwrap_or_default() as usize,
            flush_interval_sec: optimizer_config.flush_interval_sec.unwrap_or_default(),
            max_optimization_threads: optimizer_config
                .max_optimization_threads
                .unwrap_or_default() as usize,
        }
    }
}

impl From<api::grpc::solvio::WalConfigDiff> for WalConfig {
    fn from(wal_config: api::grpc::solvio::WalConfigDiff) -> Self {
        Self {
            wal_capacity_mb: wal_config.wal_capacity_mb.unwrap_or_default() as usize,
            wal_segments_ahead: wal_config.wal_segments_ahead.unwrap_or_default() as usize,
        }
    }
}

impl TryFrom<api::grpc::solvio::VectorParams> for VectorParams {
    type Error = Status;

    fn try_from(vector_params: api::grpc::solvio::VectorParams) -> Result<Self, Self::Error> {
        Ok(Self {
            size: NonZeroU64::new(vector_params.size).ok_or_else(|| {
                Status::invalid_argument("VectorParams size must be greater than zero")
            })?,
            distance: from_grpc_dist(vector_params.distance)?,
        })
    }
}

impl TryFrom<api::grpc::solvio::CollectionConfig> for CollectionConfig {
    type Error = Status;

    fn try_from(config: api::grpc::solvio::CollectionConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            params: match config.params {
                None => return Err(Status::invalid_argument("Malformed CollectionParams type")),
                Some(params) => CollectionParams {
                    vectors: match params.vectors_config {
                        None => {
                            return Err(Status::invalid_argument(
                                "Expected `vectors` - configuration for vector storage",
                            ))
                        }
                        Some(vector_config) => match vector_config.config {
                            None => {
                                return Err(Status::invalid_argument(
                                    "Expected `vectors` - configuration for vector storage",
                                ))
                            }
                            Some(api::grpc::solvio::vectors_config::Config::Params(params)) => {
                                VectorsConfig::Single(params.try_into()?)
                            }
                            Some(api::grpc::solvio::vectors_config::Config::ParamsMap(
                                params_map,
                            )) => VectorsConfig::Multi(
                                params_map
                                    .map
                                    .into_iter()
                                    .map(|(k, v)| Ok((k, v.try_into()?)))
                                    .collect::<Result<BTreeMap<String, VectorParams>, Status>>()?,
                            ),
                        },
                    },
                    shard_number: NonZeroU32::new(params.shard_number)
                        .ok_or_else(|| Status::invalid_argument("`shard_number` cannot be zero"))?,
                    on_disk_payload: params.on_disk_payload,
                    replication_factor: NonZeroU32::new(
                        params
                            .replication_factor
                            .unwrap_or_else(|| default_replication_factor().get()),
                    )
                    .ok_or_else(|| {
                        Status::invalid_argument("`replication_factor` cannot be zero")
                    })?,
                    write_consistency_factor: NonZeroU32::new(
                        params
                            .write_consistency_factor
                            .unwrap_or_else(|| default_write_consistency_factor().get()),
                    )
                    .ok_or_else(|| {
                        Status::invalid_argument("`write_consistency_factor` cannot be zero")
                    })?,
                },
            },
            hnsw_config: match config.hnsw_config {
                None => return Err(Status::invalid_argument("Malformed HnswConfig type")),
                Some(hnsw_config) => hnsw_config.into(),
            },
            optimizer_config: match config.optimizer_config {
                None => return Err(Status::invalid_argument("Malformed OptimizerConfig type")),
                Some(optimizer_config) => optimizer_config.into(),
            },
            wal_config: match config.wal_config {
                None => return Err(Status::invalid_argument("Malformed WalConfig type")),
                Some(wal_config) => wal_config.into(),
            },
        })
    }
}

impl TryFrom<api::grpc::solvio::GetCollectionInfoResponse> for CollectionInfo {
    type Error = Status;

    fn try_from(
        collection_info_response: api::grpc::solvio::GetCollectionInfoResponse,
    ) -> Result<Self, Self::Error> {
        match collection_info_response.result {
            None => Err(Status::invalid_argument("Malformed CollectionInfo type")),
            Some(collection_info_response) => Ok(Self {
                status: collection_info_response.status.try_into()?,
                optimizer_status: match collection_info_response.optimizer_status {
                    None => return Err(Status::invalid_argument("Malformed OptimizerStatus type")),
                    Some(api::grpc::solvio::OptimizerStatus { ok, error }) => {
                        if ok {
                            OptimizersStatus::Ok
                        } else {
                            OptimizersStatus::Error(error)
                        }
                    }
                },
                vectors_count: collection_info_response.vectors_count as usize,
                indexed_vectors_count: collection_info_response
                    .indexed_vectors_count
                    .unwrap_or_default() as usize,
                points_count: collection_info_response.points_count as usize,
                segments_count: collection_info_response.segments_count as usize,
                config: match collection_info_response.config {
                    None => {
                        return Err(Status::invalid_argument("Malformed CollectionConfig type"))
                    }
                    Some(config) => config.try_into()?,
                },
                payload_schema: collection_info_response
                    .payload_schema
                    .into_iter()
                    .map(|(k, v)| v.try_into().map(|v| (k, v)))
                    .try_collect()?,
            }),
        }
    }
}

impl TryFrom<api::grpc::solvio::PointStruct> for PointStruct {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::PointStruct) -> Result<Self, Self::Error> {
        let api::grpc::solvio::PointStruct {
            id,
            vectors,
            payload,
        } = value;

        let converted_payload = proto_to_payloads(payload)?;

        let vector_struct: VectorStruct = match vectors {
            None => return Err(Status::invalid_argument("Expected some vectors")),
            Some(vectors) => vectors.try_into()?,
        };

        Ok(Self {
            id: id
                .ok_or_else(|| Status::invalid_argument("Empty ID is not allowed"))?
                .try_into()?,
            vector: vector_struct,
            payload: Some(converted_payload),
        })
    }
}

impl TryFrom<PointStruct> for api::grpc::solvio::PointStruct {
    type Error = Status;

    fn try_from(value: PointStruct) -> Result<Self, Self::Error> {
        let vectors: api::grpc::solvio::Vectors = value.vector.into();

        let id = value.id;
        let payload = value.payload;

        let converted_payload = match payload {
            None => HashMap::new(),
            Some(payload) => payload_to_proto(payload),
        };

        Ok(Self {
            id: Some(id.into()),
            vectors: Some(vectors),
            payload: converted_payload,
        })
    }
}

impl TryFrom<Batch> for Vec<api::grpc::solvio::PointStruct> {
    type Error = Status;

    fn try_from(batch: Batch) -> Result<Self, Self::Error> {
        let mut points = Vec::new();
        let all_vectors = batch.vectors.into_all_vectors(batch.ids.len());
        for (i, p_id) in batch.ids.into_iter().enumerate() {
            let id = Some(p_id.into());
            let vector = all_vectors.get(i).cloned();
            let payload = batch.payloads.as_ref().and_then(|payloads| {
                payloads.get(i).map(|payload| match payload {
                    None => HashMap::new(),
                    Some(payload) => payload_to_proto(payload.clone()),
                })
            });
            let vectors: Option<VectorStruct> = vector.map(|v| v.into());

            let point = api::grpc::solvio::PointStruct {
                id,
                vectors: vectors.map(|v| v.into()),
                payload: payload.unwrap_or_default(),
            };
            points.push(point);
        }

        Ok(points)
    }
}

impl TryFrom<api::grpc::solvio::PointsSelector> for PointsSelector {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::PointsSelector) -> Result<Self, Self::Error> {
        match value.points_selector_one_of {
            Some(api::grpc::solvio::points_selector::PointsSelectorOneOf::Points(points)) => {
                Ok(PointIdsSelector(PointIdsList {
                    points: points
                        .ids
                        .into_iter()
                        .map(|p| p.try_into())
                        .collect::<Result<Vec<_>, _>>()?,
                }))
            }
            Some(api::grpc::solvio::points_selector::PointsSelectorOneOf::Filter(f)) => {
                Ok(PointsSelector::FilterSelector(FilterSelector {
                    filter: f.try_into()?,
                }))
            }
            _ => Err(Status::invalid_argument("Malformed PointsSelector type")),
        }
    }
}

impl From<UpdateResult> for api::grpc::solvio::UpdateResult {
    fn from(value: UpdateResult) -> Self {
        Self {
            operation_id: value.operation_id,
            status: match value.status {
                UpdateStatus::Acknowledged => api::grpc::solvio::UpdateStatus::Acknowledged as i32,
                UpdateStatus::Completed => api::grpc::solvio::UpdateStatus::Completed as i32,
            },
        }
    }
}

impl TryFrom<api::grpc::solvio::UpdateResult> for UpdateResult {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::UpdateResult) -> Result<Self, Self::Error> {
        Ok(Self {
            operation_id: value.operation_id,
            status: match value.status {
                status if status == api::grpc::solvio::UpdateStatus::Acknowledged as i32 => {
                    UpdateStatus::Acknowledged
                }
                status if status == api::grpc::solvio::UpdateStatus::Completed as i32 => {
                    UpdateStatus::Completed
                }
                _ => return Err(Status::invalid_argument("Malformed UpdateStatus type")),
            },
        })
    }
}

impl From<api::grpc::solvio::CountResult> for CountResult {
    fn from(value: api::grpc::solvio::CountResult) -> Self {
        Self {
            count: value.count as usize,
        }
    }
}

impl From<CountResult> for api::grpc::solvio::CountResult {
    fn from(value: CountResult) -> Self {
        Self {
            count: value.count as u64,
        }
    }
}

// Use wrapper type to bundle CollectionId & SearchRequest
impl<'a> From<CollectionSearchRequest<'a>> for api::grpc::solvio::SearchPoints {
    fn from(value: CollectionSearchRequest<'a>) -> Self {
        let (collection_id, request) = value.0;

        api::grpc::solvio::SearchPoints {
            collection_name: collection_id,
            vector: request.vector.get_vector().clone(),
            filter: request.filter.clone().map(|f| f.into()),
            limit: request.limit as u64,
            with_vectors: request.with_vector.clone().map(|wv| wv.into()),
            with_payload: request.with_payload.clone().map(|wp| wp.into()),
            params: request.params.map(|sp| sp.into()),
            score_threshold: request.score_threshold,
            offset: Some(request.offset as u64),
            vector_name: match request.vector.get_name() {
                DEFAULT_VECTOR_NAME => None,
                vector_name => Some(vector_name.to_string()),
            },
        }
    }
}

impl TryFrom<api::grpc::solvio::SearchPoints> for SearchRequest {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::SearchPoints) -> Result<Self, Self::Error> {
        Ok(SearchRequest {
            vector: match value.vector_name {
                Some(vector_name) => NamedVector {
                    name: vector_name,
                    vector: value.vector,
                }
                .into(),
                None => value.vector.into(),
            },
            filter: value.filter.map(|f| f.try_into()).transpose()?,
            params: value.params.map(|p| p.into()),
            limit: value.limit as usize,
            offset: value.offset.unwrap_or_default() as usize,
            with_payload: value.with_payload.map(|wp| wp.try_into()).transpose()?,
            with_vector: Some(
                value
                    .with_vectors
                    .map(|with_vectors| with_vectors.into())
                    .unwrap_or_default(),
            ),
            score_threshold: value.score_threshold,
        })
    }
}

impl From<api::grpc::solvio::LookupLocation> for LookupLocation {
    fn from(value: api::grpc::solvio::LookupLocation) -> Self {
        Self {
            collection: value.collection_name,
            vector: value.vector_name,
        }
    }
}

impl TryFrom<api::grpc::solvio::RecommendPoints> for RecommendRequest {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::RecommendPoints) -> Result<Self, Self::Error> {
        Ok(RecommendRequest {
            positive: value
                .positive
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<_, _>>()?,
            negative: value
                .negative
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<_, _>>()?,
            filter: value.filter.map(|f| f.try_into()).transpose()?,
            params: value.params.map(|p| p.into()),
            limit: value.limit as usize,
            offset: value.offset.unwrap_or_default() as usize,
            with_payload: value.with_payload.map(|wp| wp.try_into()).transpose()?,
            with_vector: Some(
                value
                    .with_vectors
                    .map(|with_vectors| with_vectors.into())
                    .unwrap_or_default(),
            ),
            score_threshold: value.score_threshold,
            using: value.using.map(|name| name.into()),
            lookup_from: value.lookup_from.map(|x| x.into()),
        })
    }
}

impl From<VectorParams> for api::grpc::solvio::VectorParams {
    fn from(value: VectorParams) -> Self {
        api::grpc::solvio::VectorParams {
            size: value.size.get(),
            distance: match value.distance {
                Distance::Cosine => api::grpc::solvio::Distance::Cosine,
                Distance::Euclid => api::grpc::solvio::Distance::Euclid,
                Distance::Dot => api::grpc::solvio::Distance::Dot,
            }
            .into(),
        }
    }
}
