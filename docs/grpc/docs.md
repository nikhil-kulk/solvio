# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [collections.proto](#collections-proto)
    - [AliasDescription](#solvio-AliasDescription)
    - [AliasOperations](#solvio-AliasOperations)
    - [BinaryQuantization](#solvio-BinaryQuantization)
    - [ChangeAliases](#solvio-ChangeAliases)
    - [CollectionClusterInfoRequest](#solvio-CollectionClusterInfoRequest)
    - [CollectionClusterInfoResponse](#solvio-CollectionClusterInfoResponse)
    - [CollectionConfig](#solvio-CollectionConfig)
    - [CollectionDescription](#solvio-CollectionDescription)
    - [CollectionInfo](#solvio-CollectionInfo)
    - [CollectionInfo.PayloadSchemaEntry](#solvio-CollectionInfo-PayloadSchemaEntry)
    - [CollectionOperationResponse](#solvio-CollectionOperationResponse)
    - [CollectionParams](#solvio-CollectionParams)
    - [CollectionParamsDiff](#solvio-CollectionParamsDiff)
    - [CreateAlias](#solvio-CreateAlias)
    - [CreateCollection](#solvio-CreateCollection)
    - [DeleteAlias](#solvio-DeleteAlias)
    - [DeleteCollection](#solvio-DeleteCollection)
    - [Disabled](#solvio-Disabled)
    - [GetCollectionInfoRequest](#solvio-GetCollectionInfoRequest)
    - [GetCollectionInfoResponse](#solvio-GetCollectionInfoResponse)
    - [HnswConfigDiff](#solvio-HnswConfigDiff)
    - [ListAliasesRequest](#solvio-ListAliasesRequest)
    - [ListAliasesResponse](#solvio-ListAliasesResponse)
    - [ListCollectionAliasesRequest](#solvio-ListCollectionAliasesRequest)
    - [ListCollectionsRequest](#solvio-ListCollectionsRequest)
    - [ListCollectionsResponse](#solvio-ListCollectionsResponse)
    - [LocalShardInfo](#solvio-LocalShardInfo)
    - [MoveShard](#solvio-MoveShard)
    - [OptimizerStatus](#solvio-OptimizerStatus)
    - [OptimizersConfigDiff](#solvio-OptimizersConfigDiff)
    - [PayloadIndexParams](#solvio-PayloadIndexParams)
    - [PayloadSchemaInfo](#solvio-PayloadSchemaInfo)
    - [ProductQuantization](#solvio-ProductQuantization)
    - [QuantizationConfig](#solvio-QuantizationConfig)
    - [QuantizationConfigDiff](#solvio-QuantizationConfigDiff)
    - [RemoteShardInfo](#solvio-RemoteShardInfo)
    - [RenameAlias](#solvio-RenameAlias)
    - [Replica](#solvio-Replica)
    - [ScalarQuantization](#solvio-ScalarQuantization)
    - [ShardTransferInfo](#solvio-ShardTransferInfo)
    - [TextIndexParams](#solvio-TextIndexParams)
    - [UpdateCollection](#solvio-UpdateCollection)
    - [UpdateCollectionClusterSetupRequest](#solvio-UpdateCollectionClusterSetupRequest)
    - [UpdateCollectionClusterSetupResponse](#solvio-UpdateCollectionClusterSetupResponse)
    - [VectorParams](#solvio-VectorParams)
    - [VectorParamsDiff](#solvio-VectorParamsDiff)
    - [VectorParamsDiffMap](#solvio-VectorParamsDiffMap)
    - [VectorParamsDiffMap.MapEntry](#solvio-VectorParamsDiffMap-MapEntry)
    - [VectorParamsMap](#solvio-VectorParamsMap)
    - [VectorParamsMap.MapEntry](#solvio-VectorParamsMap-MapEntry)
    - [VectorsConfig](#solvio-VectorsConfig)
    - [VectorsConfigDiff](#solvio-VectorsConfigDiff)
    - [WalConfigDiff](#solvio-WalConfigDiff)
  
    - [CollectionStatus](#solvio-CollectionStatus)
    - [CompressionRatio](#solvio-CompressionRatio)
    - [Distance](#solvio-Distance)
    - [PayloadSchemaType](#solvio-PayloadSchemaType)
    - [QuantizationType](#solvio-QuantizationType)
    - [ReplicaState](#solvio-ReplicaState)
    - [TokenizerType](#solvio-TokenizerType)
  
- [collections_service.proto](#collections_service-proto)
    - [Collections](#solvio-Collections)
  
- [health_check.proto](#health_check-proto)
    - [HealthCheckRequest](#grpc-health-v1-HealthCheckRequest)
    - [HealthCheckResponse](#grpc-health-v1-HealthCheckResponse)
  
    - [HealthCheckResponse.ServingStatus](#grpc-health-v1-HealthCheckResponse-ServingStatus)
  
    - [Health](#grpc-health-v1-Health)
  
- [json_with_int.proto](#json_with_int-proto)
    - [ListValue](#solvio-ListValue)
    - [Struct](#solvio-Struct)
    - [Struct.FieldsEntry](#solvio-Struct-FieldsEntry)
    - [Value](#solvio-Value)
  
    - [NullValue](#solvio-NullValue)
  
- [points.proto](#points-proto)
    - [BatchResult](#solvio-BatchResult)
    - [ClearPayloadPoints](#solvio-ClearPayloadPoints)
    - [Condition](#solvio-Condition)
    - [CountPoints](#solvio-CountPoints)
    - [CountResponse](#solvio-CountResponse)
    - [CountResult](#solvio-CountResult)
    - [CreateFieldIndexCollection](#solvio-CreateFieldIndexCollection)
    - [DeleteFieldIndexCollection](#solvio-DeleteFieldIndexCollection)
    - [DeletePayloadPoints](#solvio-DeletePayloadPoints)
    - [DeletePointVectors](#solvio-DeletePointVectors)
    - [DeletePoints](#solvio-DeletePoints)
    - [FieldCondition](#solvio-FieldCondition)
    - [Filter](#solvio-Filter)
    - [GeoBoundingBox](#solvio-GeoBoundingBox)
    - [GeoLineString](#solvio-GeoLineString)
    - [GeoPoint](#solvio-GeoPoint)
    - [GeoPolygon](#solvio-GeoPolygon)
    - [GeoRadius](#solvio-GeoRadius)
    - [GetPoints](#solvio-GetPoints)
    - [GetResponse](#solvio-GetResponse)
    - [GroupId](#solvio-GroupId)
    - [GroupsResult](#solvio-GroupsResult)
    - [HasIdCondition](#solvio-HasIdCondition)
    - [IsEmptyCondition](#solvio-IsEmptyCondition)
    - [IsNullCondition](#solvio-IsNullCondition)
    - [LookupLocation](#solvio-LookupLocation)
    - [Match](#solvio-Match)
    - [NamedVectors](#solvio-NamedVectors)
    - [NamedVectors.VectorsEntry](#solvio-NamedVectors-VectorsEntry)
    - [NestedCondition](#solvio-NestedCondition)
    - [PayloadExcludeSelector](#solvio-PayloadExcludeSelector)
    - [PayloadIncludeSelector](#solvio-PayloadIncludeSelector)
    - [PointGroup](#solvio-PointGroup)
    - [PointId](#solvio-PointId)
    - [PointStruct](#solvio-PointStruct)
    - [PointStruct.PayloadEntry](#solvio-PointStruct-PayloadEntry)
    - [PointVectors](#solvio-PointVectors)
    - [PointsIdsList](#solvio-PointsIdsList)
    - [PointsOperationResponse](#solvio-PointsOperationResponse)
    - [PointsSelector](#solvio-PointsSelector)
    - [PointsUpdateOperation](#solvio-PointsUpdateOperation)
    - [PointsUpdateOperation.DeletePayload](#solvio-PointsUpdateOperation-DeletePayload)
    - [PointsUpdateOperation.DeleteVectors](#solvio-PointsUpdateOperation-DeleteVectors)
    - [PointsUpdateOperation.PointStructList](#solvio-PointsUpdateOperation-PointStructList)
    - [PointsUpdateOperation.SetPayload](#solvio-PointsUpdateOperation-SetPayload)
    - [PointsUpdateOperation.SetPayload.PayloadEntry](#solvio-PointsUpdateOperation-SetPayload-PayloadEntry)
    - [PointsUpdateOperation.UpdateVectors](#solvio-PointsUpdateOperation-UpdateVectors)
    - [QuantizationSearchParams](#solvio-QuantizationSearchParams)
    - [Range](#solvio-Range)
    - [ReadConsistency](#solvio-ReadConsistency)
    - [RecommendBatchPoints](#solvio-RecommendBatchPoints)
    - [RecommendBatchResponse](#solvio-RecommendBatchResponse)
    - [RecommendGroupsResponse](#solvio-RecommendGroupsResponse)
    - [RecommendPointGroups](#solvio-RecommendPointGroups)
    - [RecommendPoints](#solvio-RecommendPoints)
    - [RecommendResponse](#solvio-RecommendResponse)
    - [RepeatedIntegers](#solvio-RepeatedIntegers)
    - [RepeatedStrings](#solvio-RepeatedStrings)
    - [RetrievedPoint](#solvio-RetrievedPoint)
    - [RetrievedPoint.PayloadEntry](#solvio-RetrievedPoint-PayloadEntry)
    - [ScoredPoint](#solvio-ScoredPoint)
    - [ScoredPoint.PayloadEntry](#solvio-ScoredPoint-PayloadEntry)
    - [ScrollPoints](#solvio-ScrollPoints)
    - [ScrollResponse](#solvio-ScrollResponse)
    - [SearchBatchPoints](#solvio-SearchBatchPoints)
    - [SearchBatchResponse](#solvio-SearchBatchResponse)
    - [SearchGroupsResponse](#solvio-SearchGroupsResponse)
    - [SearchParams](#solvio-SearchParams)
    - [SearchPointGroups](#solvio-SearchPointGroups)
    - [SearchPoints](#solvio-SearchPoints)
    - [SearchResponse](#solvio-SearchResponse)
    - [SetPayloadPoints](#solvio-SetPayloadPoints)
    - [SetPayloadPoints.PayloadEntry](#solvio-SetPayloadPoints-PayloadEntry)
    - [UpdateBatchPoints](#solvio-UpdateBatchPoints)
    - [UpdateBatchResponse](#solvio-UpdateBatchResponse)
    - [UpdatePointVectors](#solvio-UpdatePointVectors)
    - [UpdateResult](#solvio-UpdateResult)
    - [UpsertPoints](#solvio-UpsertPoints)
    - [ValuesCount](#solvio-ValuesCount)
    - [Vector](#solvio-Vector)
    - [Vectors](#solvio-Vectors)
    - [VectorsSelector](#solvio-VectorsSelector)
    - [WithLookup](#solvio-WithLookup)
    - [WithPayloadSelector](#solvio-WithPayloadSelector)
    - [WithVectorsSelector](#solvio-WithVectorsSelector)
    - [WriteOrdering](#solvio-WriteOrdering)
  
    - [FieldType](#solvio-FieldType)
    - [ReadConsistencyType](#solvio-ReadConsistencyType)
    - [RecommendStrategy](#solvio-RecommendStrategy)
    - [UpdateStatus](#solvio-UpdateStatus)
    - [WriteOrderingType](#solvio-WriteOrderingType)
  
- [points_service.proto](#points_service-proto)
    - [Points](#solvio-Points)
  
- [solvio.proto](#solvio-proto)
    - [HealthCheckReply](#solvio-HealthCheckReply)
    - [HealthCheckRequest](#solvio-HealthCheckRequest)
  
    - [Solvio](#solvio-Solvio)
  
- [solvio_internal_service.proto](#solvio_internal_service-proto)
    - [HttpPortRequest](#solvio-HttpPortRequest)
    - [HttpPortResponse](#solvio-HttpPortResponse)
    - [WaitOnConsensusCommitRequest](#solvio-WaitOnConsensusCommitRequest)
    - [WaitOnConsensusCommitResponse](#solvio-WaitOnConsensusCommitResponse)
  
    - [SolvioInternal](#solvio-SolvioInternal)
  
- [snapshots_service.proto](#snapshots_service-proto)
    - [CreateFullSnapshotRequest](#solvio-CreateFullSnapshotRequest)
    - [CreateSnapshotRequest](#solvio-CreateSnapshotRequest)
    - [CreateSnapshotResponse](#solvio-CreateSnapshotResponse)
    - [DeleteFullSnapshotRequest](#solvio-DeleteFullSnapshotRequest)
    - [DeleteSnapshotRequest](#solvio-DeleteSnapshotRequest)
    - [DeleteSnapshotResponse](#solvio-DeleteSnapshotResponse)
    - [ListFullSnapshotsRequest](#solvio-ListFullSnapshotsRequest)
    - [ListSnapshotsRequest](#solvio-ListSnapshotsRequest)
    - [ListSnapshotsResponse](#solvio-ListSnapshotsResponse)
    - [SnapshotDescription](#solvio-SnapshotDescription)
  
    - [Snapshots](#solvio-Snapshots)
  
- [Scalar Value Types](#scalar-value-types)



<a name="collections-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## collections.proto



<a name="solvio-AliasDescription"></a>

### AliasDescription



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| alias_name | [string](#string) |  | Name of the alias |
| collection_name | [string](#string) |  | Name of the collection |






<a name="solvio-AliasOperations"></a>

### AliasOperations



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| create_alias | [CreateAlias](#solvio-CreateAlias) |  |  |
| rename_alias | [RenameAlias](#solvio-RenameAlias) |  |  |
| delete_alias | [DeleteAlias](#solvio-DeleteAlias) |  |  |






<a name="solvio-BinaryQuantization"></a>

### BinaryQuantization



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| always_ram | [bool](#bool) | optional | If true - quantized vectors always will be stored in RAM, ignoring the config of main storage |






<a name="solvio-ChangeAliases"></a>

### ChangeAliases



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| actions | [AliasOperations](#solvio-AliasOperations) | repeated | List of actions |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="solvio-CollectionClusterInfoRequest"></a>

### CollectionClusterInfoRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="solvio-CollectionClusterInfoResponse"></a>

### CollectionClusterInfoResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| peer_id | [uint64](#uint64) |  | ID of this peer |
| shard_count | [uint64](#uint64) |  | Total number of shards |
| local_shards | [LocalShardInfo](#solvio-LocalShardInfo) | repeated | Local shards |
| remote_shards | [RemoteShardInfo](#solvio-RemoteShardInfo) | repeated | Remote shards |
| shard_transfers | [ShardTransferInfo](#solvio-ShardTransferInfo) | repeated | Shard transfers |






<a name="solvio-CollectionConfig"></a>

### CollectionConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [CollectionParams](#solvio-CollectionParams) |  | Collection parameters |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) |  | Configuration of vector index |
| optimizer_config | [OptimizersConfigDiff](#solvio-OptimizersConfigDiff) |  | Configuration of the optimizers |
| wal_config | [WalConfigDiff](#solvio-WalConfigDiff) |  | Configuration of the Write-Ahead-Log |
| quantization_config | [QuantizationConfig](#solvio-QuantizationConfig) | optional | Configuration of the vector quantization |






<a name="solvio-CollectionDescription"></a>

### CollectionDescription



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name of the collection |






<a name="solvio-CollectionInfo"></a>

### CollectionInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [CollectionStatus](#solvio-CollectionStatus) |  | operating condition of the collection |
| optimizer_status | [OptimizerStatus](#solvio-OptimizerStatus) |  | status of collection optimizers |
| vectors_count | [uint64](#uint64) |  | number of vectors in the collection |
| segments_count | [uint64](#uint64) |  | Number of independent segments |
| config | [CollectionConfig](#solvio-CollectionConfig) |  | Configuration |
| payload_schema | [CollectionInfo.PayloadSchemaEntry](#solvio-CollectionInfo-PayloadSchemaEntry) | repeated | Collection data types |
| points_count | [uint64](#uint64) |  | number of points in the collection |
| indexed_vectors_count | [uint64](#uint64) | optional | number of indexed vectors in the collection. |






<a name="solvio-CollectionInfo-PayloadSchemaEntry"></a>

### CollectionInfo.PayloadSchemaEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [PayloadSchemaInfo](#solvio-PayloadSchemaInfo) |  |  |






<a name="solvio-CollectionOperationResponse"></a>

### CollectionOperationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [bool](#bool) |  | if operation made changes |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-CollectionParams"></a>

### CollectionParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_number | [uint32](#uint32) |  | Number of shards in collection |
| on_disk_payload | [bool](#bool) |  | If true - point&#39;s payload will not be stored in memory |
| vectors_config | [VectorsConfig](#solvio-VectorsConfig) | optional | Configuration for vectors |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard that network tries to maintain |
| write_consistency_factor | [uint32](#uint32) | optional | How many replicas should apply the operation for us to consider it successful |
| read_fan_out_factor | [uint32](#uint32) | optional | Fan-out every read request to these many additional remote nodes (and return first available response) |






<a name="solvio-CollectionParamsDiff"></a>

### CollectionParamsDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard that network tries to maintain |
| write_consistency_factor | [uint32](#uint32) | optional | How many replicas should apply the operation for us to consider it successful |
| on_disk_payload | [bool](#bool) | optional | If true - point&#39;s payload will not be stored in memory |
| read_fan_out_factor | [uint32](#uint32) | optional | Fan-out every read request to these many additional remote nodes (and return first available response) |






<a name="solvio-CreateAlias"></a>

### CreateAlias



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| alias_name | [string](#string) |  | New name of the alias |






<a name="solvio-CreateCollection"></a>

### CreateCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) | optional | Configuration of vector index |
| wal_config | [WalConfigDiff](#solvio-WalConfigDiff) | optional | Configuration of the Write-Ahead-Log |
| optimizers_config | [OptimizersConfigDiff](#solvio-OptimizersConfigDiff) | optional | Configuration of the optimizers |
| shard_number | [uint32](#uint32) | optional | Number of shards in the collection, default is 1 for standalone, otherwise equal to the number of nodes. Minimum is 1 |
| on_disk_payload | [bool](#bool) | optional | If true - point&#39;s payload will not be stored in memory |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |
| vectors_config | [VectorsConfig](#solvio-VectorsConfig) | optional | Configuration for vectors |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard that network tries to maintain, default = 1 |
| write_consistency_factor | [uint32](#uint32) | optional | How many replicas should apply the operation for us to consider it successful, default = 1 |
| init_from_collection | [string](#string) | optional | Specify name of the other collection to copy data from |
| quantization_config | [QuantizationConfig](#solvio-QuantizationConfig) | optional | Quantization configuration of vector |






<a name="solvio-DeleteAlias"></a>

### DeleteAlias



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| alias_name | [string](#string) |  | Name of the alias |






<a name="solvio-DeleteCollection"></a>

### DeleteCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="solvio-Disabled"></a>

### Disabled







<a name="solvio-GetCollectionInfoRequest"></a>

### GetCollectionInfoRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="solvio-GetCollectionInfoResponse"></a>

### GetCollectionInfoResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [CollectionInfo](#solvio-CollectionInfo) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-HnswConfigDiff"></a>

### HnswConfigDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| m | [uint64](#uint64) | optional | Number of edges per node in the index graph. Larger the value - more accurate the search, more space required. |
| ef_construct | [uint64](#uint64) | optional | Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build the index. |
| full_scan_threshold | [uint64](#uint64) | optional | Minimal size (in KiloBytes) of vectors for additional payload-based indexing. If the payload chunk is smaller than `full_scan_threshold` additional indexing won&#39;t be used - in this case full-scan search should be preferred by query planner and additional indexing is not required. Note: 1 Kb = 1 vector of size 256 |
| max_indexing_threads | [uint64](#uint64) | optional | Number of parallel threads used for background index building. If 0 - auto selection. |
| on_disk | [bool](#bool) | optional | Store HNSW index on disk. If set to false, the index will be stored in RAM. |
| payload_m | [uint64](#uint64) | optional | Number of additional payload-aware links per node in the index graph. If not set - regular M parameter will be used. |






<a name="solvio-ListAliasesRequest"></a>

### ListAliasesRequest







<a name="solvio-ListAliasesResponse"></a>

### ListAliasesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| aliases | [AliasDescription](#solvio-AliasDescription) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-ListCollectionAliasesRequest"></a>

### ListCollectionAliasesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="solvio-ListCollectionsRequest"></a>

### ListCollectionsRequest







<a name="solvio-ListCollectionsResponse"></a>

### ListCollectionsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collections | [CollectionDescription](#solvio-CollectionDescription) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-LocalShardInfo"></a>

### LocalShardInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| points_count | [uint64](#uint64) |  | Number of points in the shard |
| state | [ReplicaState](#solvio-ReplicaState) |  | Is replica active |






<a name="solvio-MoveShard"></a>

### MoveShard



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| from_peer_id | [uint64](#uint64) |  |  |
| to_peer_id | [uint64](#uint64) |  |  |






<a name="solvio-OptimizerStatus"></a>

### OptimizerStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ok | [bool](#bool) |  |  |
| error | [string](#string) |  |  |






<a name="solvio-OptimizersConfigDiff"></a>

### OptimizersConfigDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| deleted_threshold | [double](#double) | optional | The minimal fraction of deleted vectors in a segment, required to perform segment optimization |
| vacuum_min_vector_number | [uint64](#uint64) | optional | The minimal number of vectors in a segment, required to perform segment optimization |
| default_segment_number | [uint64](#uint64) | optional | Target amount of segments the optimizer will try to keep. Real amount of segments may vary depending on multiple parameters:

- Amount of stored points. - Current write RPS.

It is recommended to select the default number of segments as a factor of the number of search threads, so that each segment would be handled evenly by one of the threads. |
| max_segment_size | [uint64](#uint64) | optional | Do not create segments larger this size (in kilobytes). Large segments might require disproportionately long indexation times, therefore it makes sense to limit the size of segments.

If indexing speed is more important - make this parameter lower. If search speed is more important - make this parameter higher. Note: 1Kb = 1 vector of size 256 If not set, will be automatically selected considering the number of available CPUs. |
| memmap_threshold | [uint64](#uint64) | optional | Maximum size (in kilobytes) of vectors to store in-memory per segment. Segments larger than this threshold will be stored as read-only memmaped file.

Memmap storage is disabled by default, to enable it, set this threshold to a reasonable value.

To disable memmap storage, set this to `0`.

Note: 1Kb = 1 vector of size 256 |
| indexing_threshold | [uint64](#uint64) | optional | Maximum size (in kilobytes) of vectors allowed for plain index, exceeding this threshold will enable vector indexing

Default value is 20,000, based on &lt;https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md&gt;.

To disable vector indexing, set to `0`.

Note: 1kB = 1 vector of size 256. |
| flush_interval_sec | [uint64](#uint64) | optional | Interval between forced flushes. |
| max_optimization_threads | [uint64](#uint64) | optional | Max number of threads, which can be used for optimization. If 0 - `NUM_CPU - 1` will be used |






<a name="solvio-PayloadIndexParams"></a>

### PayloadIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text_index_params | [TextIndexParams](#solvio-TextIndexParams) |  | Parameters for text index |






<a name="solvio-PayloadSchemaInfo"></a>

### PayloadSchemaInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data_type | [PayloadSchemaType](#solvio-PayloadSchemaType) |  | Field data type |
| params | [PayloadIndexParams](#solvio-PayloadIndexParams) | optional | Field index parameters |
| points | [uint64](#uint64) | optional | Number of points indexed within this field indexed |






<a name="solvio-ProductQuantization"></a>

### ProductQuantization



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| compression | [CompressionRatio](#solvio-CompressionRatio) |  | Compression ratio |
| always_ram | [bool](#bool) | optional | If true - quantized vectors always will be stored in RAM, ignoring the config of main storage |






<a name="solvio-QuantizationConfig"></a>

### QuantizationConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| scalar | [ScalarQuantization](#solvio-ScalarQuantization) |  |  |
| product | [ProductQuantization](#solvio-ProductQuantization) |  |  |
| binary | [BinaryQuantization](#solvio-BinaryQuantization) |  |  |






<a name="solvio-QuantizationConfigDiff"></a>

### QuantizationConfigDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| scalar | [ScalarQuantization](#solvio-ScalarQuantization) |  |  |
| product | [ProductQuantization](#solvio-ProductQuantization) |  |  |
| disabled | [Disabled](#solvio-Disabled) |  |  |
| binary | [BinaryQuantization](#solvio-BinaryQuantization) |  |  |






<a name="solvio-RemoteShardInfo"></a>

### RemoteShardInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| peer_id | [uint64](#uint64) |  | Remote peer id |
| state | [ReplicaState](#solvio-ReplicaState) |  | Is replica active |






<a name="solvio-RenameAlias"></a>

### RenameAlias



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| old_alias_name | [string](#string) |  | Name of the alias to rename |
| new_alias_name | [string](#string) |  | Name of the alias |






<a name="solvio-Replica"></a>

### Replica



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  |  |
| peer_id | [uint64](#uint64) |  |  |






<a name="solvio-ScalarQuantization"></a>

### ScalarQuantization



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [QuantizationType](#solvio-QuantizationType) |  | Type of quantization |
| quantile | [float](#float) | optional | Number of bits to use for quantization |
| always_ram | [bool](#bool) | optional | If true - quantized vectors always will be stored in RAM, ignoring the config of main storage |






<a name="solvio-ShardTransferInfo"></a>

### ShardTransferInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| from | [uint64](#uint64) |  |  |
| to | [uint64](#uint64) |  |  |
| sync | [bool](#bool) |  | If `true` transfer is a synchronization of a replicas; If `false` transfer is a moving of a shard from one peer to another |






<a name="solvio-TextIndexParams"></a>

### TextIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tokenizer | [TokenizerType](#solvio-TokenizerType) |  | Tokenizer type |
| lowercase | [bool](#bool) | optional | If true - all tokens will be lowercase |
| min_token_len | [uint64](#uint64) | optional | Minimal token length |
| max_token_len | [uint64](#uint64) | optional | Maximal token length |






<a name="solvio-UpdateCollection"></a>

### UpdateCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| optimizers_config | [OptimizersConfigDiff](#solvio-OptimizersConfigDiff) | optional | New configuration parameters for the collection. This operation is blocking, it will only proceed once all current optimizations are complete |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds if blocking, if not specified - default value will be supplied |
| params | [CollectionParamsDiff](#solvio-CollectionParamsDiff) | optional | New configuration parameters for the collection |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) | optional | New HNSW parameters for the collection index |
| vectors_config | [VectorsConfigDiff](#solvio-VectorsConfigDiff) | optional | New vector parameters |
| quantization_config | [QuantizationConfigDiff](#solvio-QuantizationConfigDiff) | optional | Quantization configuration of vector |






<a name="solvio-UpdateCollectionClusterSetupRequest"></a>

### UpdateCollectionClusterSetupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| move_shard | [MoveShard](#solvio-MoveShard) |  |  |
| replicate_shard | [MoveShard](#solvio-MoveShard) |  |  |
| abort_transfer | [MoveShard](#solvio-MoveShard) |  |  |
| drop_replica | [Replica](#solvio-Replica) |  |  |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="solvio-UpdateCollectionClusterSetupResponse"></a>

### UpdateCollectionClusterSetupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [bool](#bool) |  |  |






<a name="solvio-VectorParams"></a>

### VectorParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| size | [uint64](#uint64) |  | Size of the vectors |
| distance | [Distance](#solvio-Distance) |  | Distance function used for comparing vectors |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) | optional | Configuration of vector HNSW graph. If omitted - the collection configuration will be used |
| quantization_config | [QuantizationConfig](#solvio-QuantizationConfig) | optional | Configuration of vector quantization config. If omitted - the collection configuration will be used |
| on_disk | [bool](#bool) | optional | If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM. |






<a name="solvio-VectorParamsDiff"></a>

### VectorParamsDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) | optional | Update params for HNSW index. If empty object - it will be unset |
| quantization_config | [QuantizationConfigDiff](#solvio-QuantizationConfigDiff) | optional | Update quantization params. If none - it is left unchanged. |
| on_disk | [bool](#bool) | optional | If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM. |






<a name="solvio-VectorParamsDiffMap"></a>

### VectorParamsDiffMap



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| map | [VectorParamsDiffMap.MapEntry](#solvio-VectorParamsDiffMap-MapEntry) | repeated |  |






<a name="solvio-VectorParamsDiffMap-MapEntry"></a>

### VectorParamsDiffMap.MapEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [VectorParamsDiff](#solvio-VectorParamsDiff) |  |  |






<a name="solvio-VectorParamsMap"></a>

### VectorParamsMap



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| map | [VectorParamsMap.MapEntry](#solvio-VectorParamsMap-MapEntry) | repeated |  |






<a name="solvio-VectorParamsMap-MapEntry"></a>

### VectorParamsMap.MapEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [VectorParams](#solvio-VectorParams) |  |  |






<a name="solvio-VectorsConfig"></a>

### VectorsConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [VectorParams](#solvio-VectorParams) |  |  |
| params_map | [VectorParamsMap](#solvio-VectorParamsMap) |  |  |






<a name="solvio-VectorsConfigDiff"></a>

### VectorsConfigDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [VectorParamsDiff](#solvio-VectorParamsDiff) |  |  |
| params_map | [VectorParamsDiffMap](#solvio-VectorParamsDiffMap) |  |  |






<a name="solvio-WalConfigDiff"></a>

### WalConfigDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| wal_capacity_mb | [uint64](#uint64) | optional | Size of a single WAL block file |
| wal_segments_ahead | [uint64](#uint64) | optional | Number of segments to create in advance |





 


<a name="solvio-CollectionStatus"></a>

### CollectionStatus


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownCollectionStatus | 0 |  |
| Green | 1 | All segments are ready |
| Yellow | 2 | Optimization in process |
| Red | 3 | Something went wrong |



<a name="solvio-CompressionRatio"></a>

### CompressionRatio


| Name | Number | Description |
| ---- | ------ | ----------- |
| x4 | 0 |  |
| x8 | 1 |  |
| x16 | 2 |  |
| x32 | 3 |  |
| x64 | 4 |  |



<a name="solvio-Distance"></a>

### Distance


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownDistance | 0 |  |
| Cosine | 1 |  |
| Euclid | 2 |  |
| Dot | 3 |  |



<a name="solvio-PayloadSchemaType"></a>

### PayloadSchemaType


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownType | 0 |  |
| Keyword | 1 |  |
| Integer | 2 |  |
| Float | 3 |  |
| Geo | 4 |  |
| Text | 5 |  |
| Bool | 6 |  |



<a name="solvio-QuantizationType"></a>

### QuantizationType


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownQuantization | 0 |  |
| Int8 | 1 |  |



<a name="solvio-ReplicaState"></a>

### ReplicaState


| Name | Number | Description |
| ---- | ------ | ----------- |
| Active | 0 | Active and sound |
| Dead | 1 | Failed for some reason |
| Partial | 2 | The shard is partially loaded and is currently receiving data from other shards |
| Initializing | 3 | Collection is being created |
| Listener | 4 | A shard which receives data, but is not used for search; Useful for backup shards |



<a name="solvio-TokenizerType"></a>

### TokenizerType


| Name | Number | Description |
| ---- | ------ | ----------- |
| Unknown | 0 |  |
| Prefix | 1 |  |
| Whitespace | 2 |  |
| Word | 3 |  |
| Multilingual | 4 |  |


 

 

 



<a name="collections_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## collections_service.proto


 

 

 


<a name="solvio-Collections"></a>

### Collections


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Get | [GetCollectionInfoRequest](#solvio-GetCollectionInfoRequest) | [GetCollectionInfoResponse](#solvio-GetCollectionInfoResponse) | Get detailed information about specified existing collection |
| List | [ListCollectionsRequest](#solvio-ListCollectionsRequest) | [ListCollectionsResponse](#solvio-ListCollectionsResponse) | Get list name of all existing collections |
| Create | [CreateCollection](#solvio-CreateCollection) | [CollectionOperationResponse](#solvio-CollectionOperationResponse) | Create new collection with given parameters |
| Update | [UpdateCollection](#solvio-UpdateCollection) | [CollectionOperationResponse](#solvio-CollectionOperationResponse) | Update parameters of the existing collection |
| Delete | [DeleteCollection](#solvio-DeleteCollection) | [CollectionOperationResponse](#solvio-CollectionOperationResponse) | Drop collection and all associated data |
| UpdateAliases | [ChangeAliases](#solvio-ChangeAliases) | [CollectionOperationResponse](#solvio-CollectionOperationResponse) | Update Aliases of the existing collection |
| ListCollectionAliases | [ListCollectionAliasesRequest](#solvio-ListCollectionAliasesRequest) | [ListAliasesResponse](#solvio-ListAliasesResponse) | Get list of all aliases for a collection |
| ListAliases | [ListAliasesRequest](#solvio-ListAliasesRequest) | [ListAliasesResponse](#solvio-ListAliasesResponse) | Get list of all aliases for all existing collections |
| CollectionClusterInfo | [CollectionClusterInfoRequest](#solvio-CollectionClusterInfoRequest) | [CollectionClusterInfoResponse](#solvio-CollectionClusterInfoResponse) | Get cluster information for a collection |
| UpdateCollectionClusterSetup | [UpdateCollectionClusterSetupRequest](#solvio-UpdateCollectionClusterSetupRequest) | [UpdateCollectionClusterSetupResponse](#solvio-UpdateCollectionClusterSetupResponse) | Update cluster setup for a collection |

 



<a name="health_check-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## health_check.proto
source: https://github.com/grpc/grpc/blob/master/doc/health-checking.md#service-definition


<a name="grpc-health-v1-HealthCheckRequest"></a>

### HealthCheckRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| service | [string](#string) |  |  |






<a name="grpc-health-v1-HealthCheckResponse"></a>

### HealthCheckResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [HealthCheckResponse.ServingStatus](#grpc-health-v1-HealthCheckResponse-ServingStatus) |  |  |





 


<a name="grpc-health-v1-HealthCheckResponse-ServingStatus"></a>

### HealthCheckResponse.ServingStatus


| Name | Number | Description |
| ---- | ------ | ----------- |
| UNKNOWN | 0 |  |
| SERVING | 1 |  |
| NOT_SERVING | 2 |  |
| SERVICE_UNKNOWN | 3 | Used only by the Watch method. |


 

 


<a name="grpc-health-v1-Health"></a>

### Health


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Check | [HealthCheckRequest](#grpc-health-v1-HealthCheckRequest) | [HealthCheckResponse](#grpc-health-v1-HealthCheckResponse) |  |

 



<a name="json_with_int-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## json_with_int.proto



<a name="solvio-ListValue"></a>

### ListValue
`ListValue` is a wrapper around a repeated field of values.

The JSON representation for `ListValue` is a JSON array.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [Value](#solvio-Value) | repeated | Repeated field of dynamically typed values. |






<a name="solvio-Struct"></a>

### Struct
`Struct` represents a structured data value, consisting of fields
which map to dynamically typed values. In some languages, `Struct`
might be supported by a native representation. For example, in
scripting languages like JS a struct is represented as an
object. The details of that representation are described together
with the proto support for the language.

The JSON representation for `Struct` is a JSON object.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fields | [Struct.FieldsEntry](#solvio-Struct-FieldsEntry) | repeated | Unordered map of dynamically typed values. |






<a name="solvio-Struct-FieldsEntry"></a>

### Struct.FieldsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-Value"></a>

### Value
`Value` represents a dynamically typed value which can be either
null, a number, a string, a boolean, a recursive struct value, or a
list of values. A producer of value is expected to set one of those
variants, absence of any variant indicates an error.

The JSON representation for `Value` is a JSON value.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| null_value | [NullValue](#solvio-NullValue) |  | Represents a null value. |
| double_value | [double](#double) |  | Represents a double value. |
| integer_value | [int64](#int64) |  | Represents an integer value |
| string_value | [string](#string) |  | Represents a string value. |
| bool_value | [bool](#bool) |  | Represents a boolean value. |
| struct_value | [Struct](#solvio-Struct) |  | Represents a structured value. |
| list_value | [ListValue](#solvio-ListValue) |  | Represents a repeated `Value`. |





 


<a name="solvio-NullValue"></a>

### NullValue
`NullValue` is a singleton enumeration to represent the null value for the
`Value` type union.

 The JSON representation for `NullValue` is JSON `null`.

| Name | Number | Description |
| ---- | ------ | ----------- |
| NULL_VALUE | 0 | Null value. |


 

 

 



<a name="points-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## points.proto



<a name="solvio-BatchResult"></a>

### BatchResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#solvio-ScoredPoint) | repeated |  |






<a name="solvio-ClearPayloadPoints"></a>

### ClearPayloadPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-Condition"></a>

### Condition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| field | [FieldCondition](#solvio-FieldCondition) |  |  |
| is_empty | [IsEmptyCondition](#solvio-IsEmptyCondition) |  |  |
| has_id | [HasIdCondition](#solvio-HasIdCondition) |  |  |
| filter | [Filter](#solvio-Filter) |  |  |
| is_null | [IsNullCondition](#solvio-IsNullCondition) |  |  |
| nested | [NestedCondition](#solvio-NestedCondition) |  |  |






<a name="solvio-CountPoints"></a>

### CountPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| exact | [bool](#bool) | optional | If `true` - return exact count, if `false` - return approximate count |






<a name="solvio-CountResponse"></a>

### CountResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [CountResult](#solvio-CountResult) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-CountResult"></a>

### CountResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| count | [uint64](#uint64) |  |  |






<a name="solvio-CreateFieldIndexCollection"></a>

### CreateFieldIndexCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| field_name | [string](#string) |  | Field name to index |
| field_type | [FieldType](#solvio-FieldType) | optional | Field type. |
| field_index_params | [PayloadIndexParams](#solvio-PayloadIndexParams) | optional | Payload index params. |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-DeleteFieldIndexCollection"></a>

### DeleteFieldIndexCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| field_name | [string](#string) |  | Field name to delete |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-DeletePayloadPoints"></a>

### DeletePayloadPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| keys | [string](#string) | repeated | List of keys to delete |
| points_selector | [PointsSelector](#solvio-PointsSelector) | optional | Affected points |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-DeletePointVectors"></a>

### DeletePointVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points_selector | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| vectors | [VectorsSelector](#solvio-VectorsSelector) |  | List of vector names to delete |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-DeletePoints"></a>

### DeletePoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-FieldCondition"></a>

### FieldCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| match | [Match](#solvio-Match) |  | Check if point has field with a given value |
| range | [Range](#solvio-Range) |  | Check if points value lies in a given range |
| geo_bounding_box | [GeoBoundingBox](#solvio-GeoBoundingBox) |  | Check if points geolocation lies in a given area |
| geo_radius | [GeoRadius](#solvio-GeoRadius) |  | Check if geo point is within a given radius |
| values_count | [ValuesCount](#solvio-ValuesCount) |  | Check number of values for a specific field |
| geo_polygon | [GeoPolygon](#solvio-GeoPolygon) |  | Check if geo point is within a given polygon |






<a name="solvio-Filter"></a>

### Filter



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| should | [Condition](#solvio-Condition) | repeated | At least one of those conditions should match |
| must | [Condition](#solvio-Condition) | repeated | All conditions must match |
| must_not | [Condition](#solvio-Condition) | repeated | All conditions must NOT match |






<a name="solvio-GeoBoundingBox"></a>

### GeoBoundingBox



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| top_left | [GeoPoint](#solvio-GeoPoint) |  | north-west corner |
| bottom_right | [GeoPoint](#solvio-GeoPoint) |  | south-east corner |






<a name="solvio-GeoLineString"></a>

### GeoLineString



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points | [GeoPoint](#solvio-GeoPoint) | repeated | Ordered sequence of GeoPoints representing the line |






<a name="solvio-GeoPoint"></a>

### GeoPoint



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lon | [double](#double) |  |  |
| lat | [double](#double) |  |  |






<a name="solvio-GeoPolygon"></a>

### GeoPolygon
For a valid GeoPolygon, both the exterior and interior GeoLineStrings must consist of a minimum of 4 points.
Additionally, the first and last points of each GeoLineString must be the same.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| exterior | [GeoLineString](#solvio-GeoLineString) |  | The exterior line bounds the surface |
| interiors | [GeoLineString](#solvio-GeoLineString) | repeated | Interior lines (if present) bound holes within the surface |






<a name="solvio-GeoRadius"></a>

### GeoRadius



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| center | [GeoPoint](#solvio-GeoPoint) |  | Center of the circle |
| radius | [float](#float) |  | In meters |






<a name="solvio-GetPoints"></a>

### GetPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| ids | [PointId](#solvio-PointId) | repeated | List of points to retrieve |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="solvio-GetResponse"></a>

### GetResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [RetrievedPoint](#solvio-RetrievedPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-GroupId"></a>

### GroupId



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unsigned_value | [uint64](#uint64) |  | Represents a double value. |
| integer_value | [int64](#int64) |  | Represents an integer value |
| string_value | [string](#string) |  | Represents a string value. |






<a name="solvio-GroupsResult"></a>

### GroupsResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| groups | [PointGroup](#solvio-PointGroup) | repeated | Groups |






<a name="solvio-HasIdCondition"></a>

### HasIdCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| has_id | [PointId](#solvio-PointId) | repeated |  |






<a name="solvio-IsEmptyCondition"></a>

### IsEmptyCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |






<a name="solvio-IsNullCondition"></a>

### IsNullCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |






<a name="solvio-LookupLocation"></a>

### LookupLocation



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  |  |
| vector_name | [string](#string) | optional | Which vector to use for search, if not specified - use default vector |






<a name="solvio-Match"></a>

### Match



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keyword | [string](#string) |  | Match string keyword |
| integer | [int64](#int64) |  | Match integer |
| boolean | [bool](#bool) |  | Match boolean |
| text | [string](#string) |  | Match text |
| keywords | [RepeatedStrings](#solvio-RepeatedStrings) |  | Match multiple keywords |
| integers | [RepeatedIntegers](#solvio-RepeatedIntegers) |  | Match multiple integers |
| except_integers | [RepeatedIntegers](#solvio-RepeatedIntegers) |  | Match any other value except those integers |
| except_keywords | [RepeatedStrings](#solvio-RepeatedStrings) |  | Match any other value except those keywords |






<a name="solvio-NamedVectors"></a>

### NamedVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vectors | [NamedVectors.VectorsEntry](#solvio-NamedVectors-VectorsEntry) | repeated |  |






<a name="solvio-NamedVectors-VectorsEntry"></a>

### NamedVectors.VectorsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Vector](#solvio-Vector) |  |  |






<a name="solvio-NestedCondition"></a>

### NestedCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  | Path to nested object |
| filter | [Filter](#solvio-Filter) |  | Filter condition |






<a name="solvio-PayloadExcludeSelector"></a>

### PayloadExcludeSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fields | [string](#string) | repeated | List of payload keys to exclude from the result |






<a name="solvio-PayloadIncludeSelector"></a>

### PayloadIncludeSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fields | [string](#string) | repeated | List of payload keys to include into result |






<a name="solvio-PointGroup"></a>

### PointGroup



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [GroupId](#solvio-GroupId) |  | Group id |
| hits | [ScoredPoint](#solvio-ScoredPoint) | repeated | Points in the group |
| lookup | [RetrievedPoint](#solvio-RetrievedPoint) |  | Point(s) from the lookup collection that matches the group id |






<a name="solvio-PointId"></a>

### PointId



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| num | [uint64](#uint64) |  | Numerical ID of the point |
| uuid | [string](#string) |  | UUID |






<a name="solvio-PointStruct"></a>

### PointStruct



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#solvio-PointId) |  |  |
| payload | [PointStruct.PayloadEntry](#solvio-PointStruct-PayloadEntry) | repeated |  |
| vectors | [Vectors](#solvio-Vectors) | optional |  |






<a name="solvio-PointStruct-PayloadEntry"></a>

### PointStruct.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-PointVectors"></a>

### PointVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#solvio-PointId) |  | ID to update vectors for |
| vectors | [Vectors](#solvio-Vectors) |  | Named vectors to update, leave others intact |






<a name="solvio-PointsIdsList"></a>

### PointsIdsList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ids | [PointId](#solvio-PointId) | repeated |  |






<a name="solvio-PointsOperationResponse"></a>

### PointsOperationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [UpdateResult](#solvio-UpdateResult) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-PointsSelector"></a>

### PointsSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points | [PointsIdsList](#solvio-PointsIdsList) |  |  |
| filter | [Filter](#solvio-Filter) |  |  |






<a name="solvio-PointsUpdateOperation"></a>

### PointsUpdateOperation



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| upsert | [PointsUpdateOperation.PointStructList](#solvio-PointsUpdateOperation-PointStructList) |  |  |
| delete | [PointsSelector](#solvio-PointsSelector) |  |  |
| set_payload | [PointsUpdateOperation.SetPayload](#solvio-PointsUpdateOperation-SetPayload) |  |  |
| overwrite_payload | [PointsUpdateOperation.SetPayload](#solvio-PointsUpdateOperation-SetPayload) |  |  |
| delete_payload | [PointsUpdateOperation.DeletePayload](#solvio-PointsUpdateOperation-DeletePayload) |  |  |
| clear_payload | [PointsSelector](#solvio-PointsSelector) |  |  |
| update_vectors | [PointsUpdateOperation.UpdateVectors](#solvio-PointsUpdateOperation-UpdateVectors) |  |  |
| delete_vectors | [PointsUpdateOperation.DeleteVectors](#solvio-PointsUpdateOperation-DeleteVectors) |  |  |






<a name="solvio-PointsUpdateOperation-DeletePayload"></a>

### PointsUpdateOperation.DeletePayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keys | [string](#string) | repeated |  |
| points_selector | [PointsSelector](#solvio-PointsSelector) | optional | Affected points |






<a name="solvio-PointsUpdateOperation-DeleteVectors"></a>

### PointsUpdateOperation.DeleteVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points_selector | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| vectors | [VectorsSelector](#solvio-VectorsSelector) |  | List of vector names to delete |






<a name="solvio-PointsUpdateOperation-PointStructList"></a>

### PointsUpdateOperation.PointStructList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points | [PointStruct](#solvio-PointStruct) | repeated |  |






<a name="solvio-PointsUpdateOperation-SetPayload"></a>

### PointsUpdateOperation.SetPayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payload | [PointsUpdateOperation.SetPayload.PayloadEntry](#solvio-PointsUpdateOperation-SetPayload-PayloadEntry) | repeated |  |
| points_selector | [PointsSelector](#solvio-PointsSelector) | optional | Affected points |






<a name="solvio-PointsUpdateOperation-SetPayload-PayloadEntry"></a>

### PointsUpdateOperation.SetPayload.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-PointsUpdateOperation-UpdateVectors"></a>

### PointsUpdateOperation.UpdateVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points | [PointVectors](#solvio-PointVectors) | repeated | List of points and vectors to update |






<a name="solvio-QuantizationSearchParams"></a>

### QuantizationSearchParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ignore | [bool](#bool) | optional | If set to true, search will ignore quantized vector data |
| rescore | [bool](#bool) | optional | If true, use original vectors to re-score top-k results. If ignored, solvio decides automatically does rescore enabled or not. |
| oversampling | [double](#double) | optional | Oversampling factor for quantization.

Defines how many extra vectors should be pre-selected using quantized index, and then re-scored using original vectors.

For example, if `oversampling` is 2.4 and `limit` is 100, then 240 vectors will be pre-selected using quantized index, and then top-100 will be returned after re-scoring. |






<a name="solvio-Range"></a>

### Range



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lt | [double](#double) | optional |  |
| gt | [double](#double) | optional |  |
| gte | [double](#double) | optional |  |
| lte | [double](#double) | optional |  |






<a name="solvio-ReadConsistency"></a>

### ReadConsistency



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [ReadConsistencyType](#solvio-ReadConsistencyType) |  | Common read consistency configurations |
| factor | [uint64](#uint64) |  | Send request to a specified number of nodes, and return points which are present on all of them |






<a name="solvio-RecommendBatchPoints"></a>

### RecommendBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| recommend_points | [RecommendPoints](#solvio-RecommendPoints) | repeated |  |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="solvio-RecommendBatchResponse"></a>

### RecommendBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#solvio-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-RecommendGroupsResponse"></a>

### RecommendGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [GroupsResult](#solvio-GroupsResult) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-RecommendPointGroups"></a>

### RecommendPointGroups



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| positive | [PointId](#solvio-PointId) | repeated | Look for vectors closest to the vectors from these points |
| negative | [PointId](#solvio-PointId) | repeated | Try to avoid vectors like the vector from these points |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint32](#uint32) |  | Max number of groups in result |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#solvio-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| using | [string](#string) | optional | Define which vector to use for recommendation, if not specified - default vector |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| lookup_from | [LookupLocation](#solvio-LookupLocation) | optional | Name of the collection to use for points lookup, if not specified - use current collection |
| group_by | [string](#string) |  | Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups. |
| group_size | [uint32](#uint32) |  | Maximum amount of points to return per group |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| with_lookup | [WithLookup](#solvio-WithLookup) | optional | Options for specifying how to use the group id to lookup points in another collection |
| strategy | [RecommendStrategy](#solvio-RecommendStrategy) | optional | How to use the example vectors to find the results |
| positive_vectors | [Vector](#solvio-Vector) | repeated | Look for vectors closest to those |
| negative_vectors | [Vector](#solvio-Vector) | repeated | Try to avoid vectors like this |






<a name="solvio-RecommendPoints"></a>

### RecommendPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| positive | [PointId](#solvio-PointId) | repeated | Look for vectors closest to the vectors from these points |
| negative | [PointId](#solvio-PointId) | repeated | Try to avoid vectors like the vector from these points |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint64](#uint64) |  | Max number of result |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#solvio-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| offset | [uint64](#uint64) | optional | Offset of the result |
| using | [string](#string) | optional | Define which vector to use for recommendation, if not specified - default vector |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| lookup_from | [LookupLocation](#solvio-LookupLocation) | optional | Name of the collection to use for points lookup, if not specified - use current collection |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| strategy | [RecommendStrategy](#solvio-RecommendStrategy) | optional | How to use the example vectors to find the results |
| positive_vectors | [Vector](#solvio-Vector) | repeated | Look for vectors closest to those |
| negative_vectors | [Vector](#solvio-Vector) | repeated | Try to avoid vectors like this |






<a name="solvio-RecommendResponse"></a>

### RecommendResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#solvio-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-RepeatedIntegers"></a>

### RepeatedIntegers



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integers | [int64](#int64) | repeated |  |






<a name="solvio-RepeatedStrings"></a>

### RepeatedStrings



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| strings | [string](#string) | repeated |  |






<a name="solvio-RetrievedPoint"></a>

### RetrievedPoint



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#solvio-PointId) |  |  |
| payload | [RetrievedPoint.PayloadEntry](#solvio-RetrievedPoint-PayloadEntry) | repeated |  |
| vectors | [Vectors](#solvio-Vectors) | optional |  |






<a name="solvio-RetrievedPoint-PayloadEntry"></a>

### RetrievedPoint.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-ScoredPoint"></a>

### ScoredPoint



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#solvio-PointId) |  | Point id |
| payload | [ScoredPoint.PayloadEntry](#solvio-ScoredPoint-PayloadEntry) | repeated | Payload |
| score | [float](#float) |  | Similarity score |
| version | [uint64](#uint64) |  | Last update operation applied to this point |
| vectors | [Vectors](#solvio-Vectors) | optional | Vectors to search |






<a name="solvio-ScoredPoint-PayloadEntry"></a>

### ScoredPoint.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-ScrollPoints"></a>

### ScrollPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  |  |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| offset | [PointId](#solvio-PointId) | optional | Start with this ID |
| limit | [uint32](#uint32) | optional | Max number of result |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="solvio-ScrollResponse"></a>

### ScrollResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| next_page_offset | [PointId](#solvio-PointId) | optional | Use this offset for the next query |
| result | [RetrievedPoint](#solvio-RetrievedPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-SearchBatchPoints"></a>

### SearchBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| search_points | [SearchPoints](#solvio-SearchPoints) | repeated |  |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="solvio-SearchBatchResponse"></a>

### SearchBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#solvio-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-SearchGroupsResponse"></a>

### SearchGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [GroupsResult](#solvio-GroupsResult) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-SearchParams"></a>

### SearchParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hnsw_ef | [uint64](#uint64) | optional | Params relevant to HNSW index. Size of the beam in a beam-search. Larger the value - more accurate the result, more time required for search. |
| exact | [bool](#bool) | optional | Search without approximation. If set to true, search may run long but with exact results. |
| quantization | [QuantizationSearchParams](#solvio-QuantizationSearchParams) | optional | If set to true, search will ignore quantized vector data |
| indexed_only | [bool](#bool) | optional | If enabled, the engine will only perform search among indexed or small segments. Using this option prevents slow searches in case of delayed index, but does not guarantee that all uploaded vectors will be included in search results |






<a name="solvio-SearchPointGroups"></a>

### SearchPointGroups



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| vector | [float](#float) | repeated | Vector to compare against |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint32](#uint32) |  | Max number of result |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#solvio-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| vector_name | [string](#string) | optional | Which vector to use for search, if not specified - use default vector |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| group_by | [string](#string) |  | Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups. |
| group_size | [uint32](#uint32) |  | Maximum amount of points to return per group |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| with_lookup | [WithLookup](#solvio-WithLookup) | optional | Options for specifying how to use the group id to lookup points in another collection |






<a name="solvio-SearchPoints"></a>

### SearchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| vector | [float](#float) | repeated | vector |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint64](#uint64) |  | Max number of result |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#solvio-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| offset | [uint64](#uint64) | optional | Offset of the result |
| vector_name | [string](#string) | optional | Which vector to use for search, if not specified - use default vector |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |






<a name="solvio-SearchResponse"></a>

### SearchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#solvio-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-SetPayloadPoints"></a>

### SetPayloadPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| payload | [SetPayloadPoints.PayloadEntry](#solvio-SetPayloadPoints-PayloadEntry) | repeated | New payload values |
| points_selector | [PointsSelector](#solvio-PointsSelector) | optional | Affected points |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-SetPayloadPoints-PayloadEntry"></a>

### SetPayloadPoints.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-UpdateBatchPoints"></a>

### UpdateBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| operations | [PointsUpdateOperation](#solvio-PointsUpdateOperation) | repeated |  |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-UpdateBatchResponse"></a>

### UpdateBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [UpdateResult](#solvio-UpdateResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-UpdatePointVectors"></a>

### UpdatePointVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointVectors](#solvio-PointVectors) | repeated | List of points and vectors to update |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-UpdateResult"></a>

### UpdateResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| operation_id | [uint64](#uint64) |  | Number of operation |
| status | [UpdateStatus](#solvio-UpdateStatus) |  | Operation status |






<a name="solvio-UpsertPoints"></a>

### UpsertPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointStruct](#solvio-PointStruct) | repeated |  |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |






<a name="solvio-ValuesCount"></a>

### ValuesCount



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lt | [uint64](#uint64) | optional |  |
| gt | [uint64](#uint64) | optional |  |
| gte | [uint64](#uint64) | optional |  |
| lte | [uint64](#uint64) | optional |  |






<a name="solvio-Vector"></a>

### Vector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [float](#float) | repeated |  |






<a name="solvio-Vectors"></a>

### Vectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vector | [Vector](#solvio-Vector) |  |  |
| vectors | [NamedVectors](#solvio-NamedVectors) |  |  |






<a name="solvio-VectorsSelector"></a>

### VectorsSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| names | [string](#string) | repeated | List of vectors to include into result |






<a name="solvio-WithLookup"></a>

### WithLookup



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection | [string](#string) |  | Name of the collection to use for points lookup |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) | optional | Options for specifying which payload to include (or not) |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include (or not) |






<a name="solvio-WithPayloadSelector"></a>

### WithPayloadSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enable | [bool](#bool) |  | If `true` - return all payload, if `false` - none |
| include | [PayloadIncludeSelector](#solvio-PayloadIncludeSelector) |  |  |
| exclude | [PayloadExcludeSelector](#solvio-PayloadExcludeSelector) |  |  |






<a name="solvio-WithVectorsSelector"></a>

### WithVectorsSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enable | [bool](#bool) |  | If `true` - return all vectors, if `false` - none |
| include | [VectorsSelector](#solvio-VectorsSelector) |  | List of payload keys to include into result |






<a name="solvio-WriteOrdering"></a>

### WriteOrdering



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [WriteOrderingType](#solvio-WriteOrderingType) |  | Write ordering guarantees |





 


<a name="solvio-FieldType"></a>

### FieldType


| Name | Number | Description |
| ---- | ------ | ----------- |
| FieldTypeKeyword | 0 |  |
| FieldTypeInteger | 1 |  |
| FieldTypeFloat | 2 |  |
| FieldTypeGeo | 3 |  |
| FieldTypeText | 4 |  |
| FieldTypeBool | 5 |  |



<a name="solvio-ReadConsistencyType"></a>

### ReadConsistencyType


| Name | Number | Description |
| ---- | ------ | ----------- |
| All | 0 | Send request to all nodes and return points which are present on all of them |
| Majority | 1 | Send requests to all nodes and return points which are present on majority of them |
| Quorum | 2 | Send requests to half &#43; 1 nodes, return points which are present on all of them |



<a name="solvio-RecommendStrategy"></a>

### RecommendStrategy
How to use positive and negative vectors to find the results, default is `AverageVector`:

| Name | Number | Description |
| ---- | ------ | ----------- |
| AverageVector | 0 | Average positive and negative vectors and create a single query with the formula `query = avg_pos &#43; avg_pos - avg_neg`. Then performs normal search. |
| BestScore | 1 | Uses custom search objective. Each candidate is compared against all examples, its score is then chosen from the `max(max_pos_score, max_neg_score)`. If the `max_neg_score` is chosen then it is squared and negated. |



<a name="solvio-UpdateStatus"></a>

### UpdateStatus


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownUpdateStatus | 0 |  |
| Acknowledged | 1 | Update is received, but not processed yet |
| Completed | 2 | Update is applied and ready for search |



<a name="solvio-WriteOrderingType"></a>

### WriteOrderingType


| Name | Number | Description |
| ---- | ------ | ----------- |
| Weak | 0 | Write operations may be reordered, works faster, default |
| Medium | 1 | Write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change |
| Strong | 2 | Write operations go through the permanent leader, consistent, but may be unavailable if leader is down |


 

 

 



<a name="points_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## points_service.proto


 

 

 


<a name="solvio-Points"></a>

### Points


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Upsert | [UpsertPoints](#solvio-UpsertPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Perform insert &#43; updates on points. If a point with a given ID already exists - it will be overwritten. |
| Delete | [DeletePoints](#solvio-DeletePoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Delete points |
| Get | [GetPoints](#solvio-GetPoints) | [GetResponse](#solvio-GetResponse) | Retrieve points |
| UpdateVectors | [UpdatePointVectors](#solvio-UpdatePointVectors) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Update named vectors for point |
| DeleteVectors | [DeletePointVectors](#solvio-DeletePointVectors) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Delete named vectors for points |
| SetPayload | [SetPayloadPoints](#solvio-SetPayloadPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Set payload for points |
| OverwritePayload | [SetPayloadPoints](#solvio-SetPayloadPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Overwrite payload for points |
| DeletePayload | [DeletePayloadPoints](#solvio-DeletePayloadPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Delete specified key payload for points |
| ClearPayload | [ClearPayloadPoints](#solvio-ClearPayloadPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Remove all payload for specified points |
| CreateFieldIndex | [CreateFieldIndexCollection](#solvio-CreateFieldIndexCollection) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Create index for field in collection |
| DeleteFieldIndex | [DeleteFieldIndexCollection](#solvio-DeleteFieldIndexCollection) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Delete field index for collection |
| Search | [SearchPoints](#solvio-SearchPoints) | [SearchResponse](#solvio-SearchResponse) | Retrieve closest points based on vector similarity and given filtering conditions |
| SearchBatch | [SearchBatchPoints](#solvio-SearchBatchPoints) | [SearchBatchResponse](#solvio-SearchBatchResponse) | Retrieve closest points based on vector similarity and given filtering conditions |
| SearchGroups | [SearchPointGroups](#solvio-SearchPointGroups) | [SearchGroupsResponse](#solvio-SearchGroupsResponse) | Retrieve closest points based on vector similarity and given filtering conditions, grouped by a given field |
| Scroll | [ScrollPoints](#solvio-ScrollPoints) | [ScrollResponse](#solvio-ScrollResponse) | Iterate over all or filtered points |
| Recommend | [RecommendPoints](#solvio-RecommendPoints) | [RecommendResponse](#solvio-RecommendResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples. |
| RecommendBatch | [RecommendBatchPoints](#solvio-RecommendBatchPoints) | [RecommendBatchResponse](#solvio-RecommendBatchResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples. |
| RecommendGroups | [RecommendPointGroups](#solvio-RecommendPointGroups) | [RecommendGroupsResponse](#solvio-RecommendGroupsResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples, grouped by a given field |
| Count | [CountPoints](#solvio-CountPoints) | [CountResponse](#solvio-CountResponse) | Count points in collection with given filtering conditions |
| UpdateBatch | [UpdateBatchPoints](#solvio-UpdateBatchPoints) | [UpdateBatchResponse](#solvio-UpdateBatchResponse) | Perform multiple update operations in one request |

 



<a name="solvio-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## solvio.proto



<a name="solvio-HealthCheckReply"></a>

### HealthCheckReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| title | [string](#string) |  |  |
| version | [string](#string) |  |  |






<a name="solvio-HealthCheckRequest"></a>

### HealthCheckRequest






 

 

 


<a name="solvio-Solvio"></a>

### Solvio


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| HealthCheck | [HealthCheckRequest](#solvio-HealthCheckRequest) | [HealthCheckReply](#solvio-HealthCheckReply) |  |

 



<a name="solvio_internal_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## solvio_internal_service.proto



<a name="solvio-HttpPortRequest"></a>

### HttpPortRequest







<a name="solvio-HttpPortResponse"></a>

### HttpPortResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| port | [int32](#int32) |  |  |






<a name="solvio-WaitOnConsensusCommitRequest"></a>

### WaitOnConsensusCommitRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| commit | [int64](#int64) |  | Raft commit as u64 |
| term | [int64](#int64) |  | Raft term as u64 |
| timeout | [int64](#int64) |  | Timeout in seconds |






<a name="solvio-WaitOnConsensusCommitResponse"></a>

### WaitOnConsensusCommitResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ok | [bool](#bool) |  | False if commit/term is diverged and never reached or if timed out. |





 

 

 


<a name="solvio-SolvioInternal"></a>

### SolvioInternal


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetHttpPort | [HttpPortRequest](#solvio-HttpPortRequest) | [HttpPortResponse](#solvio-HttpPortResponse) | Get HTTP port for remote host. |
| WaitOnConsensusCommit | [WaitOnConsensusCommitRequest](#solvio-WaitOnConsensusCommitRequest) | [WaitOnConsensusCommitResponse](#solvio-WaitOnConsensusCommitResponse) | Wait until the target node reached the given commit ID. |

 



<a name="snapshots_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## snapshots_service.proto



<a name="solvio-CreateFullSnapshotRequest"></a>

### CreateFullSnapshotRequest







<a name="solvio-CreateSnapshotRequest"></a>

### CreateSnapshotRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="solvio-CreateSnapshotResponse"></a>

### CreateSnapshotResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| snapshot_description | [SnapshotDescription](#solvio-SnapshotDescription) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-DeleteFullSnapshotRequest"></a>

### DeleteFullSnapshotRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| snapshot_name | [string](#string) |  | Name of the full snapshot |






<a name="solvio-DeleteSnapshotRequest"></a>

### DeleteSnapshotRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| snapshot_name | [string](#string) |  | Name of the collection snapshot |






<a name="solvio-DeleteSnapshotResponse"></a>

### DeleteSnapshotResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-ListFullSnapshotsRequest"></a>

### ListFullSnapshotsRequest







<a name="solvio-ListSnapshotsRequest"></a>

### ListSnapshotsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






<a name="solvio-ListSnapshotsResponse"></a>

### ListSnapshotsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| snapshot_descriptions | [SnapshotDescription](#solvio-SnapshotDescription) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-SnapshotDescription"></a>

### SnapshotDescription



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name of the snapshot |
| creation_time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Creation time of the snapshot |
| size | [int64](#int64) |  | Size of the snapshot in bytes |





 

 

 


<a name="solvio-Snapshots"></a>

### Snapshots


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateSnapshotRequest](#solvio-CreateSnapshotRequest) | [CreateSnapshotResponse](#solvio-CreateSnapshotResponse) | Create collection snapshot |
| List | [ListSnapshotsRequest](#solvio-ListSnapshotsRequest) | [ListSnapshotsResponse](#solvio-ListSnapshotsResponse) | List collection snapshots |
| Delete | [DeleteSnapshotRequest](#solvio-DeleteSnapshotRequest) | [DeleteSnapshotResponse](#solvio-DeleteSnapshotResponse) | Delete collection snapshots |
| CreateFull | [CreateFullSnapshotRequest](#solvio-CreateFullSnapshotRequest) | [CreateSnapshotResponse](#solvio-CreateSnapshotResponse) | Create full storage snapshot |
| ListFull | [ListFullSnapshotsRequest](#solvio-ListFullSnapshotsRequest) | [ListSnapshotsResponse](#solvio-ListSnapshotsResponse) | List full storage snapshots |
| DeleteFull | [DeleteFullSnapshotRequest](#solvio-DeleteFullSnapshotRequest) | [DeleteSnapshotResponse](#solvio-DeleteSnapshotResponse) | List full storage snapshots |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

