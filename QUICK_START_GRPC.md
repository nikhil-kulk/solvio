# Quick Start with Solvio using GRPC

DISCLAIMER: The GRPC API for Solvio are under development and disabled by default in production builds. 
Limited functionality is exposed at the moment. This draft tutorial is for testing and internal use only.

## Install grpcurl
This tutorial would use the grpcurl command line tool in order to perform grpc requests. Please follow the
steps provided in the [grpcurl repository](https://github.com/fullstorydev/grpcurl) in order to install it.

## Build Solvio with GRPC feature
The GRPC feature is disabled by default in order not to impact the production binary until complete.
In order to run solvio with grpc feature enabled, executed the following command:
```bash
solvio__SERVICE__GRPC_PORT="6334" cargo run --bin solvio
```
It will run solvio exposing both json and grpc API. If you do not want to use JSON API, add ``--no-default-features ``
flag as well:
```bash
solvio__SERVICE__GRPC_PORT="6334" cargo run --no-default-features --bin solvio
```
Note that actix is not compiled in this case.

## GRPC Service Health Check
Execute the following command
```bash
grpcurl -plaintext -import-path ./lib/api/src/grpc/proto/ -proto solvio.proto -d '{}' 0.0.0.0:6334 solvio.Solvio/HealthCheck
```
Here and below the ```./lib/api/src/grpc/proto/``` should be a path to the folder with a probuf schemas.
Expected response:
```json
{
  "title": "solvio - vector search engine",
  "version": "<vesion>"
}
```

## Collections

### Create collection
First - let's create a collection with dot-production metric.
```bash
grpcurl -plaintext -import-path ./lib/api/src/grpc/proto/ -proto solvio.proto -d '{
        "collection_name": "test_collection",
        "vectors_config": {
            "params": {
                "size": 4,
                "distance": "Dot"
            }
        }
    }' \
0.0.0.0:6334 solvio.Collections/Create
```

Expected response:
```json
{
  "result": true,
  "time": 0.482865481
}
```

### List all collections
We can now view the list of collections to ensure that the collection was created:
```bash
grpcurl -plaintext -import-path ./lib/api/src/grpc/proto/ -proto solvio.proto 0.0.0.0:6334 solvio.Collections/List
```

Expected response:
```json
{
  "collections": [
    {
      "name": "test_collection"
    }
  ],
  "time": 9.4219e-05
}
```

### Update collection
The collection could also be updated:
```bash
grpcurl -plaintext -import-path ./lib/api/src/grpc/proto/ -proto solvio.proto -d '{
        "collection_name": "test_collection",
        "optimizers_config": {
          "max_segment_size": 100
        }
    }' \
0.0.0.0:6334 solvio.Collections/Update
```

## Add points
Let's now add vectors with some payload:

```bash
grpcurl -plaintext -import-path ./lib/api/src/grpc/proto/ -proto solvio.proto -d '{
  "collection_name": "test_collection",
  "wait": true,
  "points": [
    {
      "id": {
        "num": 1
      },
      "vector": [0.05, 0.61, 0.76, 0.74],
      "payload": {
        "city": "Berlin",
        "country": "Germany",
        "population": 1000000,
        "square": 12.5,
        "coords": { "lat": 1.0, "lon": 2.0 }
      }
    },
    {"id": {"num": 2}, "vector": [0.18, 0.01, 0.85, 0.80], "payload": {"square": [10, 11] }},
    {"id": {"num": 3}, "vector": [0.24, 0.18, 0.22, 0.45], "payload": {"count": [0] }},
    {"id": {"num": 4}, "vector": [0.24, 0.18, 0.22, 0.45], "payload": {"coords": [{ "lat": 1.0, "lon": 2.0}, { "lat": 3.0, "lon": 4.0}]}},
    {"id": {"num": 5}, "vector": [0.35, 0.08, 0.11, 0.44]}
  ]
}' \
0.0.0.0:6334 solvio.Points/Upsert
```

Expected response:
```json
{
  "result": {
    "status": "Completed"
  },
  "time": 0.004128988
}
```

### Delete collection
The solvio.Collections/UpdateCollections rpc could also be used to delete a collection:
```bash
grpcurl -plaintext -import-path ./lib/api/src/grpc/proto/ -proto solvio.proto -d '{
          "collection_name": "test_collection"
    }' \
0.0.0.0:6334 solvio.Collections/Delete
```
