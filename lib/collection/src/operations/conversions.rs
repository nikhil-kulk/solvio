use std::collections::{BTreeMap, HashMap};
use std::num::{NonZeroU32, NonZeroU64};
use std::time::Duration;

use api::conversions::json::{json_path_from_proto, payload_to_proto};
use api::grpc::conversions::{
    convert_shard_key_from_grpc, convert_shard_key_from_grpc_opt, convert_shard_key_to_grpc,
    from_grpc_dist,
};
use api::grpc::solvio::quantization_config_diff::Quantization;
use api::grpc::solvio::update_collection_cluster_setup_request::{
    Operation as ClusterOperationsPb, Operation,
};
use api::rest::schema::ShardKeySelector;
use api::rest::{BaseGroupRequest, MaxOptimizationThreads};
use common::types::ScoreType;
use itertools::Itertools;
use segment::common::operation_error::OperationError;
use segment::data_types::vectors::{
    BatchVectorStructInternal, NamedQuery, VectorInternal, VectorStructInternal,
};
use segment::types::{
    Distance, Filter, HnswConfig, MultiVectorConfig, QuantizationConfig, SearchParams,
    StrictModeConfigOutput, WithPayloadInterface, WithVector,
};
use segment::vector_storage::query::{ContextPair, ContextQuery, DiscoveryQuery, RecoQuery};
use sparse::common::sparse_vector::{SparseVector, validate_sparse_vector_impl};
use tonic::Status;

use super::cluster_ops::ReshardingDirection;
use super::consistency_params::ReadConsistency;
use super::types::{
    CollectionConfig, ContextExamplePair, CoreSearchRequest, Datatype, DiscoverRequestInternal,
    GroupsResult, Modifier, PointGroup, RecommendExample, RecommendGroupsRequestInternal,
    ReshardingInfo, SparseIndexParams, SparseVectorParams, SparseVectorsConfig, VectorParamsDiff,
    VectorsConfigDiff,
};
use crate::config::{
    CollectionParams, ShardingMethod, WalConfig, default_replication_factor,
    default_write_consistency_factor,
};
use crate::lookup::WithLookup;
use crate::lookup::types::WithLookupInterface;
use crate::operations::ClockTag;
use crate::operations::cluster_ops::{
    AbortShardTransfer, AbortTransferOperation, ClusterOperations, CreateShardingKey,
    CreateShardingKeyOperation, DropReplicaOperation, DropShardingKey, DropShardingKeyOperation,
    MoveShard, MoveShardOperation, Replica, ReplicateShard, ReplicateShardOperation,
    RestartTransfer, RestartTransferOperation,
};
use crate::operations::config_diff::{
    CollectionParamsDiff, HnswConfigDiff, OptimizersConfigDiff, QuantizationConfigDiff,
    WalConfigDiff,
};
use crate::operations::point_ops::PointsSelector::PointIdsSelector;
use crate::operations::point_ops::{
    BatchPersisted, FilterSelector, PointIdsList, PointStructPersisted, PointsSelector,
    WriteOrdering,
};
use crate::operations::query_enum::QueryEnum;
use crate::operations::shard_selector_internal::ShardSelectorInternal;
use crate::operations::types::{
    AliasDescription, CollectionClusterInfo, CollectionInfo, CollectionStatus, CountResult,
    LocalShardInfo, OptimizersStatus, RecommendRequestInternal, RecordInternal, RemoteShardInfo,
    ShardTransferInfo, UpdateResult, UpdateStatus, VectorParams, VectorsConfig,
};
use crate::optimizers_builder::OptimizersConfig;
use crate::shards::remote_shard::CollectionCoreSearchRequest;
use crate::shards::replica_set::ReplicaState;
use crate::shards::transfer::ShardTransferMethod;

pub fn sharding_method_to_proto(sharding_method: ShardingMethod) -> i32 {
    match sharding_method {
        ShardingMethod::Auto => api::grpc::solvio::ShardingMethod::Auto as i32,
        ShardingMethod::Custom => api::grpc::solvio::ShardingMethod::Custom as i32,
    }
}

pub fn sharding_method_from_proto(sharding_method: i32) -> Result<ShardingMethod, Status> {
    let sharding_method_grpc = api::grpc::solvio::ShardingMethod::try_from(sharding_method);

    match sharding_method_grpc {
        Ok(api::grpc::solvio::ShardingMethod::Auto) => Ok(ShardingMethod::Auto),
        Ok(api::grpc::solvio::ShardingMethod::Custom) => Ok(ShardingMethod::Custom),
        Err(err) => Err(Status::invalid_argument(format!(
            "Cannot convert ShardingMethod: {sharding_method}, error: {err}"
        ))),
    }
}

pub fn write_ordering_to_proto(ordering: WriteOrdering) -> api::grpc::solvio::WriteOrdering {
    api::grpc::solvio::WriteOrdering {
        r#type: match ordering {
            WriteOrdering::Weak => api::grpc::solvio::WriteOrderingType::Weak as i32,
            WriteOrdering::Medium => api::grpc::solvio::WriteOrderingType::Medium as i32,
            WriteOrdering::Strong => api::grpc::solvio::WriteOrderingType::Strong as i32,
        },
    }
}

pub fn write_ordering_from_proto(
    ordering: Option<api::grpc::solvio::WriteOrdering>,
) -> Result<WriteOrdering, Status> {
    let ordering_parsed = match ordering {
        None => api::grpc::solvio::WriteOrderingType::Weak,
        Some(write_ordering) => {
            match api::grpc::solvio::WriteOrderingType::try_from(write_ordering.r#type) {
                Err(_) => {
                    return Err(Status::invalid_argument(format!(
                        "cannot convert ordering: {}",
                        write_ordering.r#type
                    )));
                }
                Ok(res) => res,
            }
        }
    };

    Ok(match ordering_parsed {
        api::grpc::solvio::WriteOrderingType::Weak => WriteOrdering::Weak,
        api::grpc::solvio::WriteOrderingType::Medium => WriteOrdering::Medium,
        api::grpc::solvio::WriteOrderingType::Strong => WriteOrdering::Strong,
    })
}

pub fn try_record_from_grpc(
    point: api::grpc::solvio::RetrievedPoint,
    with_payload: bool,
) -> Result<RecordInternal, Status> {
    let api::grpc::solvio::RetrievedPoint {
        id,
        payload,
        vectors,
        shard_key,
        order_value,
    } = point;
    let id = id
        .ok_or_else(|| Status::invalid_argument("retrieved point does not have an ID"))?
        .try_into()?;

    let payload = if with_payload {
        Some(api::conversions::json::proto_to_payloads(payload)?)
    } else {
        debug_assert!(payload.is_empty());
        None
    };

    let vector: Option<_> = vectors
        .map(VectorStructInternal::try_from)
        .transpose()
        .map_err(|e| Status::invalid_argument(format!("Cannot convert vectors: {e}")))?;

    let order_value = order_value.map(TryFrom::try_from).transpose()?;

    Ok(RecordInternal {
        id,
        payload,
        vector,
        shard_key: convert_shard_key_from_grpc_opt(shard_key),
        order_value,
    })
}

#[allow(clippy::type_complexity)]
pub fn try_discover_request_from_grpc(
    value: api::grpc::solvio::DiscoverPoints,
) -> Result<
    (
        DiscoverRequestInternal,
        String,
        Option<ReadConsistency>,
        Option<Duration>,
        Option<api::grpc::solvio::ShardKeySelector>,
    ),
    Status,
> {
    let api::grpc::solvio::DiscoverPoints {
        collection_name,
        target,
        context,
        filter,
        limit,
        offset,
        with_payload,
        params,
        using,
        with_vectors,
        lookup_from,
        read_consistency,
        timeout,
        shard_key_selector,
    } = value;

    let target = target.map(TryInto::try_into).transpose()?;

    let context = context
        .into_iter()
        .map(|pair| {
            match (
                pair.positive.map(|p| p.try_into()),
                pair.negative.map(|n| n.try_into()),
            ) {
                (Some(Ok(positive)), Some(Ok(negative))) => {
                    Ok(ContextExamplePair { positive, negative })
                }
                (Some(Err(e)), _) | (_, Some(Err(e))) => Err(e),
                (None, _) | (_, None) => Err(Status::invalid_argument(
                    "Both positive and negative are required in a context pair",
                )),
            }
        })
        .try_collect()?;

    let request = DiscoverRequestInternal {
        target,
        context: Some(context),
        filter: filter.map(|f| f.try_into()).transpose()?,
        params: params.map(|p| p.into()),
        limit: limit as usize,
        offset: offset.map(|x| x as usize),
        with_payload: with_payload.map(|wp| wp.try_into()).transpose()?,
        with_vector: Some(
            with_vectors
                .map(|selector| selector.into())
                .unwrap_or_default(),
        ),
        using: using.map(|u| u.into()),
        lookup_from: lookup_from.map(|l| l.into()),
    };

    let read_consistency = ReadConsistency::try_from_optional(read_consistency)?;

    let timeout = timeout.map(Duration::from_secs);

    Ok((
        request,
        collection_name,
        read_consistency,
        timeout,
        shard_key_selector,
    ))
}

