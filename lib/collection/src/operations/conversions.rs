use std::collections::HashMap;
use std::num::{NonZeroU32, NonZeroU64};

use api::grpc::conversions::{payload_to_proto, proto_to_payloads};
use itertools::Itertools;
use segment::types::Distance;
use tonic::Status;

use crate::config::{CollectionConfig, CollectionParams, WalConfig};
use crate::operations::config_diff::{HnswConfigDiff, OptimizersConfigDiff, WalConfigDiff};
use crate::operations::point_ops::PointsSelector::PointIdsSelector;
use crate::operations::point_ops::{
    Batch, FilterSelector, PointIdsList, PointStruct, PointsSelector,
};
use crate::operations::types::{
    CollectionInfo, CollectionStatus, CountResult, OptimizersStatus, Record, UpdateResult,
    UpdateStatus,
};
use crate::optimizers_builder::OptimizersConfig;

impl From<api::grpc::solvio::HnswConfigDiff> for HnswConfigDiff {
    fn from(value: api::grpc::solvio::HnswConfigDiff) -> Self {
        Self {
            m: value.m.map(|v| v as usize),
            ef_construct: value.ef_construct.map(|v| v as usize),
            full_scan_threshold: value.full_scan_threshold.map(|v| v as usize),
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
            points_count,
            segments_count,
            disk_data_size,
            ram_data_size,
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
            points_count: points_count as u64,
            segments_count: segments_count as u64,
            disk_data_size: disk_data_size as u64,
            ram_data_size: ram_data_size as u64,
            config: Some(api::grpc::solvio::CollectionConfig {
                params: Some(api::grpc::solvio::CollectionParams {
                    vector_size: config.params.vector_size.get(),
                    distance: match config.params.distance {
                        segment::types::Distance::Cosine => api::grpc::solvio::Distance::Cosine,
                        segment::types::Distance::Euclid => api::grpc::solvio::Distance::Euclid,
                        segment::types::Distance::Dot => api::grpc::solvio::Distance::Dot,
                    }
                    .into(),
                    shard_number: config.params.shard_number.get(),
                    on_disk_payload: config.params.on_disk_payload,
                }),
                hnsw_config: Some(api::grpc::solvio::HnswConfigDiff {
                    m: Some(config.hnsw_config.m as u64),
                    ef_construct: Some(config.hnsw_config.ef_construct as u64),
                    full_scan_threshold: Some(config.hnsw_config.full_scan_threshold as u64),
                    max_indexing_threads: Some(config.hnsw_config.max_indexing_threads as u64),
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
                    flush_interval_sec: Some(config.optimizer_config.flush_interval_sec as u64),
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
        Self {
            id: Some(record.id.into()),
            payload: record.payload.map(payload_to_proto).unwrap_or_default(),
            vector: record.vector.unwrap_or_default(),
        }
    }
}

impl TryFrom<api::grpc::solvio::RetrievedPoint> for Record {
    type Error = Status;

    fn try_from(retrieved_point: api::grpc::solvio::RetrievedPoint) -> Result<Self, Self::Error> {
        Ok(Self {
            id: retrieved_point.id.unwrap().try_into()?,
            payload: Some(proto_to_payloads(retrieved_point.payload)?),
            vector: Some(retrieved_point.vector),
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

impl TryFrom<api::grpc::solvio::CollectionConfig> for CollectionConfig {
    type Error = Status;

    fn try_from(config: api::grpc::solvio::CollectionConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            params: match config.params {
                None => return Err(Status::invalid_argument("Malformed CollectionParams type")),
                Some(params) => CollectionParams {
                    vector_size: NonZeroU64::new(params.vector_size).unwrap(),
                    distance: match api::grpc::solvio::Distance::from_i32(params.distance) {
                        None => {
                            return Err(Status::invalid_argument(
                                "Malformed CollectionParams distance",
                            ))
                        }
                        Some(distance) => match distance {
                            api::grpc::solvio::Distance::UnknownDistance => {
                                return Err(Status::invalid_argument(
                                    "Malformed CollectionParams distance",
                                ))
                            }
                            api::grpc::solvio::Distance::Cosine => Distance::Cosine,
                            api::grpc::solvio::Distance::Euclid => Distance::Euclid,
                            api::grpc::solvio::Distance::Dot => Distance::Dot,
                        },
                    },
                    shard_number: NonZeroU32::new(params.shard_number).unwrap(),
                    on_disk_payload: params.on_disk_payload,
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
                points_count: collection_info_response.points_count as usize,
                segments_count: collection_info_response.segments_count as usize,
                disk_data_size: collection_info_response.disk_data_size as usize,
                ram_data_size: collection_info_response.ram_data_size as usize,
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
            vector,
            payload,
        } = value;

        let converted_payload = proto_to_payloads(payload)?;

        Ok(Self {
            id: id
                .ok_or_else(|| Status::invalid_argument("Empty ID is not allowed"))?
                .try_into()?,
            vector,
            payload: Some(converted_payload),
        })
    }
}

impl TryFrom<PointStruct> for api::grpc::solvio::PointStruct {
    type Error = Status;

    fn try_from(value: PointStruct) -> Result<Self, Self::Error> {
        let PointStruct {
            id,
            vector,
            payload,
        } = value;

        let converted_payload = match payload {
            None => HashMap::new(),
            Some(payload) => payload_to_proto(payload),
        };

        Ok(Self {
            id: Some(id.into()),
            vector,
            payload: converted_payload,
        })
    }
}

impl TryFrom<Batch> for Vec<api::grpc::solvio::PointStruct> {
    type Error = Status;

    fn try_from(value: Batch) -> Result<Self, Self::Error> {
        let mut points = Vec::new();
        for (i, p_id) in value.ids.into_iter().enumerate() {
            let id = Some(p_id.into());
            let vector = value.vectors.get(i).cloned();
            let payload = value.payloads.as_ref().and_then(|payloads| {
                payloads.get(i).map(|payload| match payload {
                    None => HashMap::new(),
                    Some(payload) => payload_to_proto(payload.clone()),
                })
            });
            let point = api::grpc::solvio::PointStruct {
                id,
                vector: vector.unwrap_or_default(),
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
