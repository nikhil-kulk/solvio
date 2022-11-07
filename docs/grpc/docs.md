# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [collections.proto](#collections-proto)
    - [AliasOperations](#solvio-AliasOperations)
    - [ChangeAliases](#solvio-ChangeAliases)
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
    - [GetCollectionInfoRequest](#solvio-GetCollectionInfoRequest)
    - [GetCollectionInfoResponse](#solvio-GetCollectionInfoResponse)
    - [HnswConfigDiff](#solvio-HnswConfigDiff)
    - [ListCollectionsRequest](#solvio-ListCollectionsRequest)
    - [ListCollectionsResponse](#solvio-ListCollectionsResponse)
    - [OptimizerStatus](#solvio-OptimizerStatus)
    - [OptimizersConfigDiff](#solvio-OptimizersConfigDiff)
    - [PayloadIndexParams](#solvio-PayloadIndexParams)
    - [PayloadSchemaInfo](#solvio-PayloadSchemaInfo)
    - [RenameAlias](#solvio-RenameAlias)
    - [TextIndexParams](#solvio-TextIndexParams)
    - [UpdateCollection](#solvio-UpdateCollection)
    - [VectorParams](#solvio-VectorParams)
    - [VectorParamsMap](#solvio-VectorParamsMap)
    - [VectorParamsMap.MapEntry](#solvio-VectorParamsMap-MapEntry)
    - [VectorsConfig](#solvio-VectorsConfig)
    - [WalConfigDiff](#solvio-WalConfigDiff)
  
    - [CollectionStatus](#solvio-CollectionStatus)
    - [Distance](#solvio-Distance)
    - [PayloadSchemaType](#solvio-PayloadSchemaType)
    - [TokenizerType](#solvio-TokenizerType)
  
- [collections_service.proto](#collections_service-proto)
    - [Collections](#solvio-Collections)
  
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
    - [DeletePoints](#solvio-DeletePoints)
    - [FieldCondition](#solvio-FieldCondition)
    - [Filter](#solvio-Filter)
    - [GeoBoundingBox](#solvio-GeoBoundingBox)
    - [GeoPoint](#solvio-GeoPoint)
    - [GeoRadius](#solvio-GeoRadius)
    - [GetPoints](#solvio-GetPoints)
    - [GetResponse](#solvio-GetResponse)
    - [HasIdCondition](#solvio-HasIdCondition)
    - [IsEmptyCondition](#solvio-IsEmptyCondition)
    - [Match](#solvio-Match)
    - [NamedVectors](#solvio-NamedVectors)
    - [NamedVectors.VectorsEntry](#solvio-NamedVectors-VectorsEntry)
    - [PayloadExcludeSelector](#solvio-PayloadExcludeSelector)
    - [PayloadIncludeSelector](#solvio-PayloadIncludeSelector)
    - [PointId](#solvio-PointId)
    - [PointStruct](#solvio-PointStruct)
    - [PointStruct.PayloadEntry](#solvio-PointStruct-PayloadEntry)
    - [PointsIdsList](#solvio-PointsIdsList)
    - [PointsOperationResponse](#solvio-PointsOperationResponse)
    - [PointsSelector](#solvio-PointsSelector)
    - [Range](#solvio-Range)
    - [RecommendBatchPoints](#solvio-RecommendBatchPoints)
    - [RecommendBatchResponse](#solvio-RecommendBatchResponse)
    - [RecommendPoints](#solvio-RecommendPoints)
    - [RecommendResponse](#solvio-RecommendResponse)
    - [RetrievedPoint](#solvio-RetrievedPoint)
    - [RetrievedPoint.PayloadEntry](#solvio-RetrievedPoint-PayloadEntry)
    - [ScoredPoint](#solvio-ScoredPoint)
    - [ScoredPoint.PayloadEntry](#solvio-ScoredPoint-PayloadEntry)
    - [ScrollPoints](#solvio-ScrollPoints)
    - [ScrollResponse](#solvio-ScrollResponse)
    - [SearchBatchPoints](#solvio-SearchBatchPoints)
    - [SearchBatchResponse](#solvio-SearchBatchResponse)
    - [SearchParams](#solvio-SearchParams)
    - [SearchPoints](#solvio-SearchPoints)
    - [SearchResponse](#solvio-SearchResponse)
    - [SetPayloadPoints](#solvio-SetPayloadPoints)
    - [SetPayloadPoints.PayloadEntry](#solvio-SetPayloadPoints-PayloadEntry)
    - [UpdateResult](#solvio-UpdateResult)
    - [UpsertPoints](#solvio-UpsertPoints)
    - [ValuesCount](#solvio-ValuesCount)
    - [Vector](#solvio-Vector)
    - [Vectors](#solvio-Vectors)
    - [VectorsSelector](#solvio-VectorsSelector)
    - [WithPayloadSelector](#solvio-WithPayloadSelector)
    - [WithVectorsSelector](#solvio-WithVectorsSelector)
  
    - [FieldType](#solvio-FieldType)
    - [UpdateStatus](#solvio-UpdateStatus)
  
- [points_service.proto](#points_service-proto)
    - [Points](#solvio-Points)
  
- [solvio.proto](#solvio-proto)
    - [HealthCheckReply](#solvio-HealthCheckReply)
    - [HealthCheckRequest](#solvio-HealthCheckRequest)
  
    - [Solvio](#solvio-Solvio)
  
- [snapshots_service.proto](#snapshots_service-proto)
    - [CreateFullSnapshotRequest](#solvio-CreateFullSnapshotRequest)
    - [CreateSnapshotRequest](#solvio-CreateSnapshotRequest)
    - [CreateSnapshotResponse](#solvio-CreateSnapshotResponse)
    - [ListFullSnapshotsRequest](#solvio-ListFullSnapshotsRequest)
    - [ListSnapshotsRequest](#solvio-ListSnapshotsRequest)
    - [ListSnapshotsResponse](#solvio-ListSnapshotsResponse)
    - [SnapshotDescription](#solvio-SnapshotDescription)
  
    - [Snapshots](#solvio-Snapshots)
  
- [Scalar Value Types](#scalar-value-types)



<a name="collections-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## collections.proto



<a name="solvio-AliasOperations"></a>

### AliasOperations



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| create_alias | [CreateAlias](#solvio-CreateAlias) |  |  |
| rename_alias | [RenameAlias](#solvio-RenameAlias) |  |  |
| delete_alias | [DeleteAlias](#solvio-DeleteAlias) |  |  |






<a name="solvio-ChangeAliases"></a>

### ChangeAliases



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| actions | [AliasOperations](#solvio-AliasOperations) | repeated | List of actions |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |






<a name="solvio-CollectionConfig"></a>

### CollectionConfig



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| params | [CollectionParams](#solvio-CollectionParams) |  | Collection parameters |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) |  | Configuration of vector index |
| optimizer_config | [OptimizersConfigDiff](#solvio-OptimizersConfigDiff) |  | Configuration of the optimizers |
| wal_config | [WalConfigDiff](#solvio-WalConfigDiff) |  | Configuration of the Write-Ahead-Log |






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






<a name="solvio-CollectionParamsDiff"></a>

### CollectionParamsDiff



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard that network tries to maintain |
| write_consistency_factor | [uint32](#uint32) | optional | How many replicas should apply the operation for us to consider it successful |






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
| shard_number | [uint32](#uint32) | optional | Number of shards in the collection, default = 1 |
| on_disk_payload | [bool](#bool) | optional | If true - point&#39;s payload will not be stored in memory |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |
| vectors_config | [VectorsConfig](#solvio-VectorsConfig) | optional | Configuration for vectors |
| replication_factor | [uint32](#uint32) | optional | Number of replicas of each shard that network tries to maintain, default = 1 |
| write_consistency_factor | [uint32](#uint32) | optional | How many replicas should apply the operation for us to consider it successful, default = 1 |






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
| ef_construct | [uint64](#uint64) | optional | Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build index. |
| full_scan_threshold | [uint64](#uint64) | optional | Minimal size (in KiloBytes) of vectors for additional payload-based indexing. If payload chunk is smaller than `full_scan_threshold` additional indexing won&#39;t be used - in this case full-scan search should be preferred by query planner and additional indexing is not required. Note: 1Kb = 1 vector of size 256 |
| max_indexing_threads | [uint64](#uint64) | optional | Number of parallel threads used for background index building. If 0 - auto selection. |






<a name="solvio-ListCollectionsRequest"></a>

### ListCollectionsRequest







<a name="solvio-ListCollectionsResponse"></a>

### ListCollectionsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collections | [CollectionDescription](#solvio-CollectionDescription) | repeated |  |
| time | [double](#double) |  | Time spent to process |






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
| default_segment_number | [uint64](#uint64) | optional | Target amount of segments optimizer will try to keep. Real amount of segments may vary depending on multiple parameters:

- Amount of stored points. - Current write RPS.

It is recommended to select default number of segments as a factor of the number of search threads, so that each segment would be handled evenly by one of the threads. |
| max_segment_size | [uint64](#uint64) | optional | Do not create segments larger this size (in KiloBytes). Large segments might require disproportionately long indexation times, therefore it makes sense to limit the size of segments.

If indexation speed have more priority for your - make this parameter lower. If search speed is more important - make this parameter higher. Note: 1Kb = 1 vector of size 256 |
| memmap_threshold | [uint64](#uint64) | optional | Maximum size (in KiloBytes) of vectors to store in-memory per segment. Segments larger than this threshold will be stored as read-only memmaped file. To enable memmap storage, lower the threshold Note: 1Kb = 1 vector of size 256 |
| indexing_threshold | [uint64](#uint64) | optional | Maximum size (in KiloBytes) of vectors allowed for plain index. Default value based on https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md Note: 1Kb = 1 vector of size 256 |
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






<a name="solvio-RenameAlias"></a>

### RenameAlias



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| old_alias_name | [string](#string) |  | Name of the alias to rename |
| new_alias_name | [string](#string) |  | Name of the alias |






<a name="solvio-TextIndexParams"></a>

### TextIndexParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tokenizer | [TokenizerType](#solvio-TokenizerType) |  | Tokenizer type |
| lowercase | [bool](#bool) | optional | If true - all tokens will be lowercased |
| min_token_len | [uint64](#uint64) | optional | Minimal token length |
| max_token_len | [uint64](#uint64) | optional | Maximal token length |






<a name="solvio-UpdateCollection"></a>

### UpdateCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| optimizers_config | [OptimizersConfigDiff](#solvio-OptimizersConfigDiff) | optional | New configuration parameters for the collection |
| timeout | [uint64](#uint64) | optional | Wait timeout for operation commit in seconds, if not specified - default value will be supplied |
| params | [CollectionParamsDiff](#solvio-CollectionParamsDiff) | optional | New configuration parameters for the collection |






<a name="solvio-VectorParams"></a>

### VectorParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| size | [uint64](#uint64) |  | Size of the vectors |
| distance | [Distance](#solvio-Distance) |  | Distance function used for comparing vectors |






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



<a name="solvio-TokenizerType"></a>

### TokenizerType


| Name | Number | Description |
| ---- | ------ | ----------- |
| Unknown | 0 |  |
| Prefix | 1 |  |
| Whitespace | 2 |  |
| Word | 3 |  |


 

 

 



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

 



<a name="json_with_int-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## json_with_int.proto



<a name="solvio-ListValue"></a>

### ListValue
`ListValue` is a wrapper around a repeated field of values.

The JSON representation for `ListValue` is JSON array.


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

The JSON representation for `Struct` is JSON object.


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
list of values. A producer of value is expected to set one of that
variants, absence of any variant indicates an error.

The JSON representation for `Value` is JSON value.


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






<a name="solvio-Condition"></a>

### Condition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| field | [FieldCondition](#solvio-FieldCondition) |  |  |
| is_empty | [IsEmptyCondition](#solvio-IsEmptyCondition) |  |  |
| has_id | [HasIdCondition](#solvio-HasIdCondition) |  |  |
| filter | [Filter](#solvio-Filter) |  |  |






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






<a name="solvio-DeleteFieldIndexCollection"></a>

### DeleteFieldIndexCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| field_name | [string](#string) |  | Field name to delete |






<a name="solvio-DeletePayloadPoints"></a>

### DeletePayloadPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| keys | [string](#string) | repeated | List of keys to delete |
| points | [PointId](#solvio-PointId) | repeated | Affected points |






<a name="solvio-DeletePoints"></a>

### DeletePoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| points | [PointsSelector](#solvio-PointsSelector) |  | Affected points |






<a name="solvio-FieldCondition"></a>

### FieldCondition



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| match | [Match](#solvio-Match) |  | Check if point has field with a given value |
| range | [Range](#solvio-Range) |  | Check if points value lies in a given range |
| geo_bounding_box | [GeoBoundingBox](#solvio-GeoBoundingBox) |  | Check if points geo location lies in a given area |
| geo_radius | [GeoRadius](#solvio-GeoRadius) |  | Check if geo point is within a given radius |
| values_count | [ValuesCount](#solvio-ValuesCount) |  | Check number of values for a specific field |






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






<a name="solvio-GeoPoint"></a>

### GeoPoint



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lon | [double](#double) |  |  |
| lat | [double](#double) |  |  |






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






<a name="solvio-GetResponse"></a>

### GetResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [RetrievedPoint](#solvio-RetrievedPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






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






<a name="solvio-Match"></a>

### Match



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keyword | [string](#string) |  | Match string keyword |
| integer | [int64](#int64) |  | Match integer |
| boolean | [bool](#bool) |  | Match boolean |
| text | [string](#string) |  | Match text |






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






<a name="solvio-Range"></a>

### Range



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lt | [double](#double) | optional |  |
| gt | [double](#double) | optional |  |
| gte | [double](#double) | optional |  |
| lte | [double](#double) | optional |  |






<a name="solvio-RecommendBatchPoints"></a>

### RecommendBatchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| recommend_points | [RecommendPoints](#solvio-RecommendPoints) | repeated |  |






<a name="solvio-RecommendBatchResponse"></a>

### RecommendBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#solvio-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-RecommendPoints"></a>

### RecommendPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| positive | [PointId](#solvio-PointId) | repeated | Look for vectors closest to those |
| negative | [PointId](#solvio-PointId) | repeated | Try to avoid vectors like this |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| limit | [uint64](#uint64) |  | Max number of result |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#solvio-SearchParams) |  | Search config |
| score_threshold | [float](#float) | optional | If provided - cut off results with worse scores |
| offset | [uint64](#uint64) | optional | Offset of the result |
| using | [string](#string) | optional | Define which vector to use for recommendation, if not specified - default vector |
| with_vectors | [WithVectorsSelector](#solvio-WithVectorsSelector) | optional | Options for specifying which vectors to include into response |






<a name="solvio-RecommendResponse"></a>

### RecommendResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [ScoredPoint](#solvio-ScoredPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






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






<a name="solvio-SearchBatchResponse"></a>

### SearchBatchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [BatchResult](#solvio-BatchResult) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-SearchParams"></a>

### SearchParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hnsw_ef | [uint64](#uint64) | optional | Params relevant to HNSW index. Size of the beam in a beam-search. Larger the value - more accurate the result, more time required for search. |
| exact | [bool](#bool) | optional | Search without approximation. If set to true, search may run long but with exact results. |






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
| points | [PointId](#solvio-PointId) | repeated | List of point to modify |






<a name="solvio-SetPayloadPoints-PayloadEntry"></a>

### SetPayloadPoints.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Value](#solvio-Value) |  |  |






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





 


<a name="solvio-FieldType"></a>

### FieldType


| Name | Number | Description |
| ---- | ------ | ----------- |
| FieldTypeKeyword | 0 |  |
| FieldTypeInteger | 1 |  |
| FieldTypeFloat | 2 |  |
| FieldTypeGeo | 3 |  |
| FieldTypeText | 4 |  |



<a name="solvio-UpdateStatus"></a>

### UpdateStatus


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownUpdateStatus | 0 |  |
| Acknowledged | 1 | Update is received, but not processed yet |
| Completed | 2 | Update is applied and ready for search |


 

 

 



<a name="points_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## points_service.proto


 

 

 


<a name="solvio-Points"></a>

### Points


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Upsert | [UpsertPoints](#solvio-UpsertPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Perform insert &#43; updates on points. If point with given ID already exists - it will be overwritten. |
| Delete | [DeletePoints](#solvio-DeletePoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Delete points |
| Get | [GetPoints](#solvio-GetPoints) | [GetResponse](#solvio-GetResponse) | Retrieve points |
| SetPayload | [SetPayloadPoints](#solvio-SetPayloadPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Set payload for points |
| DeletePayload | [DeletePayloadPoints](#solvio-DeletePayloadPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Delete specified key payload for points |
| ClearPayload | [ClearPayloadPoints](#solvio-ClearPayloadPoints) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Remove all payload for specified points |
| CreateFieldIndex | [CreateFieldIndexCollection](#solvio-CreateFieldIndexCollection) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Create index for field in collection |
| DeleteFieldIndex | [DeleteFieldIndexCollection](#solvio-DeleteFieldIndexCollection) | [PointsOperationResponse](#solvio-PointsOperationResponse) | Delete field index for collection |
| Search | [SearchPoints](#solvio-SearchPoints) | [SearchResponse](#solvio-SearchResponse) | Retrieve closest points based on vector similarity and given filtering conditions |
| SearchBatch | [SearchBatchPoints](#solvio-SearchBatchPoints) | [SearchBatchResponse](#solvio-SearchBatchResponse) | Retrieve closest points based on vector similarity and given filtering conditions |
| Scroll | [ScrollPoints](#solvio-ScrollPoints) | [ScrollResponse](#solvio-ScrollResponse) | Iterate over all or filtered points points |
| Recommend | [RecommendPoints](#solvio-RecommendPoints) | [RecommendResponse](#solvio-RecommendResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples. |
| RecommendBatch | [RecommendBatchPoints](#solvio-RecommendBatchPoints) | [RecommendBatchResponse](#solvio-RecommendBatchResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples. |
| Count | [CountPoints](#solvio-CountPoints) | [CountResponse](#solvio-CountResponse) | Count points in collection with given filtering conditions |

 



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
| CreateFull | [CreateFullSnapshotRequest](#solvio-CreateFullSnapshotRequest) | [CreateSnapshotResponse](#solvio-CreateSnapshotResponse) | Create full storage snapshot |
| ListFull | [ListFullSnapshotsRequest](#solvio-ListFullSnapshotsRequest) | [ListSnapshotsResponse](#solvio-ListSnapshotsResponse) | List full storage snapshots |

 



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