impl From<api::grpc::solvio::HnswConfigDiff> for HnswConfigDiff {
    fn from(value: api::grpc::solvio::HnswConfigDiff) -> Self {
        let api::grpc::solvio::HnswConfigDiff {
            m,
            ef_construct,
            full_scan_threshold,
            max_indexing_threads,
            on_disk,
            payload_m,
        } = value;
        Self {
            m: m.map(|v| v as usize),
            ef_construct: ef_construct.map(|v| v as usize),
            full_scan_threshold: full_scan_threshold.map(|v| v as usize),
            max_indexing_threads: max_indexing_threads.map(|v| v as usize),
            on_disk,
            payload_m: payload_m.map(|v| v as usize),
        }
    }
}

impl From<HnswConfigDiff> for api::grpc::solvio::HnswConfigDiff {
    fn from(value: HnswConfigDiff) -> Self {
        let HnswConfigDiff {
            m,
            ef_construct,
            full_scan_threshold,
            max_indexing_threads,
            on_disk,
            payload_m,
        } = value;
        Self {
            m: m.map(|v| v as u64),
            ef_construct: ef_construct.map(|v| v as u64),
            full_scan_threshold: full_scan_threshold.map(|v| v as u64),
            max_indexing_threads: max_indexing_threads.map(|v| v as u64),
            on_disk,
            payload_m: payload_m.map(|v| v as u64),
        }
    }
}

impl From<api::grpc::solvio::WalConfigDiff> for WalConfigDiff {
    fn from(value: api::grpc::solvio::WalConfigDiff) -> Self {
        let api::grpc::solvio::WalConfigDiff {
            wal_capacity_mb,
            wal_segments_ahead,
        } = value;
        Self {
            wal_capacity_mb: wal_capacity_mb.map(|v| v as usize),
            wal_segments_ahead: wal_segments_ahead.map(|v| v as usize),
        }
    }
}

impl TryFrom<api::grpc::solvio::CollectionParamsDiff> for CollectionParamsDiff {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::CollectionParamsDiff) -> Result<Self, Self::Error> {
        let api::grpc::solvio::CollectionParamsDiff {
            replication_factor,
            write_consistency_factor,
            read_fan_out_factor,
            on_disk_payload,
        } = value;
        Ok(Self {
            replication_factor: replication_factor
                .map(|factor| {
                    NonZeroU32::new(factor)
                        .ok_or_else(|| Status::invalid_argument("`replication_factor` cannot be 0"))
                })
                .transpose()?,
            write_consistency_factor: write_consistency_factor
                .map(|factor| {
                    NonZeroU32::new(factor).ok_or_else(|| {
                        Status::invalid_argument("`write_consistency_factor` cannot be 0")
                    })
                })
                .transpose()?,
            read_fan_out_factor,
            on_disk_payload,
        })
    }
}

impl TryFrom<api::grpc::solvio::OptimizersConfigDiff> for OptimizersConfigDiff {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::OptimizersConfigDiff) -> Result<Self, Self::Error> {
        let api::grpc::solvio::OptimizersConfigDiff {
            deleted_threshold,
            vacuum_min_vector_number,
            default_segment_number,
            max_segment_size,
            memmap_threshold,
            indexing_threshold,
            flush_interval_sec,
            deprecated_max_optimization_threads,
            max_optimization_threads,
        } = value;
        Ok(Self {
            deleted_threshold,
            vacuum_min_vector_number: vacuum_min_vector_number.map(|v| v as usize),
            default_segment_number: default_segment_number.map(|v| v as usize),
            max_segment_size: max_segment_size.map(|v| v as usize),
            memmap_threshold: memmap_threshold.map(|v| v as usize),
            indexing_threshold: indexing_threshold.map(|v| v as usize),
            flush_interval_sec,
            // TODO: remove deprecated field in a later version
            max_optimization_threads: deprecated_max_optimization_threads
                .map(|v| MaxOptimizationThreads::Threads(v as usize))
                .or(max_optimization_threads
                    .map(TryFrom::try_from)
                    .transpose()?),
        })
    }
}

