# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [collections.proto](#collections-proto)
    - [AbortShardTransfer](#solvio-AbortShardTransfer)
    - [AliasDescription](#solvio-AliasDescription)
    - [AliasOperations](#solvio-AliasOperations)
    - [BinaryQuantization](#solvio-BinaryQuantization)
    - [BoolIndexParams](#solvio-BoolIndexParams)
    - [ChangeAliases](#solvio-ChangeAliases)
    - [CollectionClusterInfoRequest](#solvio-CollectionClusterInfoRequest)
    - [CollectionClusterInfoResponse](#solvio-CollectionClusterInfoResponse)
    - [CollectionConfig](#solvio-CollectionConfig)
    - [CollectionDescription](#solvio-CollectionDescription)
    - [CollectionExists](#solvio-CollectionExists)
    - [CollectionExistsRequest](#solvio-CollectionExistsRequest)
    - [CollectionExistsResponse](#solvio-CollectionExistsResponse)
    - [CollectionInfo](#solvio-CollectionInfo)
    - [CollectionInfo.PayloadSchemaEntry](#solvio-CollectionInfo-PayloadSchemaEntry)
    - [CollectionOperationResponse](#solvio-CollectionOperationResponse)
    - [CollectionParams](#solvio-CollectionParams)
    - [CollectionParamsDiff](#solvio-CollectionParamsDiff)
    - [CreateAlias](#solvio-CreateAlias)
    - [CreateCollection](#solvio-CreateCollection)
    - [CreateShardKey](#solvio-CreateShardKey)
    - [CreateShardKeyRequest](#solvio-CreateShardKeyRequest)
    - [CreateShardKeyResponse](#solvio-CreateShardKeyResponse)
    - [DatetimeIndexParams](#solvio-DatetimeIndexParams)
    - [DeleteAlias](#solvio-DeleteAlias)
    - [DeleteCollection](#solvio-DeleteCollection)
    - [DeleteShardKey](#solvio-DeleteShardKey)
    - [DeleteShardKeyRequest](#solvio-DeleteShardKeyRequest)
    - [DeleteShardKeyResponse](#solvio-DeleteShardKeyResponse)
    - [Disabled](#solvio-Disabled)
    - [FloatIndexParams](#solvio-FloatIndexParams)
    - [GeoIndexParams](#solvio-GeoIndexParams)
    - [GetCollectionInfoRequest](#solvio-GetCollectionInfoRequest)
    - [GetCollectionInfoResponse](#solvio-GetCollectionInfoResponse)
    - [HnswConfigDiff](#solvio-HnswConfigDiff)
    - [IntegerIndexParams](#solvio-IntegerIndexParams)
    - [KeywordIndexParams](#solvio-KeywordIndexParams)
    - [ListAliasesRequest](#solvio-ListAliasesRequest)
    - [ListAliasesResponse](#solvio-ListAliasesResponse)
    - [ListCollectionAliasesRequest](#solvio-ListCollectionAliasesRequest)
    - [ListCollectionsRequest](#solvio-ListCollectionsRequest)
    - [ListCollectionsResponse](#solvio-ListCollectionsResponse)
    - [LocalShardInfo](#solvio-LocalShardInfo)
    - [MaxOptimizationThreads](#solvio-MaxOptimizationThreads)
    - [MoveShard](#solvio-MoveShard)
    - [MultiVectorConfig](#solvio-MultiVectorConfig)
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
    - [ReplicateShard](#solvio-ReplicateShard)
    - [ReshardingInfo](#solvio-ReshardingInfo)
    - [RestartTransfer](#solvio-RestartTransfer)
    - [ScalarQuantization](#solvio-ScalarQuantization)
    - [ShardKey](#solvio-ShardKey)
    - [ShardTransferInfo](#solvio-ShardTransferInfo)
    - [SparseIndexConfig](#solvio-SparseIndexConfig)
    - [SparseVectorConfig](#solvio-SparseVectorConfig)
    - [SparseVectorConfig.MapEntry](#solvio-SparseVectorConfig-MapEntry)
    - [SparseVectorParams](#solvio-SparseVectorParams)
    - [StrictModeConfig](#solvio-StrictModeConfig)
    - [StrictModeMultivector](#solvio-StrictModeMultivector)
    - [StrictModeMultivectorConfig](#solvio-StrictModeMultivectorConfig)
    - [StrictModeMultivectorConfig.MultivectorConfigEntry](#solvio-StrictModeMultivectorConfig-MultivectorConfigEntry)
    - [StrictModeSparse](#solvio-StrictModeSparse)
    - [StrictModeSparseConfig](#solvio-StrictModeSparseConfig)
    - [StrictModeSparseConfig.SparseConfigEntry](#solvio-StrictModeSparseConfig-SparseConfigEntry)
    - [TextIndexParams](#solvio-TextIndexParams)
    - [UpdateCollection](#solvio-UpdateCollection)
    - [UpdateCollectionClusterSetupRequest](#solvio-UpdateCollectionClusterSetupRequest)
    - [UpdateCollectionClusterSetupResponse](#solvio-UpdateCollectionClusterSetupResponse)
    - [UuidIndexParams](#solvio-UuidIndexParams)
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
    - [Datatype](#solvio-Datatype)
    - [Distance](#solvio-Distance)
    - [MaxOptimizationThreads.Setting](#solvio-MaxOptimizationThreads-Setting)
    - [Modifier](#solvio-Modifier)
    - [MultiVectorComparator](#solvio-MultiVectorComparator)
    - [PayloadSchemaType](#solvio-PayloadSchemaType)
    - [QuantizationType](#solvio-QuantizationType)
    - [ReplicaState](#solvio-ReplicaState)
    - [ReshardingDirection](#solvio-ReshardingDirection)
    - [ShardTransferMethod](#solvio-ShardTransferMethod)
    - [ShardingMethod](#solvio-ShardingMethod)
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
    - [ContextExamplePair](#solvio-ContextExamplePair)
    - [ContextInput](#solvio-ContextInput)
    - [ContextInputPair](#solvio-ContextInputPair)
    - [CountPoints](#solvio-CountPoints)
    - [CountResponse](#solvio-CountResponse)
    - [CountResult](#solvio-CountResult)
    - [CreateFieldIndexCollection](#solvio-CreateFieldIndexCollection)
    - [DatetimeRange](#solvio-DatetimeRange)
    - [DecayParamsExpression](#solvio-DecayParamsExpression)
    - [DeleteFieldIndexCollection](#solvio-DeleteFieldIndexCollection)
    - [DeletePayloadPoints](#solvio-DeletePayloadPoints)
    - [DeletePointVectors](#solvio-DeletePointVectors)
    - [DeletePoints](#solvio-DeletePoints)
    - [DenseVector](#solvio-DenseVector)
    - [DiscoverBatchPoints](#solvio-DiscoverBatchPoints)
    - [DiscoverBatchResponse](#solvio-DiscoverBatchResponse)
    - [DiscoverInput](#solvio-DiscoverInput)
    - [DiscoverPoints](#solvio-DiscoverPoints)
    - [DiscoverResponse](#solvio-DiscoverResponse)
    - [DivExpression](#solvio-DivExpression)
    - [Document](#solvio-Document)
    - [Document.OptionsEntry](#solvio-Document-OptionsEntry)
    - [Expression](#solvio-Expression)
    - [FacetCounts](#solvio-FacetCounts)
    - [FacetHit](#solvio-FacetHit)
    - [FacetResponse](#solvio-FacetResponse)
    - [FacetValue](#solvio-FacetValue)
    - [FieldCondition](#solvio-FieldCondition)
    - [Filter](#solvio-Filter)
    - [Formula](#solvio-Formula)
    - [Formula.DefaultsEntry](#solvio-Formula-DefaultsEntry)
    - [GeoBoundingBox](#solvio-GeoBoundingBox)
    - [GeoDistance](#solvio-GeoDistance)
    - [GeoLineString](#solvio-GeoLineString)
    - [GeoPoint](#solvio-GeoPoint)
    - [GeoPolygon](#solvio-GeoPolygon)
    - [GeoRadius](#solvio-GeoRadius)
    - [GetPoints](#solvio-GetPoints)
    - [GetResponse](#solvio-GetResponse)
    - [GroupId](#solvio-GroupId)
    - [GroupsResult](#solvio-GroupsResult)
    - [HardwareUsage](#solvio-HardwareUsage)
    - [HasIdCondition](#solvio-HasIdCondition)
    - [HasVectorCondition](#solvio-HasVectorCondition)
    - [Image](#solvio-Image)
    - [Image.OptionsEntry](#solvio-Image-OptionsEntry)
    - [InferenceObject](#solvio-InferenceObject)
    - [InferenceObject.OptionsEntry](#solvio-InferenceObject-OptionsEntry)
    - [IsEmptyCondition](#solvio-IsEmptyCondition)
    - [IsNullCondition](#solvio-IsNullCondition)
    - [LookupLocation](#solvio-LookupLocation)
    - [Match](#solvio-Match)
    - [MinShould](#solvio-MinShould)
    - [MultExpression](#solvio-MultExpression)
    - [MultiDenseVector](#solvio-MultiDenseVector)
    - [NamedVectors](#solvio-NamedVectors)
    - [NamedVectors.VectorsEntry](#solvio-NamedVectors-VectorsEntry)
    - [NamedVectorsOutput](#solvio-NamedVectorsOutput)
    - [NamedVectorsOutput.VectorsEntry](#solvio-NamedVectorsOutput-VectorsEntry)
    - [NestedCondition](#solvio-NestedCondition)
    - [OrderBy](#solvio-OrderBy)
    - [OrderValue](#solvio-OrderValue)
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
    - [PointsUpdateOperation.ClearPayload](#solvio-PointsUpdateOperation-ClearPayload)
    - [PointsUpdateOperation.DeletePayload](#solvio-PointsUpdateOperation-DeletePayload)
    - [PointsUpdateOperation.DeletePoints](#solvio-PointsUpdateOperation-DeletePoints)
    - [PointsUpdateOperation.DeleteVectors](#solvio-PointsUpdateOperation-DeleteVectors)
    - [PointsUpdateOperation.OverwritePayload](#solvio-PointsUpdateOperation-OverwritePayload)
    - [PointsUpdateOperation.OverwritePayload.PayloadEntry](#solvio-PointsUpdateOperation-OverwritePayload-PayloadEntry)
    - [PointsUpdateOperation.PointStructList](#solvio-PointsUpdateOperation-PointStructList)
    - [PointsUpdateOperation.SetPayload](#solvio-PointsUpdateOperation-SetPayload)
    - [PointsUpdateOperation.SetPayload.PayloadEntry](#solvio-PointsUpdateOperation-SetPayload-PayloadEntry)
    - [PointsUpdateOperation.UpdateVectors](#solvio-PointsUpdateOperation-UpdateVectors)
    - [PowExpression](#solvio-PowExpression)
    - [PrefetchQuery](#solvio-PrefetchQuery)
    - [QuantizationSearchParams](#solvio-QuantizationSearchParams)
    - [Query](#solvio-Query)
    - [QueryBatchPoints](#solvio-QueryBatchPoints)
    - [QueryBatchResponse](#solvio-QueryBatchResponse)
    - [QueryGroupsResponse](#solvio-QueryGroupsResponse)
    - [QueryPointGroups](#solvio-QueryPointGroups)
    - [QueryPoints](#solvio-QueryPoints)
    - [QueryResponse](#solvio-QueryResponse)
    - [Range](#solvio-Range)
    - [ReadConsistency](#solvio-ReadConsistency)
    - [RecommendBatchPoints](#solvio-RecommendBatchPoints)
    - [RecommendBatchResponse](#solvio-RecommendBatchResponse)
    - [RecommendGroupsResponse](#solvio-RecommendGroupsResponse)
    - [RecommendInput](#solvio-RecommendInput)
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
    - [SearchMatrixOffsets](#solvio-SearchMatrixOffsets)
    - [SearchMatrixOffsetsResponse](#solvio-SearchMatrixOffsetsResponse)
    - [SearchMatrixPair](#solvio-SearchMatrixPair)
    - [SearchMatrixPairs](#solvio-SearchMatrixPairs)
    - [SearchMatrixPairsResponse](#solvio-SearchMatrixPairsResponse)
    - [SearchMatrixPoints](#solvio-SearchMatrixPoints)
    - [SearchParams](#solvio-SearchParams)
    - [SearchPointGroups](#solvio-SearchPointGroups)
    - [SearchPoints](#solvio-SearchPoints)
    - [SearchResponse](#solvio-SearchResponse)
    - [SetPayloadPoints](#solvio-SetPayloadPoints)
    - [SetPayloadPoints.PayloadEntry](#solvio-SetPayloadPoints-PayloadEntry)
    - [ShardKeySelector](#solvio-ShardKeySelector)
    - [SparseIndices](#solvio-SparseIndices)
    - [SparseVector](#solvio-SparseVector)
    - [StartFrom](#solvio-StartFrom)
    - [SumExpression](#solvio-SumExpression)
    - [TargetVector](#solvio-TargetVector)
    - [UpdateBatchPoints](#solvio-UpdateBatchPoints)
    - [UpdateBatchResponse](#solvio-UpdateBatchResponse)
    - [UpdatePointVectors](#solvio-UpdatePointVectors)
    - [UpdateResult](#solvio-UpdateResult)
    - [UpsertPoints](#solvio-UpsertPoints)
    - [Usage](#solvio-Usage)
    - [ValuesCount](#solvio-ValuesCount)
    - [Vector](#solvio-Vector)
    - [VectorExample](#solvio-VectorExample)
    - [VectorInput](#solvio-VectorInput)
    - [VectorOutput](#solvio-VectorOutput)
    - [Vectors](#solvio-Vectors)
    - [VectorsOutput](#solvio-VectorsOutput)
    - [VectorsSelector](#solvio-VectorsSelector)
    - [WithLookup](#solvio-WithLookup)
    - [WithPayloadSelector](#solvio-WithPayloadSelector)
    - [WithVectorsSelector](#solvio-WithVectorsSelector)
    - [WriteOrdering](#solvio-WriteOrdering)
  
    - [Direction](#solvio-Direction)
    - [FieldType](#solvio-FieldType)
    - [Fusion](#solvio-Fusion)
    - [ReadConsistencyType](#solvio-ReadConsistencyType)
    - [RecommendStrategy](#solvio-RecommendStrategy)
    - [Sample](#solvio-Sample)
    - [UpdateStatus](#solvio-UpdateStatus)
    - [WriteOrderingType](#solvio-WriteOrderingType)
  
- [points_service.proto](#points_service-proto)
    - [Points](#solvio-Points)
  
- [solvio.proto](#solvio-proto)
    - [HealthCheckReply](#solvio-HealthCheckReply)
    - [HealthCheckRequest](#solvio-HealthCheckRequest)
  
    - [Solvio](#solvio-Solvio)
  
- [solvio_internal_service.proto](#solvio_internal_service-proto)
    - [GetConsensusCommitRequest](#solvio-GetConsensusCommitRequest)
    - [GetConsensusCommitResponse](#solvio-GetConsensusCommitResponse)
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



<a name="solvio-AbortShardTransfer"></a>

### AbortShardTransfer



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| to_shard_id | [uint32](#uint32) | optional |  |
| from_peer_id | [uint64](#uint64) |  |  |
| to_peer_id | [uint64](#uint64) |  |  |






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






<a name="solvio-BoolIndexParams"></a>

### BoolIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| on_disk | [bool](#bool) | optional | If true - store index on disk. |






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
| resharding_operations | [ReshardingInfo](#solvio-ReshardingInfo) | repeated | Resharding operations |






<a name="solvio-CollectionConfig"></a>

### CollectionConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [CollectionParams](#solvio-CollectionParams) |  | Collection parameters |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) |  | Configuration of vector index |
| optimizer_config | [OptimizersConfigDiff](#solvio-OptimizersConfigDiff) |  | Configuration of the optimizers |
| wal_config | [WalConfigDiff](#solvio-WalConfigDiff) |  | Configuration of the Write-Ahead-Log |
| quantization_config | [QuantizationConfig](#solvio-QuantizationConfig) | optional | Configuration of the vector quantization |
| strict_mode_config | [StrictModeConfig](#solvio-StrictModeConfig) | optional | Configuration of strict mode. |






<a name="solvio-CollectionDescription"></a>

### CollectionDescription



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name of the collection |






<a name="solvio-CollectionExists"></a>

### CollectionExists



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| exists | [bool](#bool) |  |  |






<a name="solvio-CollectionExistsRequest"></a>

### CollectionExistsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  |  |






<a name="solvio-CollectionExistsResponse"></a>

### CollectionExistsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [CollectionExists](#solvio-CollectionExists) |  |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-CollectionInfo"></a>

### CollectionInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [CollectionStatus](#solvio-CollectionStatus) |  | operating condition of the collection |
| optimizer_status | [OptimizerStatus](#solvio-OptimizerStatus) |  | status of collection optimizers |
| vectors_count | [uint64](#uint64) | optional | Approximate number of vectors in the collection |
| segments_count | [uint64](#uint64) |  | Number of independent segments |
| config | [CollectionConfig](#solvio-CollectionConfig) |  | Configuration |
| payload_schema | [CollectionInfo.PayloadSchemaEntry](#solvio-CollectionInfo-PayloadSchemaEntry) | repeated | Collection data types |
| points_count | [uint64](#uint64) | optional | Approximate number of points in the collection |
| indexed_vectors_count | [uint64](#uint64) | optional | Approximate number of indexed vectors in the collection. |






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
| sharding_method | [ShardingMethod](#solvio-ShardingMethod) | optional | Sharding method |
| sparse_vectors_config | [SparseVectorConfig](#solvio-SparseVectorConfig) | optional | Configuration for sparse vectors |






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
| sharding_method | [ShardingMethod](#solvio-ShardingMethod) | optional | Sharding method |
| sparse_vectors_config | [SparseVectorConfig](#solvio-SparseVectorConfig) | optional | Configuration for sparse vectors |
| strict_mode_config | [StrictModeConfig](#solvio-StrictModeConfig) | optional | Configuration for strict mode |






<a name="solvio-CreateShardKey"></a>

### CreateShardKey



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_key | [ShardKey](#solvio-ShardKey) |  | User-defined shard key |
| shards_number | [uint32](#uint32) | optional | Number of shards to create per shard key |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard to create |
| placement | [uint64](#uint64) | repeated | List of peer ids, allowed to create shards. If empty - all peers are allowed |






<a name="solvio-CreateShardKeyRequest"></a>

### CreateShardKeyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| request | [CreateShardKey](#solvio-CreateShardKey) |  | Request to create shard key |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="solvio-CreateShardKeyResponse"></a>

### CreateShardKeyResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [bool](#bool) |  |  |






<a name="solvio-DatetimeIndexParams"></a>

### DatetimeIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| on_disk | [bool](#bool) | optional | If true - store index on disk. |
| is_principal | [bool](#bool) | optional | If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests. |






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






<a name="solvio-DeleteShardKey"></a>

### DeleteShardKey



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_key | [ShardKey](#solvio-ShardKey) |  | Shard key to delete |






<a name="solvio-DeleteShardKeyRequest"></a>

### DeleteShardKeyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| request | [DeleteShardKey](#solvio-DeleteShardKey) |  | Request to delete shard key |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="solvio-DeleteShardKeyResponse"></a>

### DeleteShardKeyResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [bool](#bool) |  |  |






<a name="solvio-Disabled"></a>

### Disabled







<a name="solvio-FloatIndexParams"></a>

### FloatIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| on_disk | [bool](#bool) | optional | If true - store index on disk. |
| is_principal | [bool](#bool) | optional | If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests. |






<a name="solvio-GeoIndexParams"></a>

### GeoIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| on_disk | [bool](#bool) | optional | If true - store index on disk. |






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
| max_indexing_threads | [uint64](#uint64) | optional | Number of parallel threads used for background index building. If 0 - automatically select from 8 to 16. Best to keep between 8 and 16 to prevent likelihood of building broken/inefficient HNSW graphs. On small CPUs, less threads are used. |
| on_disk | [bool](#bool) | optional | Store HNSW index on disk. If set to false, the index will be stored in RAM. |
| payload_m | [uint64](#uint64) | optional | Number of additional payload-aware links per node in the index graph. If not set - regular M parameter will be used. |






<a name="solvio-IntegerIndexParams"></a>

### IntegerIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lookup | [bool](#bool) | optional | If true - support direct lookups. |
| range | [bool](#bool) | optional | If true - support ranges filters. |
| is_principal | [bool](#bool) | optional | If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests. |
| on_disk | [bool](#bool) | optional | If true - store index on disk. |






<a name="solvio-KeywordIndexParams"></a>

### KeywordIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_tenant | [bool](#bool) | optional | If true - used for tenant optimization. |
| on_disk | [bool](#bool) | optional | If true - store index on disk. |






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
| shard_key | [ShardKey](#solvio-ShardKey) | optional | User-defined shard key |






<a name="solvio-MaxOptimizationThreads"></a>

### MaxOptimizationThreads



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [uint64](#uint64) |  |  |
| setting | [MaxOptimizationThreads.Setting](#solvio-MaxOptimizationThreads-Setting) |  |  |






<a name="solvio-MoveShard"></a>

### MoveShard



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| to_shard_id | [uint32](#uint32) | optional |  |
| from_peer_id | [uint64](#uint64) |  |  |
| to_peer_id | [uint64](#uint64) |  |  |
| method | [ShardTransferMethod](#solvio-ShardTransferMethod) | optional |  |






<a name="solvio-MultiVectorConfig"></a>

### MultiVectorConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| comparator | [MultiVectorComparator](#solvio-MultiVectorComparator) |  | Comparator for multi-vector search |






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
| memmap_threshold | [uint64](#uint64) | optional | Maximum size (in kilobytes) of vectors to store in-memory per segment. Segments larger than this threshold will be stored as read-only memmapped file.

Memmap storage is disabled by default, to enable it, set this threshold to a reasonable value.

To disable memmap storage, set this to `0`.

Note: 1Kb = 1 vector of size 256 |
| indexing_threshold | [uint64](#uint64) | optional | Maximum size (in kilobytes) of vectors allowed for plain index, exceeding this threshold will enable vector indexing

Default value is 20,000, based on &lt;https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md&gt;.

To disable vector indexing, set to `0`.

Note: 1kB = 1 vector of size 256. |
| flush_interval_sec | [uint64](#uint64) | optional | Interval between forced flushes. |
| deprecated_max_optimization_threads | [uint64](#uint64) | optional | Deprecated in favor of `max_optimization_threads` |
| max_optimization_threads | [MaxOptimizationThreads](#solvio-MaxOptimizationThreads) | optional | Max number of threads (jobs) for running optimizations per shard. Note: each optimization job will also use `max_indexing_threads` threads by itself for index building. If &#34;auto&#34; - have no limit and choose dynamically to saturate CPU. If 0 - no optimization threads, optimizations will be disabled. |






<a name="solvio-PayloadIndexParams"></a>

### PayloadIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keyword_index_params | [KeywordIndexParams](#solvio-KeywordIndexParams) |  | Parameters for keyword index |
| integer_index_params | [IntegerIndexParams](#solvio-IntegerIndexParams) |  | Parameters for integer index |
| float_index_params | [FloatIndexParams](#solvio-FloatIndexParams) |  | Parameters for float index |
| geo_index_params | [GeoIndexParams](#solvio-GeoIndexParams) |  | Parameters for geo index |
| text_index_params | [TextIndexParams](#solvio-TextIndexParams) |  | Parameters for text index |
| bool_index_params | [BoolIndexParams](#solvio-BoolIndexParams) |  | Parameters for bool index |
| datetime_index_params | [DatetimeIndexParams](#solvio-DatetimeIndexParams) |  | Parameters for datetime index |
| uuid_index_params | [UuidIndexParams](#solvio-UuidIndexParams) |  | Parameters for uuid index |






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
| shard_key | [ShardKey](#solvio-ShardKey) | optional | User-defined shard key |






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






<a name="solvio-ReplicateShard"></a>

### ReplicateShard



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| to_shard_id | [uint32](#uint32) | optional |  |
| from_peer_id | [uint64](#uint64) |  |  |
| to_peer_id | [uint64](#uint64) |  |  |
| method | [ShardTransferMethod](#solvio-ShardTransferMethod) | optional |  |






<a name="solvio-ReshardingInfo"></a>

### ReshardingInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  |  |
| peer_id | [uint64](#uint64) |  |  |
| shard_key | [ShardKey](#solvio-ShardKey) | optional |  |
| direction | [ReshardingDirection](#solvio-ReshardingDirection) |  |  |






<a name="solvio-RestartTransfer"></a>

### RestartTransfer



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| to_shard_id | [uint32](#uint32) | optional |  |
| from_peer_id | [uint64](#uint64) |  |  |
| to_peer_id | [uint64](#uint64) |  |  |
| method | [ShardTransferMethod](#solvio-ShardTransferMethod) |  |  |






<a name="solvio-ScalarQuantization"></a>

### ScalarQuantization



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [QuantizationType](#solvio-QuantizationType) |  | Type of quantization |
| quantile | [float](#float) | optional | Number of bits to use for quantization |
| always_ram | [bool](#bool) | optional | If true - quantized vectors always will be stored in RAM, ignoring the config of main storage |






<a name="solvio-ShardKey"></a>

### ShardKey



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keyword | [string](#string) |  | String key |
| number | [uint64](#uint64) |  | Number key |






<a name="solvio-ShardTransferInfo"></a>

### ShardTransferInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_id | [uint32](#uint32) |  | Local shard id |
| to_shard_id | [uint32](#uint32) | optional |  |
| from | [uint64](#uint64) |  |  |
| to | [uint64](#uint64) |  |  |
| sync | [bool](#bool) |  | If `true` transfer is a synchronization of a replicas; If `false` transfer is a moving of a shard from one peer to another |






<a name="solvio-SparseIndexConfig"></a>

### SparseIndexConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| full_scan_threshold | [uint64](#uint64) | optional | Prefer a full scan search upto (excluding) this number of vectors. Note: this is number of vectors, not KiloBytes. |
| on_disk | [bool](#bool) | optional | Store inverted index on disk. If set to false, the index will be stored in RAM. |
| datatype | [Datatype](#solvio-Datatype) | optional | Datatype used to store weights in the index. |






<a name="solvio-SparseVectorConfig"></a>

### SparseVectorConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| map | [SparseVectorConfig.MapEntry](#solvio-SparseVectorConfig-MapEntry) | repeated |  |






<a name="solvio-SparseVectorConfig-MapEntry"></a>

### SparseVectorConfig.MapEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [SparseVectorParams](#solvio-SparseVectorParams) |  |  |






<a name="solvio-SparseVectorParams"></a>

### SparseVectorParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [SparseIndexConfig](#solvio-SparseIndexConfig) | optional | Configuration of sparse index |
| modifier | [Modifier](#solvio-Modifier) | optional | If set - apply modifier to the vector values |






<a name="solvio-StrictModeConfig"></a>

### StrictModeConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enabled | [bool](#bool) | optional |  |
| max_query_limit | [uint32](#uint32) | optional |  |
| max_timeout | [uint32](#uint32) | optional |  |
| unindexed_filtering_retrieve | [bool](#bool) | optional |  |
| unindexed_filtering_update | [bool](#bool) | optional |  |
| search_max_hnsw_ef | [uint32](#uint32) | optional |  |
| search_allow_exact | [bool](#bool) | optional |  |
| search_max_oversampling | [float](#float) | optional |  |
| upsert_max_batchsize | [uint64](#uint64) | optional |  |
| max_collection_vector_size_bytes | [uint64](#uint64) | optional |  |
| read_rate_limit | [uint32](#uint32) | optional | Max number of read operations per minute per replica |
| write_rate_limit | [uint32](#uint32) | optional | Max number of write operations per minute per replica |
| max_collection_payload_size_bytes | [uint64](#uint64) | optional |  |
| filter_max_conditions | [uint64](#uint64) | optional |  |
| condition_max_size | [uint64](#uint64) | optional |  |
| multivector_config | [StrictModeMultivectorConfig](#solvio-StrictModeMultivectorConfig) | optional |  |
| sparse_config | [StrictModeSparseConfig](#solvio-StrictModeSparseConfig) | optional |  |
| max_points_count | [uint64](#uint64) | optional |  |






<a name="solvio-StrictModeMultivector"></a>

### StrictModeMultivector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| max_vectors | [uint64](#uint64) | optional |  |






<a name="solvio-StrictModeMultivectorConfig"></a>

### StrictModeMultivectorConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multivector_config | [StrictModeMultivectorConfig.MultivectorConfigEntry](#solvio-StrictModeMultivectorConfig-MultivectorConfigEntry) | repeated |  |






<a name="solvio-StrictModeMultivectorConfig-MultivectorConfigEntry"></a>

### StrictModeMultivectorConfig.MultivectorConfigEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [StrictModeMultivector](#solvio-StrictModeMultivector) |  |  |






<a name="solvio-StrictModeSparse"></a>

### StrictModeSparse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| max_length | [uint64](#uint64) | optional |  |






<a name="solvio-StrictModeSparseConfig"></a>

### StrictModeSparseConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sparse_config | [StrictModeSparseConfig.SparseConfigEntry](#solvio-StrictModeSparseConfig-SparseConfigEntry) | repeated |  |






<a name="solvio-StrictModeSparseConfig-SparseConfigEntry"></a>

### StrictModeSparseConfig.SparseConfigEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [StrictModeSparse](#solvio-StrictModeSparse) |  |  |






<a name="solvio-TextIndexParams"></a>

### TextIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tokenizer | [TokenizerType](#solvio-TokenizerType) |  | Tokenizer type |
| lowercase | [bool](#bool) | optional | If true - all tokens will be lowercase |
| min_token_len | [uint64](#uint64) | optional | Minimal token length |
| max_token_len | [uint64](#uint64) | optional | Maximal token length |
| on_disk | [bool](#bool) | optional | If true - store index on disk. |






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
| sparse_vectors_config | [SparseVectorConfig](#solvio-SparseVectorConfig) | optional | New sparse vector parameters |
| strict_mode_config | [StrictModeConfig](#solvio-StrictModeConfig) | optional | New strict mode configuration |






<a name="solvio-UpdateCollectionClusterSetupRequest"></a>

### UpdateCollectionClusterSetupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| move_shard | [MoveShard](#solvio-MoveShard) |  |  |
| replicate_shard | [ReplicateShard](#solvio-ReplicateShard) |  |  |
| abort_transfer | [AbortShardTransfer](#solvio-AbortShardTransfer) |  |  |
| drop_replica | [Replica](#solvio-Replica) |  |  |
| create_shard_key | [CreateShardKey](#solvio-CreateShardKey) |  |  |
| delete_shard_key | [DeleteShardKey](#solvio-DeleteShardKey) |  |  |
| restart_transfer | [RestartTransfer](#solvio-RestartTransfer) |  |  |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="solvio-UpdateCollectionClusterSetupResponse"></a>

### UpdateCollectionClusterSetupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [bool](#bool) |  |  |






<a name="solvio-UuidIndexParams"></a>

### UuidIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_tenant | [bool](#bool) | optional | If true - used for tenant optimization. |
| on_disk | [bool](#bool) | optional | If true - store index on disk. |






<a name="solvio-VectorParams"></a>

### VectorParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| size | [uint64](#uint64) |  | Size of the vectors |
| distance | [Distance](#solvio-Distance) |  | Distance function used for comparing vectors |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) | optional | Configuration of vector HNSW graph. If omitted - the collection configuration will be used |
| quantization_config | [QuantizationConfig](#solvio-QuantizationConfig) | optional | Configuration of vector quantization config. If omitted - the collection configuration will be used |
| on_disk | [bool](#bool) | optional | If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM. |
| datatype | [Datatype](#solvio-Datatype) | optional | Data type of the vectors |
| multivector_config | [MultiVectorConfig](#solvio-MultiVectorConfig) | optional | Configuration for multi-vector search |






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
| Grey | 4 | Optimization is pending |



<a name="solvio-CompressionRatio"></a>

### CompressionRatio


| Name | Number | Description |
| ---- | ------ | ----------- |
| x4 | 0 |  |
| x8 | 1 |  |
| x16 | 2 |  |
| x32 | 3 |  |
| x64 | 4 |  |



<a name="solvio-Datatype"></a>

### Datatype


| Name | Number | Description |
| ---- | ------ | ----------- |
| Default | 0 |  |
| Float32 | 1 |  |
| Uint8 | 2 |  |
| Float16 | 3 |  |



<a name="solvio-Distance"></a>

### Distance


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownDistance | 0 |  |
| Cosine | 1 |  |
| Euclid | 2 |  |
| Dot | 3 |  |
| Manhattan | 4 |  |



<a name="solvio-MaxOptimizationThreads-Setting"></a>

### MaxOptimizationThreads.Setting


| Name | Number | Description |
| ---- | ------ | ----------- |
| Auto | 0 |  |



<a name="solvio-Modifier"></a>

### Modifier


| Name | Number | Description |
| ---- | ------ | ----------- |
| None | 0 |  |
| Idf | 1 | Apply Inverse Document Frequency |



<a name="solvio-MultiVectorComparator"></a>

### MultiVectorComparator


| Name | Number | Description |
| ---- | ------ | ----------- |
| MaxSim | 0 |  |



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
| Datetime | 7 |  |
| Uuid | 8 |  |



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
| PartialSnapshot | 5 | Deprecated: snapshot shard transfer is in progress; Updates should not be sent to (and are ignored by) the shard |
| Recovery | 6 | Shard is undergoing recovered by an external node; Normally rejects updates, accepts updates if force is true |
| Resharding | 7 | Points are being migrated to this shard as part of scale-up resharding |
| ReshardingScaleDown | 8 | Points are being migrated to this shard as part of scale-down resharding |



<a name="solvio-ReshardingDirection"></a>

### ReshardingDirection
Resharding direction, scale up or down in number of shards

| Name | Number | Description |
| ---- | ------ | ----------- |
| Up | 0 | Scale up, add a new shard |
| Down | 1 | Scale down, remove a shard |



<a name="solvio-ShardTransferMethod"></a>

### ShardTransferMethod


| Name | Number | Description |
| ---- | ------ | ----------- |
| StreamRecords | 0 | Stream shard records in batches |
| Snapshot | 1 | Snapshot the shard and recover it on the target peer |
| WalDelta | 2 | Resolve WAL delta between peers and transfer the difference |
| ReshardingStreamRecords | 3 | Stream shard records in batches for resharding |



<a name="solvio-ShardingMethod"></a>

### ShardingMethod


| Name | Number | Description |
| ---- | ------ | ----------- |
| Auto | 0 | Auto-sharding based on record ids |
| Custom | 1 | Shard by user-defined key |



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
| CollectionExists | [CollectionExistsRequest](#solvio-CollectionExistsRequest) | [CollectionExistsResponse](#solvio-CollectionExistsResponse) | Check the existence of a collection |
| UpdateCollectionClusterSetup | [UpdateCollectionClusterSetupRequest](#solvio-UpdateCollectionClusterSetupRequest) | [UpdateCollectionClusterSetupResponse](#solvio-UpdateCollectionClusterSetupResponse) | Update cluster setup for a collection |
| CreateShardKey | [CreateShardKeyRequest](#solvio-CreateShardKeyRequest) | [CreateShardKeyResponse](#solvio-CreateShardKeyResponse) | Create shard key |
| DeleteShardKey | [DeleteShardKeyRequest](#solvio-DeleteShardKeyRequest) | [DeleteShardKeyResponse](#solvio-DeleteShardKeyResponse) | Delete shard key |

 



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
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






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
| has_vector | [HasVectorCondition](#solvio-HasVectorCondition) |  |  |






<a name="solvio-ContextExamplePair"></a>

### ContextExamplePair



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| positive | [VectorExample](#solvio-VectorExample) |  |  |
| negative | [VectorExample](#solvio-VectorExample) |  |  |






<a name="solvio-ContextInput"></a>

### ContextInput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pairs | [ContextInputPair](#solvio-ContextInputPair) | repeated | Search space will be constrained by these pairs of vectors |






<a name="solvio-ContextInputPair"></a>

### ContextInputPair



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| positive | [VectorInput](#solvio-VectorInput) |  | A positive vector |
| negative | [VectorInput](#solvio-VectorInput) |  | Repel from this vector |






<a name="solvio-CountPoints"></a>

### CountPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| exact | [bool](#bool) | optional | If `true` - return exact count, if `false` - return approximate count |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |






<a name="solvio-CountResponse"></a>

### CountResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [CountResult](#solvio-CountResult) |  |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






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






<a name="solvio-DatetimeRange"></a>

### DatetimeRange



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lt | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional |  |
| gt | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional |  |
| gte | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional |  |
| lte | [google.protobuf.Timestamp](#google-protobuf-Timestamp) | optional |  |






<a name="solvio-DecayParamsExpression"></a>

### DecayParamsExpression



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| x | [Expression](#solvio-Expression) |  | The variable to decay |
| target | [Expression](#solvio-Expression) | optional | The target value to start decaying from. Defaults to 0. |
| scale | [float](#float) | optional | The scale factor of the decay, in terms of `x`. Defaults to 1.0. Must be a non-zero positive number. |
| midpoint | [float](#float) | optional | The midpoint of the decay. Defaults to 0.5. Output will be this value when `|x - target| == scale`. |






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
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-DeletePointVectors"></a>

### DeletePointVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points_selector | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| vectors | [VectorsSelector](#solvio-VectorsSelector) |  | List of vector names to delete |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-DeletePoints"></a>

### DeletePoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-DenseVector"></a>

### DenseVector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [float](#float) | repeated |  |






<a name="solvio-DiscoverBatchPoints"></a>

### DiscoverBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| discover_points | [DiscoverPoints](#solvio-DiscoverPoints) | repeated |  |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |






<a name="solvio-DiscoverBatchResponse"></a>

### DiscoverBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#solvio-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-DiscoverInput"></a>

### DiscoverInput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| target | [VectorInput](#solvio-VectorInput) |  | Use this as the primary search objective |
| context | [ContextInput](#solvio-ContextInput) |  | Search space will be constrained by these pairs of vectors |






<a name="solvio-DiscoverPoints"></a>

### DiscoverPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| target | [TargetVector](#solvio-TargetVector) |  | Use this as the primary search objective |
| context | [ContextExamplePair](#solvio-ContextExamplePair) | repeated | Search will be constrained by these pairs of examples |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint64](#uint64) |  | Max number of result |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#solvio-SearchParams) |  | Search config |
| offset | [uint64](#uint64) | optional | Offset of the result |
| using | [string](#string) | optional | Define which vector to use for recommendation, if not specified - default vector |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| lookup_from | [LookupLocation](#solvio-LookupLocation) | optional | Name of the collection to use for points lookup, if not specified - use current collection |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |






<a name="solvio-DiscoverResponse"></a>

### DiscoverResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#solvio-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-DivExpression"></a>

### DivExpression



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| left | [Expression](#solvio-Expression) |  |  |
| right | [Expression](#solvio-Expression) |  |  |
| by_zero_default | [float](#float) | optional |  |






<a name="solvio-Document"></a>

### Document



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  | Text of the document |
| model | [string](#string) |  | Model name |
| options | [Document.OptionsEntry](#solvio-Document-OptionsEntry) | repeated | Model options |






<a name="solvio-Document-OptionsEntry"></a>

### Document.OptionsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-Expression"></a>

### Expression



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| constant | [float](#float) |  |  |
| variable | [string](#string) |  | Payload key or reference to score. |
| condition | [Condition](#solvio-Condition) |  | Payload condition. If true, becomes 1.0; otherwise 0.0 |
| geo_distance | [GeoDistance](#solvio-GeoDistance) |  | Geographic distance in meters |
| datetime | [string](#string) |  | Date-time constant |
| datetime_key | [string](#string) |  | Payload key with date-time values |
| mult | [MultExpression](#solvio-MultExpression) |  | Multiply |
| sum | [SumExpression](#solvio-SumExpression) |  | Sum |
| div | [DivExpression](#solvio-DivExpression) |  | Divide |
| neg | [Expression](#solvio-Expression) |  | Negate |
| abs | [Expression](#solvio-Expression) |  | Absolute value |
| sqrt | [Expression](#solvio-Expression) |  | Square root |
| pow | [PowExpression](#solvio-PowExpression) |  | Power |
| exp | [Expression](#solvio-Expression) |  | Exponential |
| log10 | [Expression](#solvio-Expression) |  | Logarithm |
| ln | [Expression](#solvio-Expression) |  | Natural logarithm |
| exp_decay | [DecayParamsExpression](#solvio-DecayParamsExpression) |  | Exponential decay |
| gauss_decay | [DecayParamsExpression](#solvio-DecayParamsExpression) |  | Gaussian decay |
| lin_decay | [DecayParamsExpression](#solvio-DecayParamsExpression) |  | Linear decay |






<a name="solvio-FacetCounts"></a>

### FacetCounts



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| key | [string](#string) |  | Payload key of the facet |
| filter | [Filter](#solvio-Filter) | optional | Filter conditions - return only those points that satisfy the specified conditions. |
| limit | [uint64](#uint64) | optional | Max number of facets. Default is 10. |
| exact | [bool](#bool) | optional | If true, return exact counts, slower but useful for debugging purposes. Default is false. |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |






<a name="solvio-FacetHit"></a>

### FacetHit



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| value | [FacetValue](#solvio-FacetValue) |  | Value from the facet |
| count | [uint64](#uint64) |  | Number of points with this value |






<a name="solvio-FacetResponse"></a>

### FacetResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hits | [FacetHit](#solvio-FacetHit) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-FacetValue"></a>

### FacetValue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| string_value | [string](#string) |  | String value from the facet |
| integer_value | [int64](#int64) |  | Integer value from the facet |
| bool_value | [bool](#bool) |  | Boolean value from the facet |






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
| datetime_range | [DatetimeRange](#solvio-DatetimeRange) |  | Check if datetime is within a given range |
| is_empty | [bool](#bool) | optional | Check if field is empty |
| is_null | [bool](#bool) | optional | Check if field is null |






<a name="solvio-Filter"></a>

### Filter



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| should | [Condition](#solvio-Condition) | repeated | At least one of those conditions should match |
| must | [Condition](#solvio-Condition) | repeated | All conditions must match |
| must_not | [Condition](#solvio-Condition) | repeated | All conditions must NOT match |
| min_should | [MinShould](#solvio-MinShould) | optional | At least minimum amount of given conditions should match |






<a name="solvio-Formula"></a>

### Formula



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| expression | [Expression](#solvio-Expression) |  |  |
| defaults | [Formula.DefaultsEntry](#solvio-Formula-DefaultsEntry) | repeated |  |






<a name="solvio-Formula-DefaultsEntry"></a>

### Formula.DefaultsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-GeoBoundingBox"></a>

### GeoBoundingBox



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| top_left | [GeoPoint](#solvio-GeoPoint) |  | north-west corner |
| bottom_right | [GeoPoint](#solvio-GeoPoint) |  | south-east corner |






<a name="solvio-GeoDistance"></a>

### GeoDistance



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| origin | [GeoPoint](#solvio-GeoPoint) |  |  |
| to | [string](#string) |  |  |






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
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |






<a name="solvio-GetResponse"></a>

### GetResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [RetrievedPoint](#solvio-RetrievedPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






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






<a name="solvio-HardwareUsage"></a>

### HardwareUsage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cpu | [uint64](#uint64) |  |  |
| payload_io_read | [uint64](#uint64) |  |  |
| payload_io_write | [uint64](#uint64) |  |  |
| payload_index_io_read | [uint64](#uint64) |  |  |
| payload_index_io_write | [uint64](#uint64) |  |  |
| vector_io_read | [uint64](#uint64) |  |  |
| vector_io_write | [uint64](#uint64) |  |  |






<a name="solvio-HasIdCondition"></a>

### HasIdCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| has_id | [PointId](#solvio-PointId) | repeated |  |






<a name="solvio-HasVectorCondition"></a>

### HasVectorCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| has_vector | [string](#string) |  |  |






<a name="solvio-Image"></a>

### Image



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| image | [Value](#solvio-Value) |  | Image data, either base64 encoded or URL |
| model | [string](#string) |  | Model name |
| options | [Image.OptionsEntry](#solvio-Image-OptionsEntry) | repeated | Model options |






<a name="solvio-Image-OptionsEntry"></a>

### Image.OptionsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-InferenceObject"></a>

### InferenceObject



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| object | [Value](#solvio-Value) |  | Object to infer |
| model | [string](#string) |  | Model name |
| options | [InferenceObject.OptionsEntry](#solvio-InferenceObject-OptionsEntry) | repeated | Model options |






<a name="solvio-InferenceObject-OptionsEntry"></a>

### InferenceObject.OptionsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






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
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |






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






<a name="solvio-MinShould"></a>

### MinShould



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| conditions | [Condition](#solvio-Condition) | repeated |  |
| min_count | [uint64](#uint64) |  |  |






<a name="solvio-MultExpression"></a>

### MultExpression



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| mult | [Expression](#solvio-Expression) | repeated |  |






<a name="solvio-MultiDenseVector"></a>

### MultiDenseVector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vectors | [DenseVector](#solvio-DenseVector) | repeated |  |






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






<a name="solvio-NamedVectorsOutput"></a>

### NamedVectorsOutput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vectors | [NamedVectorsOutput.VectorsEntry](#solvio-NamedVectorsOutput-VectorsEntry) | repeated |  |






<a name="solvio-NamedVectorsOutput-VectorsEntry"></a>

### NamedVectorsOutput.VectorsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [VectorOutput](#solvio-VectorOutput) |  |  |






<a name="solvio-NestedCondition"></a>

### NestedCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  | Path to nested object |
| filter | [Filter](#solvio-Filter) |  | Filter condition |






<a name="solvio-OrderBy"></a>

### OrderBy



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  | Payload key to order by |
| direction | [Direction](#solvio-Direction) | optional | Ascending or descending order |
| start_from | [StartFrom](#solvio-StartFrom) | optional | Start from this value |






<a name="solvio-OrderValue"></a>

### OrderValue



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| int | [int64](#int64) |  |  |
| float | [double](#double) |  |  |






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
| usage | [Usage](#solvio-Usage) | optional |  |






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
| delete_deprecated | [PointsSelector](#solvio-PointsSelector) |  | **Deprecated.**  |
| set_payload | [PointsUpdateOperation.SetPayload](#solvio-PointsUpdateOperation-SetPayload) |  |  |
| overwrite_payload | [PointsUpdateOperation.OverwritePayload](#solvio-PointsUpdateOperation-OverwritePayload) |  |  |
| delete_payload | [PointsUpdateOperation.DeletePayload](#solvio-PointsUpdateOperation-DeletePayload) |  |  |
| clear_payload_deprecated | [PointsSelector](#solvio-PointsSelector) |  | **Deprecated.**  |
| update_vectors | [PointsUpdateOperation.UpdateVectors](#solvio-PointsUpdateOperation-UpdateVectors) |  |  |
| delete_vectors | [PointsUpdateOperation.DeleteVectors](#solvio-PointsUpdateOperation-DeleteVectors) |  |  |
| delete_points | [PointsUpdateOperation.DeletePoints](#solvio-PointsUpdateOperation-DeletePoints) |  |  |
| clear_payload | [PointsUpdateOperation.ClearPayload](#solvio-PointsUpdateOperation-ClearPayload) |  |  |






<a name="solvio-PointsUpdateOperation-ClearPayload"></a>

### PointsUpdateOperation.ClearPayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-PointsUpdateOperation-DeletePayload"></a>

### PointsUpdateOperation.DeletePayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keys | [string](#string) | repeated |  |
| points_selector | [PointsSelector](#solvio-PointsSelector) | optional | Affected points |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-PointsUpdateOperation-DeletePoints"></a>

### PointsUpdateOperation.DeletePoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-PointsUpdateOperation-DeleteVectors"></a>

### PointsUpdateOperation.DeleteVectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points_selector | [PointsSelector](#solvio-PointsSelector) |  | Affected points |
| vectors | [VectorsSelector](#solvio-VectorsSelector) |  | List of vector names to delete |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-PointsUpdateOperation-OverwritePayload"></a>

### PointsUpdateOperation.OverwritePayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payload | [PointsUpdateOperation.OverwritePayload.PayloadEntry](#solvio-PointsUpdateOperation-OverwritePayload-PayloadEntry) | repeated |  |
| points_selector | [PointsSelector](#solvio-PointsSelector) | optional | Affected points |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |
| key | [string](#string) | optional | Option for indicate property of payload |






<a name="solvio-PointsUpdateOperation-OverwritePayload-PayloadEntry"></a>

### PointsUpdateOperation.OverwritePayload.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-PointsUpdateOperation-PointStructList"></a>

### PointsUpdateOperation.PointStructList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| points | [PointStruct](#solvio-PointStruct) | repeated |  |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-PointsUpdateOperation-SetPayload"></a>

### PointsUpdateOperation.SetPayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payload | [PointsUpdateOperation.SetPayload.PayloadEntry](#solvio-PointsUpdateOperation-SetPayload-PayloadEntry) | repeated |  |
| points_selector | [PointsSelector](#solvio-PointsSelector) | optional | Affected points |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |
| key | [string](#string) | optional | Option for indicate property of payload |






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
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-PowExpression"></a>

### PowExpression



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| base | [Expression](#solvio-Expression) |  |  |
| exponent | [Expression](#solvio-Expression) |  |  |






<a name="solvio-PrefetchQuery"></a>

### PrefetchQuery



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| prefetch | [PrefetchQuery](#solvio-PrefetchQuery) | repeated | Sub-requests to perform first. If present, the query will be performed on the results of the prefetches. |
| query | [Query](#solvio-Query) | optional | Query to perform. If missing, returns points ordered by their IDs. |
| using | [string](#string) | optional | Define which vector to use for querying. If missing, the default vector is is used. |
| filter | [Filter](#solvio-Filter) | optional | Filter conditions - return only those points that satisfy the specified conditions. |
| params | [SearchParams](#solvio-SearchParams) | optional | Search params for when there is no prefetch. |
| score_threshold | [float](#float) | optional | Return points with scores better than this threshold. |
| limit | [uint64](#uint64) | optional | Max number of points. Default is 10 |
| lookup_from | [LookupLocation](#solvio-LookupLocation) | optional | The location to use for IDs lookup, if not specified - use the current collection and the &#39;using&#39; vector |






<a name="solvio-QuantizationSearchParams"></a>

### QuantizationSearchParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ignore | [bool](#bool) | optional | If set to true, search will ignore quantized vector data |
| rescore | [bool](#bool) | optional | If true, use original vectors to re-score top-k results. If ignored, solvio decides automatically does rescore enabled or not. |
| oversampling | [double](#double) | optional | Oversampling factor for quantization.

Defines how many extra vectors should be pre-selected using quantized index, and then re-scored using original vectors.

For example, if `oversampling` is 2.4 and `limit` is 100, then 240 vectors will be pre-selected using quantized index, and then top-100 will be returned after re-scoring. |






<a name="solvio-Query"></a>

### Query



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| nearest | [VectorInput](#solvio-VectorInput) |  | Find the nearest neighbors to this vector. |
| recommend | [RecommendInput](#solvio-RecommendInput) |  | Use multiple positive and negative vectors to find the results. |
| discover | [DiscoverInput](#solvio-DiscoverInput) |  | Search for nearest points, but constrain the search space with context |
| context | [ContextInput](#solvio-ContextInput) |  | Return points that live in positive areas. |
| order_by | [OrderBy](#solvio-OrderBy) |  | Order the points by a payload field. |
| fusion | [Fusion](#solvio-Fusion) |  | Fuse the results of multiple prefetches. |
| sample | [Sample](#solvio-Sample) |  | Sample points from the collection. |
| formula | [Formula](#solvio-Formula) |  | Score boosting via an arbitrary formula |






<a name="solvio-QueryBatchPoints"></a>

### QueryBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  |  |
| query_points | [QueryPoints](#solvio-QueryPoints) | repeated |  |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |






<a name="solvio-QueryBatchResponse"></a>

### QueryBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#solvio-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-QueryGroupsResponse"></a>

### QueryGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [GroupsResult](#solvio-GroupsResult) |  |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-QueryPointGroups"></a>

### QueryPointGroups



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| prefetch | [PrefetchQuery](#solvio-PrefetchQuery) | repeated | Sub-requests to perform first. If present, the query will be performed on the results of the prefetches. |
| query | [Query](#solvio-Query) | optional | Query to perform. If missing, returns points ordered by their IDs. |
| using | [string](#string) | optional | Define which vector to use for querying. If missing, the default vector is used. |
| filter | [Filter](#solvio-Filter) | optional | Filter conditions - return only those points that satisfy the specified conditions. |
| params | [SearchParams](#solvio-SearchParams) | optional | Search params for when there is no prefetch. |
| score_threshold | [float](#float) | optional | Return points with scores better than this threshold. |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |
| lookup_from | [LookupLocation](#solvio-LookupLocation) | optional | The location to use for IDs lookup, if not specified - use the current collection and the &#39;using&#39; vector |
| limit | [uint64](#uint64) | optional | Max number of points. Default is 3. |
| group_size | [uint64](#uint64) | optional | Maximum amount of points to return per group. Default to 10. |
| group_by | [string](#string) |  | Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups. |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| with_lookup | [WithLookup](#solvio-WithLookup) | optional | Options for specifying how to use the group id to lookup points in another collection |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |






<a name="solvio-QueryPoints"></a>

### QueryPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| prefetch | [PrefetchQuery](#solvio-PrefetchQuery) | repeated | Sub-requests to perform first. If present, the query will be performed on the results of the prefetches. |
| query | [Query](#solvio-Query) | optional | Query to perform. If missing, returns points ordered by their IDs. |
| using | [string](#string) | optional | Define which vector to use for querying. If missing, the default vector is used. |
| filter | [Filter](#solvio-Filter) | optional | Filter conditions - return only those points that satisfy the specified conditions. |
| params | [SearchParams](#solvio-SearchParams) | optional | Search params for when there is no prefetch. |
| score_threshold | [float](#float) | optional | Return points with scores better than this threshold. |
| limit | [uint64](#uint64) | optional | Max number of points. Default is 10. |
| offset | [uint64](#uint64) | optional | Offset of the result. Skip this many points. Default is 0. |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into the response. |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) | optional | Options for specifying which payload to include or not. |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees. |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards. |
| lookup_from | [LookupLocation](#solvio-LookupLocation) | optional | The location to use for IDs lookup, if not specified - use the current collection and the &#39;using&#39; vector |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |






<a name="solvio-QueryResponse"></a>

### QueryResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#solvio-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






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
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |






<a name="solvio-RecommendBatchResponse"></a>

### RecommendBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#solvio-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-RecommendGroupsResponse"></a>

### RecommendGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [GroupsResult](#solvio-GroupsResult) |  |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-RecommendInput"></a>

### RecommendInput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| positive | [VectorInput](#solvio-VectorInput) | repeated | Look for vectors closest to the vectors from these points |
| negative | [VectorInput](#solvio-VectorInput) | repeated | Try to avoid vectors like the vector from these points |
| strategy | [RecommendStrategy](#solvio-RecommendStrategy) | optional | How to use the provided vectors to find the results |






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
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |






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
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |






<a name="solvio-RecommendResponse"></a>

### RecommendResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#solvio-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






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
| vectors | [VectorsOutput](#solvio-VectorsOutput) | optional |  |
| shard_key | [ShardKey](#solvio-ShardKey) | optional | Shard key |
| order_value | [OrderValue](#solvio-OrderValue) | optional | Order-by value |






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
| vectors | [VectorsOutput](#solvio-VectorsOutput) | optional | Vectors to search |
| shard_key | [ShardKey](#solvio-ShardKey) | optional | Shard key |
| order_value | [OrderValue](#solvio-OrderValue) | optional | Order by value |






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
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |
| order_by | [OrderBy](#solvio-OrderBy) | optional | Order the records by a payload field |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |






<a name="solvio-ScrollResponse"></a>

### ScrollResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| next_page_offset | [PointId](#solvio-PointId) | optional | Use this offset for the next query |
| result | [RetrievedPoint](#solvio-RetrievedPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-SearchBatchPoints"></a>

### SearchBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| search_points | [SearchPoints](#solvio-SearchPoints) | repeated |  |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |






<a name="solvio-SearchBatchResponse"></a>

### SearchBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#solvio-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-SearchGroupsResponse"></a>

### SearchGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [GroupsResult](#solvio-GroupsResult) |  |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-SearchMatrixOffsets"></a>

### SearchMatrixOffsets



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| offsets_row | [uint64](#uint64) | repeated | Row indices of the matrix |
| offsets_col | [uint64](#uint64) | repeated | Column indices of the matrix |
| scores | [float](#float) | repeated | Scores associated with matrix coordinates |
| ids | [PointId](#solvio-PointId) | repeated | Ids of the points in order |






<a name="solvio-SearchMatrixOffsetsResponse"></a>

### SearchMatrixOffsetsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [SearchMatrixOffsets](#solvio-SearchMatrixOffsets) |  |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-SearchMatrixPair"></a>

### SearchMatrixPair



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| a | [PointId](#solvio-PointId) |  | first id of the pair |
| b | [PointId](#solvio-PointId) |  | second id of the pair |
| score | [float](#float) |  | score of the pair |






<a name="solvio-SearchMatrixPairs"></a>

### SearchMatrixPairs



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pairs | [SearchMatrixPair](#solvio-SearchMatrixPair) | repeated | List of pairs of points with scores |






<a name="solvio-SearchMatrixPairsResponse"></a>

### SearchMatrixPairsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [SearchMatrixPairs](#solvio-SearchMatrixPairs) |  |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-SearchMatrixPoints"></a>

### SearchMatrixPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| filter | [Filter](#solvio-Filter) | optional | Filter conditions - return only those points that satisfy the specified conditions. |
| sample | [uint64](#uint64) | optional | How many points to select and search within. Default is 10. |
| limit | [uint64](#uint64) | optional | How many neighbours per sample to find. Default is 3. |
| using | [string](#string) | optional | Define which vector to use for querying. If missing, the default vector is is used. |
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |
| read_consistency | [ReadConsistency](#solvio-ReadConsistency) | optional | Options for specifying read consistency guarantees |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |






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
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |
| sparse_indices | [SparseIndices](#solvio-SparseIndices) | optional |  |






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
| timeout | [uint64](#uint64) | optional | If set, overrides global timeout setting for this request. Unit is seconds. |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Specify in which shards to look for the points, if not specified - look in all shards |
| sparse_indices | [SparseIndices](#solvio-SparseIndices) | optional |  |






<a name="solvio-SearchResponse"></a>

### SearchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#solvio-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |
| usage | [Usage](#solvio-Usage) | optional |  |






<a name="solvio-SetPayloadPoints"></a>

### SetPayloadPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| payload | [SetPayloadPoints.PayloadEntry](#solvio-SetPayloadPoints-PayloadEntry) | repeated | New payload values |
| points_selector | [PointsSelector](#solvio-PointsSelector) | optional | Affected points |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |
| key | [string](#string) | optional | Option for indicate property of payload |






<a name="solvio-SetPayloadPoints-PayloadEntry"></a>

### SetPayloadPoints.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






<a name="solvio-ShardKeySelector"></a>

### ShardKeySelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| shard_keys | [ShardKey](#solvio-ShardKey) | repeated | List of shard keys which should be used in the request |






<a name="solvio-SparseIndices"></a>

### SparseIndices



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [uint32](#uint32) | repeated |  |






<a name="solvio-SparseVector"></a>

### SparseVector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [float](#float) | repeated |  |
| indices | [uint32](#uint32) | repeated |  |






<a name="solvio-StartFrom"></a>

### StartFrom



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| float | [double](#double) |  |  |
| integer | [int64](#int64) |  |  |
| timestamp | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| datetime | [string](#string) |  |  |






<a name="solvio-SumExpression"></a>

### SumExpression



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sum | [Expression](#solvio-Expression) | repeated |  |






<a name="solvio-TargetVector"></a>

### TargetVector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| single | [VectorExample](#solvio-VectorExample) |  |  |






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
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-UpdateResult"></a>

### UpdateResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| operation_id | [uint64](#uint64) | optional | Number of operation |
| status | [UpdateStatus](#solvio-UpdateStatus) |  | Operation status |






<a name="solvio-UpsertPoints"></a>

### UpsertPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointStruct](#solvio-PointStruct) | repeated |  |
| ordering | [WriteOrdering](#solvio-WriteOrdering) | optional | Write ordering guarantees |
| shard_key_selector | [ShardKeySelector](#solvio-ShardKeySelector) | optional | Option for custom sharding to specify used shard keys |






<a name="solvio-Usage"></a>

### Usage
---------------------------------------------
----------- Measurements collector ----------
---------------------------------------------


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hardware | [HardwareUsage](#solvio-HardwareUsage) | optional |  |






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
Legacy vector format, which determines the vector type by the configuration of its fields.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [float](#float) | repeated | Vector data (flatten for multi vectors), deprecated |
| indices | [SparseIndices](#solvio-SparseIndices) | optional | Sparse indices for sparse vectors, deprecated |
| vectors_count | [uint32](#uint32) | optional | Number of vectors per multi vector, deprecated |
| dense | [DenseVector](#solvio-DenseVector) |  | Dense vector |
| sparse | [SparseVector](#solvio-SparseVector) |  | Sparse vector |
| multi_dense | [MultiDenseVector](#solvio-MultiDenseVector) |  | Multi dense vector |
| document | [Document](#solvio-Document) |  |  |
| image | [Image](#solvio-Image) |  |  |
| object | [InferenceObject](#solvio-InferenceObject) |  |  |






<a name="solvio-VectorExample"></a>

### VectorExample



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#solvio-PointId) |  |  |
| vector | [Vector](#solvio-Vector) |  |  |






<a name="solvio-VectorInput"></a>

### VectorInput
Vector type to be used in queries. Ids will be substituted with their corresponding vectors from the collection.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#solvio-PointId) |  |  |
| dense | [DenseVector](#solvio-DenseVector) |  |  |
| sparse | [SparseVector](#solvio-SparseVector) |  |  |
| multi_dense | [MultiDenseVector](#solvio-MultiDenseVector) |  |  |
| document | [Document](#solvio-Document) |  |  |
| image | [Image](#solvio-Image) |  |  |
| object | [InferenceObject](#solvio-InferenceObject) |  |  |






<a name="solvio-VectorOutput"></a>

### VectorOutput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [float](#float) | repeated | Vector data (flatten for multi vectors), deprecated |
| indices | [SparseIndices](#solvio-SparseIndices) | optional | Sparse indices for sparse vectors, deprecated |
| vectors_count | [uint32](#uint32) | optional | Number of vectors per multi vector, deprecated |
| dense | [DenseVector](#solvio-DenseVector) |  | Dense vector |
| sparse | [SparseVector](#solvio-SparseVector) |  | Sparse vector |
| multi_dense | [MultiDenseVector](#solvio-MultiDenseVector) |  | Multi dense vector |






<a name="solvio-Vectors"></a>

### Vectors



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vector | [Vector](#solvio-Vector) |  |  |
| vectors | [NamedVectors](#solvio-NamedVectors) |  |  |






<a name="solvio-VectorsOutput"></a>

### VectorsOutput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vector | [VectorOutput](#solvio-VectorOutput) |  |  |
| vectors | [NamedVectorsOutput](#solvio-NamedVectorsOutput) |  |  |






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





 


<a name="solvio-Direction"></a>

### Direction


| Name | Number | Description |
| ---- | ------ | ----------- |
| Asc | 0 |  |
| Desc | 1 |  |



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
| FieldTypeDatetime | 6 |  |
| FieldTypeUuid | 7 |  |



<a name="solvio-Fusion"></a>

### Fusion


| Name | Number | Description |
| ---- | ------ | ----------- |
| RRF | 0 | Reciprocal Rank Fusion |
| DBSF | 1 | Distribution-Based Score Fusion |



<a name="solvio-ReadConsistencyType"></a>

### ReadConsistencyType


| Name | Number | Description |
| ---- | ------ | ----------- |
| All | 0 | Send request to all nodes and return points which are present on all of them |
| Majority | 1 | Send requests to all nodes and return points which are present on majority of them |
| Quorum | 2 | Send requests to half &#43; 1 nodes, return points which are present on all of them |



<a name="solvio-RecommendStrategy"></a>

### RecommendStrategy
How to use positive and negative vectors to find the results, default is `AverageVector`.

| Name | Number | Description |
| ---- | ------ | ----------- |
| AverageVector | 0 | Average positive and negative vectors and create a single query with the formula `query = avg_pos &#43; avg_pos - avg_neg`. Then performs normal search. |
| BestScore | 1 | Uses custom search objective. Each candidate is compared against all examples, its score is then chosen from the `max(max_pos_score, max_neg_score)`. If the `max_neg_score` is chosen then it is squared and negated. |
| SumScores | 2 | Uses custom search objective. Compares against all inputs, sums all the scores. Scores against positive vectors are added, against negatives are subtracted. |



<a name="solvio-Sample"></a>

### Sample
Sample points from the collection

Available sampling methods:

* `random` - Random sampling

| Name | Number | Description |
| ---- | ------ | ----------- |
| Random | 0 |  |



<a name="solvio-UpdateStatus"></a>

### UpdateStatus


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownUpdateStatus | 0 |  |
| Acknowledged | 1 | Update is received, but not processed yet |
| Completed | 2 | Update is applied and ready for search |
| ClockRejected | 3 | Internal: update is rejected due to an outdated clock |



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
| Discover | [DiscoverPoints](#solvio-DiscoverPoints) | [DiscoverResponse](#solvio-DiscoverResponse) | Use context and a target to find the most similar points to the target, constrained by the context.

When using only the context (without a target), a special search - called context search - is performed where pairs of points are used to generate a loss that guides the search towards the zone where most positive examples overlap. This means that the score minimizes the scenario of finding a point closer to a negative than to a positive part of a pair.

Since the score of a context relates to loss, the maximum score a point can get is 0.0, and it becomes normal that many points can have a score of 0.0.

When using target (with or without context), the score behaves a little different: The integer part of the score represents the rank with respect to the context, while the decimal part of the score relates to the distance to the target. The context part of the score for each pair is calculated &#43;1 if the point is closer to a positive than to a negative part of a pair, and -1 otherwise. |
| DiscoverBatch | [DiscoverBatchPoints](#solvio-DiscoverBatchPoints) | [DiscoverBatchResponse](#solvio-DiscoverBatchResponse) | Batch request points based on { positive, negative } pairs of examples, and/or a target |
| Count | [CountPoints](#solvio-CountPoints) | [CountResponse](#solvio-CountResponse) | Count points in collection with given filtering conditions |
| UpdateBatch | [UpdateBatchPoints](#solvio-UpdateBatchPoints) | [UpdateBatchResponse](#solvio-UpdateBatchResponse) | Perform multiple update operations in one request |
| Query | [QueryPoints](#solvio-QueryPoints) | [QueryResponse](#solvio-QueryResponse) | Universally query points. This endpoint covers all capabilities of search, recommend, discover, filters. But also enables hybrid and multi-stage queries. |
| QueryBatch | [QueryBatchPoints](#solvio-QueryBatchPoints) | [QueryBatchResponse](#solvio-QueryBatchResponse) | Universally query points in a batch fashion. This endpoint covers all capabilities of search, recommend, discover, filters. But also enables hybrid and multi-stage queries. |
| QueryGroups | [QueryPointGroups](#solvio-QueryPointGroups) | [QueryGroupsResponse](#solvio-QueryGroupsResponse) | Universally query points in a group fashion. This endpoint covers all capabilities of search, recommend, discover, filters. But also enables hybrid and multi-stage queries. |
| Facet | [FacetCounts](#solvio-FacetCounts) | [FacetResponse](#solvio-FacetResponse) | Perform facet counts. For each value in the field, count the number of points that have this value and match the conditions. |
| SearchMatrixPairs | [SearchMatrixPoints](#solvio-SearchMatrixPoints) | [SearchMatrixPairsResponse](#solvio-SearchMatrixPairsResponse) | Compute distance matrix for sampled points with a pair based output format |
| SearchMatrixOffsets | [SearchMatrixPoints](#solvio-SearchMatrixPoints) | [SearchMatrixOffsetsResponse](#solvio-SearchMatrixOffsetsResponse) | Compute distance matrix for sampled points with an offset based output format |

 



<a name="solvio-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## solvio.proto



<a name="solvio-HealthCheckReply"></a>

### HealthCheckReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| title | [string](#string) |  |  |
| version | [string](#string) |  |  |
| commit | [string](#string) | optional |  |






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



<a name="solvio-GetConsensusCommitRequest"></a>

### GetConsensusCommitRequest







<a name="solvio-GetConsensusCommitResponse"></a>

### GetConsensusCommitResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| commit | [int64](#int64) |  | Raft commit as u64 |
| term | [int64](#int64) |  | Raft term as u64 |






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
| GetConsensusCommit | [GetConsensusCommitRequest](#solvio-GetConsensusCommitRequest) | [GetConsensusCommitResponse](#solvio-GetConsensusCommitResponse) | Get current commit and term on the target node. |
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
| checksum | [string](#string) | optional | SHA256 digest of the snapshot file |





 

 

 


<a name="solvio-Snapshots"></a>

### Snapshots


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateSnapshotRequest](#solvio-CreateSnapshotRequest) | [CreateSnapshotResponse](#solvio-CreateSnapshotResponse) | Create collection snapshot |
| List | [ListSnapshotsRequest](#solvio-ListSnapshotsRequest) | [ListSnapshotsResponse](#solvio-ListSnapshotsResponse) | List collection snapshots |
| Delete | [DeleteSnapshotRequest](#solvio-DeleteSnapshotRequest) | [DeleteSnapshotResponse](#solvio-DeleteSnapshotResponse) | Delete collection snapshot |
| CreateFull | [CreateFullSnapshotRequest](#solvio-CreateFullSnapshotRequest) | [CreateSnapshotResponse](#solvio-CreateSnapshotResponse) | Create full storage snapshot |
| ListFull | [ListFullSnapshotsRequest](#solvio-ListFullSnapshotsRequest) | [ListSnapshotsResponse](#solvio-ListSnapshotsResponse) | List full storage snapshots |
| DeleteFull | [DeleteFullSnapshotRequest](#solvio-DeleteFullSnapshotRequest) | [DeleteSnapshotResponse](#solvio-DeleteSnapshotResponse) | Delete full storage snapshot |

 



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

