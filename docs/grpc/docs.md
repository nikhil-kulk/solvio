# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [collections.proto](#collections-proto)
    - [CollectionConfig](#solvio-CollectionConfig)
    - [CollectionDescription](#solvio-CollectionDescription)
    - [CollectionInfo](#solvio-CollectionInfo)
    - [CollectionInfo.PayloadSchemaEntry](#solvio-CollectionInfo-PayloadSchemaEntry)
    - [CollectionOperationResponse](#solvio-CollectionOperationResponse)
    - [CollectionParams](#solvio-CollectionParams)
    - [CreateCollection](#solvio-CreateCollection)
    - [DeleteCollection](#solvio-DeleteCollection)
    - [GetCollectionInfoRequest](#solvio-GetCollectionInfoRequest)
    - [GetCollectionInfoResponse](#solvio-GetCollectionInfoResponse)
    - [HnswConfigDiff](#solvio-HnswConfigDiff)
    - [ListCollectionsRequest](#solvio-ListCollectionsRequest)
    - [ListCollectionsResponse](#solvio-ListCollectionsResponse)
    - [OptimizerStatus](#solvio-OptimizerStatus)
    - [OptimizersConfigDiff](#solvio-OptimizersConfigDiff)
    - [PayloadSchemaInfo](#solvio-PayloadSchemaInfo)
    - [UpdateCollection](#solvio-UpdateCollection)
    - [WalConfigDiff](#solvio-WalConfigDiff)
  
    - [CollectionStatus](#solvio-CollectionStatus)
    - [Distance](#solvio-Distance)
    - [PayloadSchemaType](#solvio-PayloadSchemaType)
  
    - [Collections](#solvio-Collections)
  
- [points.proto](#points-proto)
    - [ClearPayloadPoints](#solvio-ClearPayloadPoints)
    - [Condition](#solvio-Condition)
    - [CreateFieldIndexCollection](#solvio-CreateFieldIndexCollection)
    - [DeleteFieldIndexCollection](#solvio-DeleteFieldIndexCollection)
    - [DeletePayloadPoints](#solvio-DeletePayloadPoints)
    - [DeletePoints](#solvio-DeletePoints)
    - [FieldCondition](#solvio-FieldCondition)
    - [Filter](#solvio-Filter)
    - [FloatPayload](#solvio-FloatPayload)
    - [GeoBoundingBox](#solvio-GeoBoundingBox)
    - [GeoPayload](#solvio-GeoPayload)
    - [GeoPoint](#solvio-GeoPoint)
    - [GeoRadius](#solvio-GeoRadius)
    - [GetPoints](#solvio-GetPoints)
    - [GetResponse](#solvio-GetResponse)
    - [HasIdCondition](#solvio-HasIdCondition)
    - [IntegerPayload](#solvio-IntegerPayload)
    - [KeywordPayload](#solvio-KeywordPayload)
    - [Match](#solvio-Match)
    - [Payload](#solvio-Payload)
    - [PayloadExcludeSelector](#solvio-PayloadExcludeSelector)
    - [PayloadIncludeSelector](#solvio-PayloadIncludeSelector)
    - [PointId](#solvio-PointId)
    - [PointStruct](#solvio-PointStruct)
    - [PointStruct.PayloadEntry](#solvio-PointStruct-PayloadEntry)
    - [PointsIdsList](#solvio-PointsIdsList)
    - [PointsOperationResponse](#solvio-PointsOperationResponse)
    - [PointsSelector](#solvio-PointsSelector)
    - [Range](#solvio-Range)
    - [RecommendPoints](#solvio-RecommendPoints)
    - [RecommendResponse](#solvio-RecommendResponse)
    - [RetrievedPoint](#solvio-RetrievedPoint)
    - [RetrievedPoint.PayloadEntry](#solvio-RetrievedPoint-PayloadEntry)
    - [ScoredPoint](#solvio-ScoredPoint)
    - [ScoredPoint.PayloadEntry](#solvio-ScoredPoint-PayloadEntry)
    - [ScrollPoints](#solvio-ScrollPoints)
    - [ScrollResponse](#solvio-ScrollResponse)
    - [SearchParams](#solvio-SearchParams)
    - [SearchPoints](#solvio-SearchPoints)
    - [SearchResponse](#solvio-SearchResponse)
    - [SetPayloadPoints](#solvio-SetPayloadPoints)
    - [SetPayloadPoints.PayloadEntry](#solvio-SetPayloadPoints-PayloadEntry)
    - [UpdateResult](#solvio-UpdateResult)
    - [UpsertPoints](#solvio-UpsertPoints)
    - [WithPayloadSelector](#solvio-WithPayloadSelector)
  
    - [UpdateStatus](#solvio-UpdateStatus)
  
    - [Points](#solvio-Points)
  
- [solvio.proto](#solvio-proto)
    - [HealthCheckReply](#solvio-HealthCheckReply)
    - [HealthCheckRequest](#solvio-HealthCheckRequest)
  
    - [Solvio](#solvio-Solvio)
  
- [Scalar Value Types](#scalar-value-types)



<a name="collections-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## collections.proto



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
| disk_data_size | [uint64](#uint64) |  | Used disk space |
| ram_data_size | [uint64](#uint64) |  | Used RAM (not implemented) |
| config | [CollectionConfig](#solvio-CollectionConfig) |  | Configuration |
| payload_schema | [CollectionInfo.PayloadSchemaEntry](#solvio-CollectionInfo-PayloadSchemaEntry) | repeated | Collection data types |






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
| vector_size | [uint64](#uint64) |  | Size of the vectors |
| distance | [Distance](#solvio-Distance) |  | Distance function used for comparing vectors |
| shard_number | [uint32](#uint32) |  | Number of shards in collection |






<a name="solvio-CreateCollection"></a>

### CreateCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| vector_size | [uint64](#uint64) |  | Size of the vectors |
| distance | [Distance](#solvio-Distance) |  | Distance function used for comparing vectors |
| hnsw_config | [HnswConfigDiff](#solvio-HnswConfigDiff) | optional | Configuration of vector index |
| wal_config | [WalConfigDiff](#solvio-WalConfigDiff) | optional | Configuration of the Write-Ahead-Log |
| optimizers_config | [OptimizersConfigDiff](#solvio-OptimizersConfigDiff) | optional | Configuration of the optimizers |






<a name="solvio-DeleteCollection"></a>

### DeleteCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |






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
| full_scan_threshold | [uint64](#uint64) | optional | Minimal amount of points for additional payload-based indexing. If payload chunk is smaller than `full_scan_threshold` additional indexing won&#39;t be used - in this case full-scan search should be preferred by query planner and additional indexing is not required. |






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
| max_segment_size | [uint64](#uint64) | optional | Do not create segments larger this number of points. Large segments might require disproportionately long indexation times, therefore it makes sense to limit the size of segments.

If indexation speed have more priority for your - make this parameter lower. If search speed is more important - make this parameter higher. |
| memmap_threshold | [uint64](#uint64) | optional | Maximum number of vectors to store in-memory per segment. Segments larger than this threshold will be stored as read-only memmaped file. |
| indexing_threshold | [uint64](#uint64) | optional | Maximum number of vectors allowed for plain index. Default value based on https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md |
| payload_indexing_threshold | [uint64](#uint64) | optional | Starting from this amount of vectors per-segment the engine will start building index for payload. |
| flush_interval_sec | [uint64](#uint64) | optional | Interval between forced flushes. |
| max_optimization_threads | [uint64](#uint64) | optional | Max number of threads, which can be used for optimization. If 0 - `NUM_CPU - 1` will be used |






<a name="solvio-PayloadSchemaInfo"></a>

### PayloadSchemaInfo



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data_type | [PayloadSchemaType](#solvio-PayloadSchemaType) |  | Field data type |
| indexed | [bool](#bool) |  | If this field is indexed |






<a name="solvio-UpdateCollection"></a>

### UpdateCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | Name of the collection |
| optimizers_config | [OptimizersConfigDiff](#solvio-OptimizersConfigDiff) | optional | New configuration parameters for the collection |






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


 

 


<a name="solvio-Collections"></a>

### Collections


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Get | [GetCollectionInfoRequest](#solvio-GetCollectionInfoRequest) | [GetCollectionInfoResponse](#solvio-GetCollectionInfoResponse) | Get detailed information about specified existing collection |
| List | [ListCollectionsRequest](#solvio-ListCollectionsRequest) | [ListCollectionsResponse](#solvio-ListCollectionsResponse) | Get list name of all existing collections |
| Create | [CreateCollection](#solvio-CreateCollection) | [CollectionOperationResponse](#solvio-CollectionOperationResponse) | Create new collection with given parameters |
| Update | [UpdateCollection](#solvio-UpdateCollection) | [CollectionOperationResponse](#solvio-CollectionOperationResponse) | Update parameters of the existing collection |
| Delete | [DeleteCollection](#solvio-DeleteCollection) | [CollectionOperationResponse](#solvio-CollectionOperationResponse) | Drop collection and all associated data |

 



<a name="points-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## points.proto



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
| hasId | [HasIdCondition](#solvio-HasIdCondition) |  |  |
| filter | [Filter](#solvio-Filter) |  |  |






<a name="solvio-CreateFieldIndexCollection"></a>

### CreateFieldIndexCollection



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| wait | [bool](#bool) | optional | Wait until the changes have been applied? |
| field_name | [string](#string) |  | Field name to index |






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






<a name="solvio-Filter"></a>

### Filter



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| should | [Condition](#solvio-Condition) | repeated | At least one of those conditions should match |
| must | [Condition](#solvio-Condition) | repeated | All conditions must match |
| must_not | [Condition](#solvio-Condition) | repeated | All conditions must NOT match |






<a name="solvio-FloatPayload"></a>

### FloatPayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [double](#double) | repeated |  |






<a name="solvio-GeoBoundingBox"></a>

### GeoBoundingBox



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| top_left | [GeoPoint](#solvio-GeoPoint) |  | north-west corner |
| bottom_right | [GeoPoint](#solvio-GeoPoint) |  | south-east corner |






<a name="solvio-GeoPayload"></a>

### GeoPayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [GeoPoint](#solvio-GeoPoint) | repeated |  |






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
| with_vector | [bool](#bool) | optional | Return point vector with the result. |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |






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






<a name="solvio-IntegerPayload"></a>

### IntegerPayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [int64](#int64) | repeated |  |






<a name="solvio-KeywordPayload"></a>

### KeywordPayload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [string](#string) | repeated |  |






<a name="solvio-Match"></a>

### Match



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keyword | [string](#string) |  | Match string keyword |
| integer | [int64](#int64) |  | Match integer |






<a name="solvio-Payload"></a>

### Payload



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| keyword | [KeywordPayload](#solvio-KeywordPayload) |  |  |
| integer | [IntegerPayload](#solvio-IntegerPayload) |  |  |
| float | [FloatPayload](#solvio-FloatPayload) |  |  |
| geo | [GeoPayload](#solvio-GeoPayload) |  |  |






<a name="solvio-PayloadExcludeSelector"></a>

### PayloadExcludeSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| exclude | [string](#string) | repeated | List of payload keys to exclude from the result |






<a name="solvio-PayloadIncludeSelector"></a>

### PayloadIncludeSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| include | [string](#string) | repeated | List of payload keys to include into result |






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
| vector | [float](#float) | repeated |  |
| payload | [PointStruct.PayloadEntry](#solvio-PointStruct-PayloadEntry) | repeated |  |






<a name="solvio-PointStruct-PayloadEntry"></a>

### PointStruct.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Payload](#solvio-Payload) |  |  |






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






<a name="solvio-RecommendPoints"></a>

### RecommendPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| positive | [PointId](#solvio-PointId) | repeated | Look for vectors closest to those |
| negative | [PointId](#solvio-PointId) | repeated | Try to avoid vectors like this |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| top | [uint64](#uint64) |  | Max number of result |
| with_vector | [bool](#bool) | optional | Return point vector with the result. |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#solvio-SearchParams) |  | Search config |






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
| vector | [float](#float) | repeated |  |






<a name="solvio-RetrievedPoint-PayloadEntry"></a>

### RetrievedPoint.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Payload](#solvio-Payload) |  |  |






<a name="solvio-ScoredPoint"></a>

### ScoredPoint



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [PointId](#solvio-PointId) |  | Point id |
| payload | [ScoredPoint.PayloadEntry](#solvio-ScoredPoint-PayloadEntry) | repeated | Payload |
| score | [float](#float) |  | Similarity score |
| vector | [float](#float) | repeated | Vector |
| version | [uint64](#uint64) |  | Last update operation applied to this point |






<a name="solvio-ScoredPoint-PayloadEntry"></a>

### ScoredPoint.PayloadEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Payload](#solvio-Payload) |  |  |






<a name="solvio-ScrollPoints"></a>

### ScrollPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  |  |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| offset | [PointId](#solvio-PointId) | optional | Start with this ID |
| limit | [uint32](#uint32) | optional | Max number of result |
| with_vector | [bool](#bool) | optional | Return point vector with the result. |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |






<a name="solvio-ScrollResponse"></a>

### ScrollResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| next_page_offset | [PointId](#solvio-PointId) | optional | Use this offset for the next query |
| result | [RetrievedPoint](#solvio-RetrievedPoint) | repeated |  |
| time | [double](#double) |  | Time spent to process |






<a name="solvio-SearchParams"></a>

### SearchParams



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hnsw_ef | [uint64](#uint64) | optional | Params relevant to HNSW index. Size of the beam in a beam-search. Larger the value - more accurate the result, more time required for search. |






<a name="solvio-SearchPoints"></a>

### SearchPoints



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collection_name | [string](#string) |  | name of the collection |
| vector | [float](#float) | repeated | vector |
| filter | [Filter](#solvio-Filter) |  | Filter conditions - return only those points that satisfy the specified conditions |
| top | [uint64](#uint64) |  | Max number of result |
| with_vector | [bool](#bool) | optional | Return point vector with the result. |
| with_payload | [WithPayloadSelector](#solvio-WithPayloadSelector) |  | Options for specifying which payload to include or not |
| params | [SearchParams](#solvio-SearchParams) |  | Search config |






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
| value | [Payload](#solvio-Payload) |  |  |






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






<a name="solvio-WithPayloadSelector"></a>

### WithPayloadSelector



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enable | [bool](#bool) |  | If `true` - return all payload, if `false` - none |
| include | [PayloadIncludeSelector](#solvio-PayloadIncludeSelector) |  |  |
| exclude | [PayloadExcludeSelector](#solvio-PayloadExcludeSelector) |  |  |





 


<a name="solvio-UpdateStatus"></a>

### UpdateStatus


| Name | Number | Description |
| ---- | ------ | ----------- |
| UnknownUpdateStatus | 0 |  |
| Acknowledged | 1 | Update is received, but not processed yet |
| Completed | 2 | Update is applied and ready for search |


 

 


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
| Scroll | [ScrollPoints](#solvio-ScrollPoints) | [ScrollResponse](#solvio-ScrollResponse) | Iterate over all or filtered points points |
| Recommend | [RecommendPoints](#solvio-RecommendPoints) | [RecommendResponse](#solvio-RecommendResponse) | Look for the points which are closer to stored positive examples and at the same time further to negative examples. |

 



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