impl TryFrom<api::grpc::solvio::QuantizationConfigDiff> for QuantizationConfigDiff {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::QuantizationConfigDiff) -> Result<Self, Self::Error> {
        let api::grpc::solvio::QuantizationConfigDiff { quantization } = value;
        match quantization {
            None => Err(Status::invalid_argument(
                "Quantization type is not specified",
            )),
            Some(quantization) => match quantization {
                Quantization::Scalar(scalar) => Ok(Self::Scalar(scalar.try_into()?)),
                Quantization::Product(product) => Ok(Self::Product(product.try_into()?)),
                Quantization::Binary(binary) => Ok(Self::Binary(binary.try_into()?)),
                Quantization::Disabled(_) => Ok(Self::new_disabled()),
            },
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

        let CollectionConfig {
            params,
            hnsw_config,
            optimizer_config,
            wal_config,
            quantization_config,
            strict_mode_config,
        } = config;

        let OptimizersConfig {
            deleted_threshold,
            vacuum_min_vector_number,
            default_segment_number,
            max_segment_size,
            memmap_threshold,
            indexing_threshold,
            flush_interval_sec,
            max_optimization_threads,
        } = optimizer_config;

        let HnswConfig {
            m,
            ef_construct,
            full_scan_threshold,
            max_indexing_threads,
            on_disk,
            payload_m,
        } = hnsw_config;

        let CollectionParams {
            vectors,
            shard_number,
            replication_factor,
            on_disk_payload,
            write_consistency_factor,
            read_fan_out_factor,
            sharding_method,
            sparse_vectors,
        } = params;

        api::grpc::solvio::CollectionInfo {
            status: match status {
                CollectionStatus::Green => api::grpc::solvio::CollectionStatus::Green,
                CollectionStatus::Yellow => api::grpc::solvio::CollectionStatus::Yellow,
                CollectionStatus::Red => api::grpc::solvio::CollectionStatus::Red,
                CollectionStatus::Grey => api::grpc::solvio::CollectionStatus::Grey,
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
            vectors_count: vectors_count.map(|count| count as u64),
            indexed_vectors_count: indexed_vectors_count.map(|count| count as u64),
            points_count: points_count.map(|count| count as u64),
            segments_count: segments_count as u64,
            config: Some(api::grpc::solvio::CollectionConfig {
                params: Some(api::grpc::solvio::CollectionParams {
                    vectors_config: {
                        let config = match vectors {
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
                    shard_number: shard_number.get(),
                    replication_factor: Some(replication_factor.get()),
                    on_disk_payload,
                    write_consistency_factor: Some(write_consistency_factor.get()),
                    read_fan_out_factor,
                    sharding_method: sharding_method.map(sharding_method_to_proto),
                    sparse_vectors_config: sparse_vectors.map(|sparse_vectors| {
                        api::grpc::solvio::SparseVectorConfig {
                            map: sparse_vectors
                                .into_iter()
                                .map(|(name, sparse_vector_params)| {
                                    (name, sparse_vector_params.into())
                                })
                                .collect(),
                        }
                    }),
                }),
                hnsw_config: Some(api::grpc::solvio::HnswConfigDiff {
                    m: Some(m as u64),
                    ef_construct: Some(ef_construct as u64),
                    full_scan_threshold: Some(full_scan_threshold as u64),
                    max_indexing_threads: Some(max_indexing_threads as u64),
                    on_disk,
                    payload_m: payload_m.map(|v| v as u64),
                }),
                optimizer_config: Some(api::grpc::solvio::OptimizersConfigDiff {
                    deleted_threshold: Some(deleted_threshold),
                    vacuum_min_vector_number: Some(vacuum_min_vector_number as u64),
                    default_segment_number: Some(default_segment_number as u64),
                    max_segment_size: max_segment_size.map(|x| x as u64),
                    memmap_threshold: memmap_threshold.map(|x| x as u64),
                    indexing_threshold: indexing_threshold.map(|x| x as u64),
                    flush_interval_sec: Some(flush_interval_sec),
                    deprecated_max_optimization_threads: max_optimization_threads.map(|x| x as u64),
                    max_optimization_threads: Some(From::from(max_optimization_threads)),
                }),
                wal_config: wal_config.map(|wal_config| {
                    let WalConfig {
                        wal_capacity_mb,
                        wal_segments_ahead,
                    } = wal_config;

                    api::grpc::solvio::WalConfigDiff {
                        wal_capacity_mb: Some(wal_capacity_mb as u64),
                        wal_segments_ahead: Some(wal_segments_ahead as u64),
                    }
                }),
                quantization_config: quantization_config.map(|x| x.into()),
                strict_mode_config: strict_mode_config
                    .map(api::grpc::solvio::StrictModeConfig::from),
            }),
            payload_schema: payload_schema
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect(),
        }
    }
}

impl From<RecordInternal> for api::grpc::solvio::RetrievedPoint {
    fn from(record: RecordInternal) -> Self {
        let RecordInternal {
            id,
            payload,
            vector,
            shard_key,
            order_value,
        } = record;
        Self {
            id: Some(id.into()),
            payload: payload.map(payload_to_proto).unwrap_or_default(),
            vectors: vector.map(api::grpc::solvio::VectorsOutput::from),
            shard_key: shard_key.map(convert_shard_key_to_grpc),
            order_value: order_value.map(From::from),
        }
    }
}

impl TryFrom<i32> for CollectionStatus {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let status_grpc = api::grpc::solvio::CollectionStatus::try_from(value);
        match status_grpc {
            Ok(api::grpc::solvio::CollectionStatus::Green) => Ok(CollectionStatus::Green),
            Ok(api::grpc::solvio::CollectionStatus::Yellow) => Ok(CollectionStatus::Yellow),
            Ok(api::grpc::solvio::CollectionStatus::Red) => Ok(CollectionStatus::Red),
            Ok(api::grpc::solvio::CollectionStatus::Grey) => Ok(CollectionStatus::Grey),
            Ok(api::grpc::solvio::CollectionStatus::UnknownCollectionStatus) => Err(
                Status::invalid_argument(format!("Unknown CollectionStatus: {value}")),
            ),
            Err(err) => Err(Status::invalid_argument(format!(
                "Cannot convert CollectionStatus: {value}, error: {err}"
            ))),
        }
    }
}

impl TryFrom<api::grpc::solvio::OptimizersConfigDiff> for OptimizersConfig {
    type Error = Status;

    fn try_from(
        optimizer_config: api::grpc::solvio::OptimizersConfigDiff,
    ) -> Result<Self, Self::Error> {
        let api::grpc::solvio::OptimizersConfigDiff {
            deleted_threshold,
            vacuum_min_vector_number,
            default_segment_number,
            max_segment_size,
            memmap_threshold,
            indexing_threshold,
            flush_interval_sec,
            deprecated_max_optimization_threads,
            max_optimization_threads,
        } = optimizer_config;

        let converted_max_optimization_threads: Option<usize> =
            match deprecated_max_optimization_threads {
                None => match max_optimization_threads {
                    None => None,
                    Some(max_optimization_threads) => TryFrom::try_from(max_optimization_threads)?,
                },
                Some(threads) => Some(threads as usize),
            };

        Ok(Self {
            deleted_threshold: deleted_threshold.unwrap_or_default(),
            vacuum_min_vector_number: vacuum_min_vector_number.unwrap_or_default() as usize,
            default_segment_number: default_segment_number.unwrap_or_default() as usize,
            max_segment_size: max_segment_size.map(|x| x as usize),
            memmap_threshold: memmap_threshold.map(|x| x as usize),
            indexing_threshold: indexing_threshold.map(|x| x as usize),
            flush_interval_sec: flush_interval_sec.unwrap_or_default(),
            max_optimization_threads: converted_max_optimization_threads,
        })
    }
}

impl From<api::grpc::solvio::WalConfigDiff> for WalConfig {
    fn from(wal_config: api::grpc::solvio::WalConfigDiff) -> Self {
        let api::grpc::solvio::WalConfigDiff {
            wal_capacity_mb,
            wal_segments_ahead,
        } = wal_config;
        Self {
            wal_capacity_mb: wal_capacity_mb.unwrap_or_default() as usize,
            wal_segments_ahead: wal_segments_ahead.unwrap_or_default() as usize,
        }
    }
}

impl TryFrom<api::grpc::solvio::vectors_config::Config> for VectorsConfig {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::vectors_config::Config) -> Result<Self, Self::Error> {
        Ok(match value {
            api::grpc::solvio::vectors_config::Config::Params(vector_params) => {
                VectorsConfig::Single(vector_params.try_into()?)
            }
            api::grpc::solvio::vectors_config::Config::ParamsMap(vectors_params) => {
                let mut params_map = BTreeMap::new();
                for (name, params) in vectors_params.map {
                    params_map.insert(name, params.try_into()?);
                }
                VectorsConfig::Multi(params_map)
            }
        })
    }
}

impl TryFrom<api::grpc::solvio::vectors_config_diff::Config> for VectorsConfigDiff {
    type Error = Status;

    fn try_from(
        value: api::grpc::solvio::vectors_config_diff::Config,
    ) -> Result<Self, Self::Error> {
        Ok(match value {
            api::grpc::solvio::vectors_config_diff::Config::Params(vector_params) => {
                let diff: VectorParamsDiff = vector_params.try_into()?;
                VectorsConfigDiff::from(diff)
            }
            api::grpc::solvio::vectors_config_diff::Config::ParamsMap(vectors_params) => {
                let mut params_map = BTreeMap::new();
                for (name, params) in vectors_params.map {
                    params_map.insert(name, params.try_into()?);
                }
                VectorsConfigDiff(params_map)
            }
        })
    }
}

impl TryFrom<api::grpc::solvio::VectorParams> for VectorParams {
    type Error = Status;

    fn try_from(vector_params: api::grpc::solvio::VectorParams) -> Result<Self, Self::Error> {
        let api::grpc::solvio::VectorParams {
            size,
            distance,
            hnsw_config,
            quantization_config,
            on_disk,
            datatype,
            multivector_config,
        } = vector_params;
        Ok(Self {
            size: NonZeroU64::new(size).ok_or_else(|| {
                Status::invalid_argument("VectorParams size must be greater than zero")
            })?,
            distance: from_grpc_dist(distance)?,
            hnsw_config: hnsw_config.map(Into::into),
            quantization_config: quantization_config
                .map(grpc_to_segment_quantization_config)
                .transpose()?,
            on_disk,
            datatype: convert_datatype_from_proto(datatype)?,
            multivector_config: multivector_config
                .map(MultiVectorConfig::try_from)
                .transpose()?,
        })
    }
}

fn convert_datatype_from_proto(datatype: Option<i32>) -> Result<Option<Datatype>, Status> {
    if let Some(datatype_int) = datatype {
        let grpc_datatype = api::grpc::solvio::Datatype::try_from(datatype_int);
        if let Ok(grpc_datatype) = grpc_datatype {
            match grpc_datatype {
                api::grpc::solvio::Datatype::Uint8 => Ok(Some(Datatype::Uint8)),
                api::grpc::solvio::Datatype::Float32 => Ok(Some(Datatype::Float32)),
                api::grpc::solvio::Datatype::Float16 => Ok(Some(Datatype::Float16)),
                api::grpc::solvio::Datatype::Default => Ok(None),
            }
        } else {
            Err(Status::invalid_argument(format!(
                "Cannot convert datatype: {datatype_int}"
            )))
        }
    } else {
        Ok(None)
    }
}

impl TryFrom<api::grpc::solvio::VectorParamsDiff> for VectorParamsDiff {
    type Error = Status;

    fn try_from(vector_params: api::grpc::solvio::VectorParamsDiff) -> Result<Self, Self::Error> {
        let api::grpc::solvio::VectorParamsDiff {
            hnsw_config,
            quantization_config,
            on_disk,
        } = vector_params;
        Ok(Self {
            hnsw_config: hnsw_config.map(Into::into),
            quantization_config: quantization_config.map(TryInto::try_into).transpose()?,
            on_disk,
        })
    }
}

impl From<api::grpc::solvio::Modifier> for Modifier {
    fn from(value: api::grpc::solvio::Modifier) -> Self {
        match value {
            api::grpc::solvio::Modifier::None => Modifier::None,
            api::grpc::solvio::Modifier::Idf => Modifier::Idf,
        }
    }
}

impl TryFrom<api::grpc::solvio::SparseVectorParams> for SparseVectorParams {
    type Error = Status;

    fn try_from(
        sparse_vector_params: api::grpc::solvio::SparseVectorParams,
    ) -> Result<Self, Self::Error> {
        let api::grpc::solvio::SparseVectorParams { index, modifier } = sparse_vector_params;
        Ok(Self {
            index: index
                .map(|index_config| -> Result<_, Status> {
                    Ok(SparseIndexParams {
                        full_scan_threshold: index_config.full_scan_threshold.map(|v| v as usize),
                        on_disk: index_config.on_disk,
                        datatype: convert_datatype_from_proto(index_config.datatype)?,
                    })
                })
                .transpose()?,
            modifier: modifier
                .and_then(|x|
                    // XXX: Invalid values silently converted to None
                    api::grpc::solvio::Modifier::try_from(x).ok())
                .map(Modifier::from),
        })
    }
}

impl From<Modifier> for api::grpc::solvio::Modifier {
    fn from(value: Modifier) -> Self {
        match value {
            Modifier::None => api::grpc::solvio::Modifier::None,
            Modifier::Idf => api::grpc::solvio::Modifier::Idf,
        }
    }
}

impl From<SparseVectorParams> for api::grpc::solvio::SparseVectorParams {
    fn from(sparse_vector_params: SparseVectorParams) -> Self {
        let SparseVectorParams { index, modifier } = sparse_vector_params;
        Self {
            index: index.map(|index_config| {
                let SparseIndexParams {
                    full_scan_threshold,
                    on_disk,
                    datatype,
                } = index_config;
                api::grpc::solvio::SparseIndexConfig {
                    full_scan_threshold: full_scan_threshold.map(|v| v as u64),
                    on_disk,
                    datatype: datatype.map(|dt| api::grpc::solvio::Datatype::from(dt).into()),
                }
            }),
            modifier: modifier.map(|modifier| api::grpc::solvio::Modifier::from(modifier) as i32),
        }
    }
}

fn grpc_to_segment_quantization_config(
    value: api::grpc::solvio::QuantizationConfig,
) -> Result<QuantizationConfig, Status> {
    let api::grpc::solvio::QuantizationConfig { quantization } = value;
    let quantization = quantization
        .ok_or_else(|| Status::invalid_argument("QuantizationConfig must contain quantization"))?;
    match quantization {
        api::grpc::solvio::quantization_config::Quantization::Scalar(config) => {
            Ok(QuantizationConfig::Scalar(config.try_into()?))
        }
        api::grpc::solvio::quantization_config::Quantization::Product(config) => {
            Ok(QuantizationConfig::Product(config.try_into()?))
        }
        api::grpc::solvio::quantization_config::Quantization::Binary(config) => {
            Ok(QuantizationConfig::Binary(config.try_into()?))
        }
    }
}

impl TryFrom<api::grpc::solvio::GetCollectionInfoResponse> for CollectionInfo {
    type Error = Status;

    fn try_from(
        collection_info_response: api::grpc::solvio::GetCollectionInfoResponse,
    ) -> Result<Self, Self::Error> {
        let api::grpc::solvio::GetCollectionInfoResponse { result, time: _ } =
            collection_info_response;
        match result {
            None => Err(Status::invalid_argument("Malformed CollectionInfo type")),
            Some(collection_info_response) => {
                let api::grpc::solvio::CollectionInfo {
                    status,
                    optimizer_status,
                    vectors_count,
                    indexed_vectors_count,
                    points_count,
                    segments_count,
                    config,
                    payload_schema,
                } = collection_info_response;
                Ok(Self {
                    status: CollectionStatus::try_from(status)?,
                    optimizer_status: match optimizer_status {
                        None => {
                            return Err(Status::invalid_argument("Malformed OptimizerStatus type"));
                        }
                        Some(api::grpc::solvio::OptimizerStatus { ok, error }) => {
                            if ok {
                                OptimizersStatus::Ok
                            } else {
                                OptimizersStatus::Error(error)
                            }
                        }
                    },
                    vectors_count: vectors_count.map(|count| count as usize),
                    indexed_vectors_count: indexed_vectors_count.map(|count| count as usize),
                    points_count: points_count.map(|count| count as usize),
                    segments_count: segments_count as usize,
                    config: match config {
                        None => {
                            return Err(Status::invalid_argument(
                                "Malformed CollectionConfig type",
                            ));
                        }
                        Some(config) => CollectionConfig::try_from(config)?,
                    },
                    payload_schema: payload_schema
                        .into_iter()
                        .map(|(k, v)| Ok::<_, Status>((json_path_from_proto(&k)?, v.try_into()?)))
                        .try_collect()?,
                })
            }
        }
    }
}

impl TryFrom<PointStructPersisted> for api::grpc::solvio::PointStruct {
    type Error = Status;

    fn try_from(value: PointStructPersisted) -> Result<Self, Self::Error> {
        let PointStructPersisted {
            id,
            vector,
            payload,
        } = value;

        let vectors_internal = VectorStructInternal::try_from(vector)
            .map_err(|e| Status::invalid_argument(format!("Failed to convert vectors: {e}")))?;

        let vectors = api::grpc::solvio::Vectors::from(vectors_internal);
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

impl TryFrom<BatchPersisted> for Vec<api::grpc::solvio::PointStruct> {
    type Error = Status;

    fn try_from(batch: BatchPersisted) -> Result<Self, Self::Error> {
        let BatchPersisted {
            ids,
            vectors,
            payloads,
        } = batch;
        let mut points = Vec::with_capacity(ids.len());
        let batch_vectors = BatchVectorStructInternal::from(vectors);
        let all_vectors = batch_vectors.into_all_vectors(ids.len());
        for (i, p_id) in ids.into_iter().enumerate() {
            let id = Some(p_id.into());
            let vector = all_vectors.get(i).cloned();
            let payload = payloads.as_ref().and_then(|payloads| {
                payloads.get(i).map(|payload| match payload {
                    None => HashMap::new(),
                    Some(payload) => payload_to_proto(payload.clone()),
                })
            });
            let vectors: Option<VectorStructInternal> = vector.map(|v| v.into());

            let point = api::grpc::solvio::PointStruct {
                id,
                vectors: vectors.map(api::grpc::solvio::Vectors::from),
                payload: payload.unwrap_or_default(),
            };
            points.push(point);
        }

        Ok(points)
    }
}

pub fn try_points_selector_from_grpc(
    value: api::grpc::solvio::PointsSelector,
    shard_key_selector: Option<api::grpc::solvio::ShardKeySelector>,
) -> Result<PointsSelector, Status> {
    let api::grpc::solvio::PointsSelector {
        points_selector_one_of,
    } = value;
    match points_selector_one_of {
        Some(api::grpc::solvio::points_selector::PointsSelectorOneOf::Points(points)) => {
            let api::grpc::solvio::PointsIdsList { ids } = points;
            Ok(PointIdsSelector(PointIdsList {
                points: ids
                    .into_iter()
                    .map(|p| p.try_into())
                    .collect::<Result<_, _>>()?,
                shard_key: shard_key_selector.map(ShardKeySelector::from),
            }))
        }
        Some(api::grpc::solvio::points_selector::PointsSelectorOneOf::Filter(f)) => {
            Ok(PointsSelector::FilterSelector(FilterSelector {
                filter: f.try_into()?,
                shard_key: shard_key_selector.map(ShardKeySelector::from),
            }))
        }
        _ => Err(Status::invalid_argument("Malformed PointsSelector type")),
    }
}

impl From<UpdateResult> for api::grpc::solvio::UpdateResultInternal {
    fn from(res: UpdateResult) -> Self {
        let UpdateResult {
            operation_id,
            status,
            clock_tag,
        } = res;
        Self {
            operation_id,
            status: status.into(),
            clock_tag: clock_tag.map(Into::into),
        }
    }
}

impl From<UpdateResult> for api::grpc::solvio::UpdateResult {
    fn from(res: UpdateResult) -> Self {
        api::grpc::solvio::UpdateResultInternal::from(res).into()
    }
}

impl TryFrom<api::grpc::solvio::UpdateResultInternal> for UpdateResult {
    type Error = Status;

    fn try_from(res: api::grpc::solvio::UpdateResultInternal) -> Result<Self, Self::Error> {
        let api::grpc::solvio::UpdateResultInternal {
            operation_id,
            status,
            clock_tag,
        } = res;
        let res = Self {
            operation_id,
            status: status.try_into()?,
            clock_tag: clock_tag.map(ClockTag::from),
        };

        Ok(res)
    }
}

impl TryFrom<api::grpc::solvio::UpdateResult> for UpdateResult {
    type Error = Status;

    fn try_from(res: api::grpc::solvio::UpdateResult) -> Result<Self, Self::Error> {
        api::grpc::solvio::UpdateResultInternal::from(res).try_into()
    }
}

impl From<UpdateStatus> for i32 {
    fn from(status: UpdateStatus) -> Self {
        match status {
            UpdateStatus::Acknowledged => api::grpc::solvio::UpdateStatus::Acknowledged as i32,
            UpdateStatus::Completed => api::grpc::solvio::UpdateStatus::Completed as i32,
            UpdateStatus::ClockRejected => api::grpc::solvio::UpdateStatus::ClockRejected as i32,
        }
    }
}

impl TryFrom<i32> for UpdateStatus {
    type Error = Status;

    fn try_from(status: i32) -> Result<Self, Self::Error> {
        let status = api::grpc::solvio::UpdateStatus::try_from(status)
            .map_err(|_| Status::invalid_argument("Malformed UpdateStatus type"))?;

        let status = match status {
            api::grpc::solvio::UpdateStatus::Acknowledged => Self::Acknowledged,
            api::grpc::solvio::UpdateStatus::Completed => Self::Completed,
            api::grpc::solvio::UpdateStatus::ClockRejected => Self::ClockRejected,

            api::grpc::solvio::UpdateStatus::UnknownUpdateStatus => {
                return Err(Status::invalid_argument(
                    "Malformed UpdateStatus type: update status is unknown",
                ));
            }
        };

        Ok(status)
    }
}

impl From<api::grpc::solvio::CountResult> for CountResult {
    fn from(value: api::grpc::solvio::CountResult) -> Self {
        let api::grpc::solvio::CountResult { count } = value;
        Self {
            count: count as usize,
        }
    }
}

impl From<CountResult> for api::grpc::solvio::CountResult {
    fn from(value: CountResult) -> Self {
        let CountResult { count } = value;
        Self {
            count: count as u64,
        }
    }
}

impl TryFrom<api::grpc::solvio::SearchPoints> for CoreSearchRequest {
    type Error = Status;
    fn try_from(value: api::grpc::solvio::SearchPoints) -> Result<Self, Self::Error> {
        let api::grpc::solvio::SearchPoints {
            collection_name: _,
            vector,
            filter,
            limit,
            with_payload,
            params,
            score_threshold,
            offset,
            vector_name,
            with_vectors,
            read_consistency: _,
            timeout: _,
            shard_key_selector: _,
            sparse_indices,
        } = value;

        if let Some(sparse_indices) = &sparse_indices {
            let api::grpc::solvio::SparseIndices { data } = sparse_indices;
            validate_sparse_vector_impl(data, &vector).map_err(|e| {
                Status::invalid_argument(format!(
                    "Sparse indices does not match sparse vector conditions: {e}"
                ))
            })?;
        }

        let vector_struct =
            api::grpc::conversions::into_named_vector_struct(vector_name, vector, sparse_indices)?;

        Ok(Self {
            query: QueryEnum::Nearest(NamedQuery::from(vector_struct)),
            filter: filter.map(Filter::try_from).transpose()?,
            params: params.map(SearchParams::from),
            limit: limit as usize,
            offset: offset.map(|v| v as usize).unwrap_or_default(),
            with_payload: with_payload
                .map(WithPayloadInterface::try_from)
                .transpose()?,
            with_vector: with_vectors.map(WithVector::from),
            score_threshold: score_threshold.map(|s| s as ScoreType),
        })
    }
}

impl From<QueryEnum> for api::grpc::solvio::QueryEnum {
    fn from(value: QueryEnum) -> Self {
        match value {
            QueryEnum::Nearest(vector) => api::grpc::solvio::QueryEnum {
                query: Some(api::grpc::solvio::query_enum::Query::NearestNeighbors(
                    api::grpc::solvio::Vector::from(vector.query),
                )),
            },
            QueryEnum::RecommendBestScore(named) => api::grpc::solvio::QueryEnum {
                query: Some(api::grpc::solvio::query_enum::Query::RecommendBestScore(
                    named.query.into(),
                )),
            },
            QueryEnum::RecommendSumScores(named) => api::grpc::solvio::QueryEnum {
                query: Some(api::grpc::solvio::query_enum::Query::RecommendSumScores(
                    named.query.into(),
                )),
            },
            QueryEnum::Discover(named) => api::grpc::solvio::QueryEnum {
                query: Some(api::grpc::solvio::query_enum::Query::Discover(
                    api::grpc::solvio::DiscoveryQuery {
                        target: Some(named.query.target.into()),
                        context: named
                            .query
                            .pairs
                            .into_iter()
                            .map(|pair| api::grpc::solvio::ContextPair {
                                positive: { Some(pair.positive.into()) },
                                negative: { Some(pair.negative.into()) },
                            })
                            .collect(),
                    },
                )),
            },
            QueryEnum::Context(named) => api::grpc::solvio::QueryEnum {
                query: Some(api::grpc::solvio::query_enum::Query::Context(
                    api::grpc::solvio::ContextQuery {
                        context: named
                            .query
                            .pairs
                            .into_iter()
                            .map(|pair| api::grpc::solvio::ContextPair {
                                positive: { Some(pair.positive.into()) },
                                negative: { Some(pair.negative.into()) },
                            })
                            .collect(),
                    },
                )),
            },
        }
    }
}

impl<'a> From<CollectionCoreSearchRequest<'a>> for api::grpc::solvio::CoreSearchPoints {
    fn from(value: CollectionCoreSearchRequest<'a>) -> Self {
        let (collection_id, request) = value.0;
        let CoreSearchRequest {
            query,
            filter,
            limit,
            with_payload,
            with_vector,
            params,
            score_threshold,
            offset,
        } = request;
        Self {
            collection_name: collection_id,
            query: Some(query.clone().into()),
            filter: filter.clone().map(|f| f.into()),
            limit: *limit as u64,
            with_vectors: with_vector.clone().map(|wv| wv.into()),
            with_payload: with_payload.clone().map(|wp| wp.into()),
            params: params.map(|sp| sp.into()),
            score_threshold: *score_threshold,
            offset: Some(*offset as u64),
            vector_name: Some(query.get_vector_name().to_owned()),
            read_consistency: None,
        }
    }
}

impl TryFrom<api::grpc::solvio::WithLookup> for WithLookup {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::WithLookup) -> Result<Self, Self::Error> {
        let with_default_payload = || Some(WithPayloadInterface::Bool(true));
        let api::grpc::solvio::WithLookup {
            collection,
            with_payload,
            with_vectors,
        } = value;
        Ok(Self {
            collection_name: collection,
            with_payload: with_payload
                .map(|wp| wp.try_into())
                .transpose()?
                .or_else(with_default_payload),
            with_vectors: with_vectors.map(|wv| wv.into()),
        })
    }
}

impl TryFrom<api::grpc::solvio::WithLookup> for WithLookupInterface {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::WithLookup) -> Result<Self, Self::Error> {
        Ok(Self::WithLookup(value.try_into()?))
    }
}

impl TryFrom<api::grpc::solvio::TargetVector> for RecommendExample {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::TargetVector) -> Result<Self, Self::Error> {
        let api::grpc::solvio::TargetVector { target } = value;
        target
            .ok_or_else(|| Status::invalid_argument("Target vector is malformed"))
            .and_then(|target| match target {
                api::grpc::solvio::target_vector::Target::Single(vector_example) => {
                    Ok(vector_example.try_into()?)
                }
            })
    }
}

fn try_context_pair_from_grpc(
    pair: api::grpc::solvio::ContextPair,
) -> Result<ContextPair<VectorInternal>, Status> {
    let api::grpc::solvio::ContextPair { positive, negative } = pair;
    match (positive, negative) {
        (Some(positive), Some(negative)) => Ok(ContextPair {
            positive: positive.try_into()?,
            negative: negative.try_into()?,
        }),
        _ => Err(Status::invalid_argument(
            "All context pairs must have both positive and negative parts",
        )),
    }
}

impl TryFrom<api::grpc::solvio::CoreSearchPoints> for CoreSearchRequest {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::CoreSearchPoints) -> Result<Self, Self::Error> {
        let query = value
            .query
            .and_then(|query| query.query)
            .map(|query| {
                Ok(match query {
                    api::grpc::solvio::query_enum::Query::NearestNeighbors(vector) => {
                        QueryEnum::Nearest(NamedQuery::from(
                            api::grpc::conversions::into_named_vector_struct(
                                value.vector_name,
                                vector.data,
                                vector.indices,
                            )?,
                        ))
                    }
                    api::grpc::solvio::query_enum::Query::RecommendBestScore(query) => {
                        QueryEnum::RecommendBestScore(NamedQuery {
                            query: RecoQuery::try_from(query)?,
                            using: value.vector_name,
                        })
                    }
                    api::grpc::solvio::query_enum::Query::RecommendSumScores(query) => {
                        QueryEnum::RecommendSumScores(NamedQuery {
                            query: RecoQuery::try_from(query)?,
                            using: value.vector_name,
                        })
                    }
                    api::grpc::solvio::query_enum::Query::Discover(query) => {
                        let Some(target) = query.target else {
                            return Err(Status::invalid_argument("Target is not specified"));
                        };

                        let pairs = query
                            .context
                            .into_iter()
                            .map(try_context_pair_from_grpc)
                            .try_collect()?;

                        QueryEnum::Discover(NamedQuery {
                            query: DiscoveryQuery::new(target.try_into()?, pairs),
                            using: value.vector_name,
                        })
                    }
                    api::grpc::solvio::query_enum::Query::Context(query) => {
                        let pairs = query
                            .context
                            .into_iter()
                            .map(try_context_pair_from_grpc)
                            .try_collect()?;

                        QueryEnum::Context(NamedQuery {
                            query: ContextQuery::new(pairs),
                            using: value.vector_name,
                        })
                    }
                })
            })
            .transpose()?
            .ok_or_else(|| Status::invalid_argument("Query is not specified"))?;

        Ok(Self {
            query,
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

impl TryFrom<PointGroup> for api::grpc::solvio::PointGroup {
    type Error = OperationError;
    fn try_from(group: PointGroup) -> Result<Self, Self::Error> {
        let PointGroup { hits, id, lookup } = group;
        let hits: Result<_, _> = hits
            .into_iter()
            .map(api::grpc::solvio::ScoredPoint::try_from)
            .collect();

        Ok(Self {
            hits: hits?,
            id: Some(id.into()),
            lookup: lookup
                .map(api::grpc::solvio::RetrievedPoint::try_from)
                .transpose()?,
        })
    }
}

impl TryFrom<i32> for ReplicaState {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let replica_state = api::grpc::solvio::ReplicaState::try_from(value)
            .map_err(|_| Status::invalid_argument(format!("Unknown replica state: {value}")))?;
        Ok(replica_state.into())
    }
}

impl From<api::grpc::solvio::ReplicaState> for ReplicaState {
    fn from(value: api::grpc::solvio::ReplicaState) -> Self {
        match value {
            api::grpc::solvio::ReplicaState::Active => Self::Active,
            api::grpc::solvio::ReplicaState::Dead => Self::Dead,
            api::grpc::solvio::ReplicaState::Partial => Self::Partial,
            api::grpc::solvio::ReplicaState::Initializing => Self::Initializing,
            api::grpc::solvio::ReplicaState::Listener => Self::Listener,
            api::grpc::solvio::ReplicaState::PartialSnapshot => Self::PartialSnapshot,
            api::grpc::solvio::ReplicaState::Recovery => Self::Recovery,
            api::grpc::solvio::ReplicaState::Resharding => Self::Resharding,
            api::grpc::solvio::ReplicaState::ReshardingScaleDown => Self::ReshardingScaleDown,
        }
    }
}

impl From<ReplicaState> for api::grpc::solvio::ReplicaState {
    fn from(value: ReplicaState) -> Self {
        match value {
            ReplicaState::Active => Self::Active,
            ReplicaState::Dead => Self::Dead,
            ReplicaState::Partial => Self::Partial,
            ReplicaState::Initializing => Self::Initializing,
            ReplicaState::Listener => Self::Listener,
            ReplicaState::PartialSnapshot => Self::PartialSnapshot,
            ReplicaState::Recovery => Self::Recovery,
            ReplicaState::Resharding => Self::Resharding,
            ReplicaState::ReshardingScaleDown => Self::ReshardingScaleDown,
        }
    }
}

impl TryFrom<api::grpc::solvio::PointId> for RecommendExample {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::PointId) -> Result<Self, Self::Error> {
        Ok(Self::PointId(value.try_into()?))
    }
}

impl TryFrom<api::grpc::solvio::Vector> for RecommendExample {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::Vector) -> Result<Self, Self::Error> {
        let vector: VectorInternal = value.try_into()?;
        match vector {
            VectorInternal::Dense(vector) => Ok(Self::Dense(vector)),
            VectorInternal::Sparse(vector) => Ok(Self::Sparse(vector)),
            VectorInternal::MultiDense(_vector) => Err(Status::invalid_argument(
                "MultiDense vector is not supported in search request",
            )),
        }
    }
}

impl TryFrom<api::grpc::solvio::VectorExample> for RecommendExample {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::VectorExample) -> Result<Self, Self::Error> {
        let api::grpc::solvio::VectorExample { example } = value;
        example
            .ok_or_else(|| {
                Status::invalid_argument(
                    "Vector example, which can be id or bare vector, is malformed",
                )
            })
            .and_then(|example| match example {
                api::grpc::solvio::vector_example::Example::Id(id) => {
                    Ok(Self::PointId(id.try_into()?))
                }
                api::grpc::solvio::vector_example::Example::Vector(vector) => {
                    let api::grpc::solvio::Vector {
                        data,
                        indices,
                        vectors_count: _,
                        vector: _,
                    } = vector;
                    match indices {
                        Some(indices) => {
                            let api::grpc::solvio::SparseIndices { data: indices } = indices;
                            validate_sparse_vector_impl(&indices, &data).map_err(|e| {
                                Status::invalid_argument(format!(
                                    "Sparse indices does not match sparse vector conditions: {e}"
                                ))
                            })?;
                            Ok(Self::Sparse(SparseVector {
                                indices,
                                values: data,
                            }))
                        }
                        None => Ok(Self::Dense(data)),
                    }
                }
            })
    }
}

impl TryFrom<api::grpc::solvio::RecommendPoints> for RecommendRequestInternal {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::RecommendPoints) -> Result<Self, Self::Error> {
        let api::grpc::solvio::RecommendPoints {
            positive,
            negative,
            strategy,
            using,
            lookup_from,
            filter,
            params,
            with_payload,
            with_vectors,
            score_threshold,
            read_consistency: _,
            limit,
            offset,
            collection_name: _,
            positive_vectors,
            negative_vectors,
            timeout: _,
            shard_key_selector: _,
        } = value;
        let positive_ids = positive
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<RecommendExample>, Self::Error>>()?;

        let positive_vectors = positive_vectors
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        let positive = [positive_ids, positive_vectors].concat();

        let negative_ids = negative
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<RecommendExample>, Self::Error>>()?;

        let negative_vectors = negative_vectors
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        let negative = [negative_ids, negative_vectors].concat();

        Ok(RecommendRequestInternal {
            positive,
            negative,
            strategy: strategy.map(|s| s.try_into()).transpose()?,
            filter: filter.map(|f| f.try_into()).transpose()?,
            params: params.map(|p| p.into()),
            limit: limit as usize,
            offset: offset.map(|x| x as usize),
            with_payload: with_payload.map(|wp| wp.try_into()).transpose()?,
            with_vector: Some(
                with_vectors
                    .map(|with_vectors| with_vectors.into())
                    .unwrap_or_default(),
            ),
            score_threshold,
            using: using.map(|name| name.into()),
            lookup_from: lookup_from.map(|x| x.into()),
        })
    }
}

impl TryFrom<api::grpc::solvio::RecommendPointGroups> for RecommendGroupsRequestInternal {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::RecommendPointGroups) -> Result<Self, Self::Error> {
        let recommend_points = api::grpc::solvio::RecommendPoints {
            positive: value.positive,
            negative: value.negative,
            strategy: value.strategy,
            using: value.using,
            lookup_from: value.lookup_from,
            filter: value.filter,
            params: value.params,
            with_payload: value.with_payload,
            with_vectors: value.with_vectors,
            score_threshold: value.score_threshold,
            read_consistency: None,
            limit: 0,     // Will be calculated from group_size
            offset: None, // Not enabled for groups
            collection_name: String::new(),
            positive_vectors: value.positive_vectors,
            negative_vectors: value.negative_vectors,
            timeout: None, // Passed as query param
            shard_key_selector: None,
        };

        let RecommendRequestInternal {
            positive,
            negative,
            strategy,
            using,
            lookup_from,
            filter,
            params,
            with_payload,
            with_vector,
            score_threshold,
            limit: _,
            offset: _,
        } = recommend_points.try_into()?;

        Ok(RecommendGroupsRequestInternal {
            positive,
            negative,
            strategy,
            using,
            lookup_from,
            filter,
            params,
            with_payload,
            with_vector,
            score_threshold,
            group_request: BaseGroupRequest {
                group_by: json_path_from_proto(&value.group_by)?,
                limit: value.limit,
                group_size: value.group_size,
                with_lookup: value.with_lookup.map(|l| l.try_into()).transpose()?,
            },
        })
    }
}

impl TryFrom<GroupsResult> for api::grpc::solvio::GroupsResult {
    type Error = OperationError;

    fn try_from(value: GroupsResult) -> Result<Self, Self::Error> {
        let GroupsResult { groups } = value;
        let groups: Result<_, _> = groups
            .into_iter()
            .map(api::grpc::solvio::PointGroup::try_from)
            .collect();

        Ok(Self { groups: groups? })
    }
}

impl From<VectorParams> for api::grpc::solvio::VectorParams {
    fn from(value: VectorParams) -> Self {
        let VectorParams {
            size,
            distance,
            hnsw_config,
            quantization_config,
            on_disk,
            datatype,
            multivector_config,
        } = value;
        api::grpc::solvio::VectorParams {
            size: size.get(),
            distance: match distance {
                Distance::Cosine => api::grpc::solvio::Distance::Cosine,
                Distance::Euclid => api::grpc::solvio::Distance::Euclid,
                Distance::Dot => api::grpc::solvio::Distance::Dot,
                Distance::Manhattan => api::grpc::solvio::Distance::Manhattan,
            }
            .into(),
            hnsw_config: hnsw_config.map(Into::into),
            quantization_config: quantization_config.map(Into::into),
            on_disk,
            datatype: datatype.map(|dt| api::grpc::solvio::Datatype::from(dt).into()),
            multivector_config: multivector_config.map(api::grpc::solvio::MultiVectorConfig::from),
        }
    }
}

impl From<Datatype> for api::grpc::solvio::Datatype {
    fn from(value: Datatype) -> Self {
        match value {
            Datatype::Float32 => api::grpc::solvio::Datatype::Float32,
            Datatype::Uint8 => api::grpc::solvio::Datatype::Uint8,
            Datatype::Float16 => api::grpc::solvio::Datatype::Float16,
        }
    }
}

impl From<AliasDescription> for api::grpc::solvio::AliasDescription {
    fn from(value: AliasDescription) -> Self {
        let AliasDescription {
            alias_name,
            collection_name,
        } = value;
        api::grpc::solvio::AliasDescription {
            alias_name,
            collection_name,
        }
    }
}

impl From<LocalShardInfo> for api::grpc::solvio::LocalShardInfo {
    fn from(value: LocalShardInfo) -> Self {
        let LocalShardInfo {
            shard_id,
            points_count,
            state,
            shard_key,
        } = value;
        Self {
            shard_id,
            points_count: points_count as u64,
            state: state as i32,
            shard_key: shard_key.map(convert_shard_key_to_grpc),
        }
    }
}

impl From<RemoteShardInfo> for api::grpc::solvio::RemoteShardInfo {
    fn from(value: RemoteShardInfo) -> Self {
        let RemoteShardInfo {
            shard_id,
            peer_id,
            state,
            shard_key,
        } = value;
        Self {
            shard_id,
            peer_id,
            state: state as i32,
            shard_key: shard_key.map(convert_shard_key_to_grpc),
        }
    }
}

impl From<ReshardingInfo> for api::grpc::solvio::ReshardingInfo {
    fn from(value: ReshardingInfo) -> Self {
        let ReshardingInfo {
            uuid: _,
            direction,
            shard_id,
            peer_id,
            shard_key,
        } = value;
        Self {
            shard_id,
            peer_id,
            shard_key: shard_key.map(convert_shard_key_to_grpc),
            direction: api::grpc::solvio::ReshardingDirection::from(direction) as i32,
        }
    }
}

impl From<ReshardingDirection> for api::grpc::solvio::ReshardingDirection {
    fn from(value: ReshardingDirection) -> Self {
        match value {
            ReshardingDirection::Up => api::grpc::solvio::ReshardingDirection::Up,
            ReshardingDirection::Down => api::grpc::solvio::ReshardingDirection::Down,
        }
    }
}

impl From<ShardTransferInfo> for api::grpc::solvio::ShardTransferInfo {
    fn from(value: ShardTransferInfo) -> Self {
        let ShardTransferInfo {
            shard_id,
            to_shard_id,
            from,
            to,
            sync,
            method: _,
            comment: _,
        } = value;
        Self {
            shard_id,
            to_shard_id,
            from,
            to,
            sync,
        }
    }
}

impl From<CollectionClusterInfo> for api::grpc::solvio::CollectionClusterInfoResponse {
    fn from(value: CollectionClusterInfo) -> Self {
        let CollectionClusterInfo {
            peer_id,
            shard_count,
            local_shards,
            remote_shards,
            shard_transfers,
            resharding_operations,
        } = value;
        Self {
            peer_id,
            shard_count: shard_count as u64,
            local_shards: local_shards.into_iter().map(|shard| shard.into()).collect(),
            remote_shards: remote_shards
                .into_iter()
                .map(|shard| shard.into())
                .collect(),
            shard_transfers: shard_transfers
                .into_iter()
                .map(|shard| shard.into())
                .collect(),
            resharding_operations: resharding_operations
                .into_iter()
                .flatten()
                .map(|info| info.into())
                .collect(),
        }
    }
}

impl TryFrom<api::grpc::solvio::ReplicateShard> for ReplicateShard {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::ReplicateShard) -> Result<Self, Self::Error> {
        let api::grpc::solvio::ReplicateShard {
            shard_id,
            to_shard_id,
            from_peer_id,
            to_peer_id,
            method,
        } = value;
        let method = method.map(TryInto::try_into).transpose()?;
        Ok(Self {
            shard_id,
            to_shard_id,
            to_peer_id,
            from_peer_id,
            method,
        })
    }
}

impl TryFrom<api::grpc::solvio::MoveShard> for MoveShard {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::MoveShard) -> Result<Self, Self::Error> {
        let api::grpc::solvio::MoveShard {
            shard_id,
            to_shard_id,
            from_peer_id,
            to_peer_id,
            method,
        } = value;
        let method = method.map(TryInto::try_into).transpose()?;
        Ok(Self {
            shard_id,
            to_shard_id,
            to_peer_id,
            from_peer_id,
            method,
        })
    }
}

impl TryFrom<api::grpc::solvio::AbortShardTransfer> for AbortShardTransfer {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::AbortShardTransfer) -> Result<Self, Self::Error> {
        let api::grpc::solvio::AbortShardTransfer {
            shard_id,
            to_shard_id,
            from_peer_id,
            to_peer_id,
        } = value;
        Ok(Self {
            shard_id,
            to_shard_id,
            to_peer_id,
            from_peer_id,
        })
    }
}

impl TryFrom<i32> for ShardTransferMethod {
    type Error = Status;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        api::grpc::solvio::ShardTransferMethod::try_from(value)
            .map(Into::into)
            .map_err(|_| {
                Status::invalid_argument(format!("Unknown shard transfer method: {value}"))
            })
    }
}

impl From<api::grpc::solvio::ShardTransferMethod> for ShardTransferMethod {
    fn from(value: api::grpc::solvio::ShardTransferMethod) -> Self {
        match value {
            api::grpc::solvio::ShardTransferMethod::StreamRecords => {
                ShardTransferMethod::StreamRecords
            }
            api::grpc::solvio::ShardTransferMethod::Snapshot => ShardTransferMethod::Snapshot,
            api::grpc::solvio::ShardTransferMethod::WalDelta => ShardTransferMethod::WalDelta,
            api::grpc::solvio::ShardTransferMethod::ReshardingStreamRecords => {
                ShardTransferMethod::ReshardingStreamRecords
            }
        }
    }
}

impl TryFrom<api::grpc::solvio::CreateShardKey> for CreateShardingKey {
    type Error = Status;

    fn try_from(op: api::grpc::solvio::CreateShardKey) -> Result<Self, Self::Error> {
        let api::grpc::solvio::CreateShardKey {
            shard_key,
            shards_number,
            replication_factor,
            placement,
        } = op;
        let res = CreateShardingKey {
            shard_key: shard_key
                .and_then(convert_shard_key_from_grpc)
                .ok_or_else(|| Status::invalid_argument("Shard key is not specified"))?,
            shards_number: shards_number
                .map(NonZeroU32::try_from)
                .transpose()
                .map_err(|err| {
                    Status::invalid_argument(format!("Shard number cannot be zero: {err}"))
                })?,
            replication_factor: replication_factor
                .map(NonZeroU32::try_from)
                .transpose()
                .map_err(|err| {
                    Status::invalid_argument(format!("Replication factor cannot be zero: {err}"))
                })?,
            placement: (!placement.is_empty()).then_some(placement),
        };
        Ok(res)
    }
}

impl TryFrom<api::grpc::solvio::DeleteShardKey> for DropShardingKey {
    type Error = Status;

    fn try_from(op: api::grpc::solvio::DeleteShardKey) -> Result<Self, Self::Error> {
        let api::grpc::solvio::DeleteShardKey { shard_key } = op;
        Ok(DropShardingKey {
            shard_key: shard_key
                .and_then(convert_shard_key_from_grpc)
                .ok_or_else(|| Status::invalid_argument("Shard key is not specified"))?,
        })
    }
}

impl TryFrom<ClusterOperationsPb> for ClusterOperations {
    type Error = Status;

    fn try_from(value: ClusterOperationsPb) -> Result<Self, Self::Error> {
        Ok(match value {
            ClusterOperationsPb::MoveShard(op) => {
                ClusterOperations::MoveShard(MoveShardOperation {
                    move_shard: op.try_into()?,
                })
            }
            ClusterOperationsPb::ReplicateShard(op) => {
                ClusterOperations::ReplicateShard(ReplicateShardOperation {
                    replicate_shard: op.try_into()?,
                })
            }
            ClusterOperationsPb::AbortTransfer(op) => {
                ClusterOperations::AbortTransfer(AbortTransferOperation {
                    abort_transfer: op.try_into()?,
                })
            }
            ClusterOperationsPb::DropReplica(op) => {
                let api::grpc::solvio::Replica { shard_id, peer_id } = op;
                ClusterOperations::DropReplica(DropReplicaOperation {
                    drop_replica: Replica { shard_id, peer_id },
                })
            }
            Operation::CreateShardKey(op) => {
                ClusterOperations::CreateShardingKey(CreateShardingKeyOperation {
                    create_sharding_key: op.try_into()?,
                })
            }
            Operation::DeleteShardKey(op) => {
                ClusterOperations::DropShardingKey(DropShardingKeyOperation {
                    drop_sharding_key: op.try_into()?,
                })
            }
            Operation::RestartTransfer(op) => {
                let api::grpc::solvio::RestartTransfer {
                    shard_id,
                    to_shard_id,
                    from_peer_id,
                    to_peer_id,
                    method,
                } = op;
                ClusterOperations::RestartTransfer(RestartTransferOperation {
                    restart_transfer: RestartTransfer {
                        shard_id,
                        to_shard_id,
                        from_peer_id,
                        to_peer_id,
                        method: ShardTransferMethod::try_from(method)?,
                    },
                })
            }
        })
    }
}

impl From<api::grpc::solvio::ShardKeySelector> for ShardSelectorInternal {
    fn from(value: api::grpc::solvio::ShardKeySelector) -> Self {
        let api::grpc::solvio::ShardKeySelector { shard_keys } = value;
        let shard_keys: Vec<_> = shard_keys
            .into_iter()
            .filter_map(convert_shard_key_from_grpc)
            .collect();

        if shard_keys.len() == 1 {
            ShardSelectorInternal::ShardKey(shard_keys.into_iter().next().unwrap())
        } else {
            ShardSelectorInternal::ShardKeys(shard_keys)
        }
    }
}

impl TryFrom<api::grpc::solvio::SparseVectorConfig> for SparseVectorsConfig {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::SparseVectorConfig) -> Result<Self, Self::Error> {
        let api::grpc::solvio::SparseVectorConfig { map } = value;
        map.into_iter()
            .map(|(k, v)| Ok((k, v.try_into()?)))
            .collect::<Result<_, Status>>()
            .map(SparseVectorsConfig)
    }
}

impl TryFrom<api::grpc::solvio::CollectionConfig> for CollectionConfig {
    type Error = Status;

    fn try_from(config: api::grpc::solvio::CollectionConfig) -> Result<Self, Self::Error> {
        let api::grpc::solvio::CollectionConfig {
            params,
            hnsw_config,
            optimizer_config,
            wal_config,
            quantization_config,
            strict_mode_config,
        } = config;
        Ok(Self {
            params: match params {
                None => return Err(Status::invalid_argument("Malformed CollectionParams type")),
                Some(params) => {
                    let api::grpc::solvio::CollectionParams {
                        shard_number,
                        on_disk_payload,
                        vectors_config,
                        replication_factor,
                        write_consistency_factor,
                        read_fan_out_factor,
                        sharding_method,
                        sparse_vectors_config,
                    } = params;
                    CollectionParams {
                        vectors: match vectors_config {
                            None => {
                                return Err(Status::invalid_argument(
                                    "Expected `vectors` - configuration for vector storage",
                                ));
                            }
                            Some(vector_config) => match vector_config.config {
                                None => {
                                    return Err(Status::invalid_argument(
                                        "Expected `vectors` - configuration for vector storage",
                                    ));
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
                                        .collect::<Result<BTreeMap<_, _>, Status>>()?,
                                ),
                            },
                        },
                        sparse_vectors: sparse_vectors_config
                            .map(|v| {
                                SparseVectorsConfig::try_from(v).map(|SparseVectorsConfig(x)| x)
                            })
                            .transpose()?,
                        shard_number: NonZeroU32::new(shard_number).ok_or_else(|| {
                            Status::invalid_argument("`shard_number` cannot be zero")
                        })?,
                        on_disk_payload,
                        replication_factor: NonZeroU32::new(
                            replication_factor
                                .unwrap_or_else(|| default_replication_factor().get()),
                        )
                        .ok_or_else(|| {
                            Status::invalid_argument("`replication_factor` cannot be zero")
                        })?,
                        write_consistency_factor: NonZeroU32::new(
                            write_consistency_factor
                                .unwrap_or_else(|| default_write_consistency_factor().get()),
                        )
                        .ok_or_else(|| {
                            Status::invalid_argument("`write_consistency_factor` cannot be zero")
                        })?,

                        read_fan_out_factor,
                        sharding_method: sharding_method
                            .map(sharding_method_from_proto)
                            .transpose()?,
                    }
                }
            },
            hnsw_config: match hnsw_config {
                None => return Err(Status::invalid_argument("Malformed HnswConfig type")),
                Some(hnsw_config) => HnswConfig::from(hnsw_config),
            },
            optimizer_config: match optimizer_config {
                None => return Err(Status::invalid_argument("Malformed OptimizerConfig type")),
                Some(optimizer_config) => OptimizersConfig::try_from(optimizer_config)?,
            },
            wal_config: match wal_config {
                None => return Err(Status::invalid_argument("Malformed WalConfig type")),
                Some(wal_config) => Some(WalConfig::from(wal_config)),
            },
            quantization_config: {
                if let Some(config) = quantization_config {
                    Some(QuantizationConfig::try_from(config)?)
                } else {
                    None
                }
            },
            strict_mode_config: strict_mode_config.map(StrictModeConfigOutput::from),
        })
    }
}
