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
cargo run --features=grpc --bin solvio
```
It will run solvio exposing both json and grpc API. If you do not want to use JSON API, add ``--no-default-features ``
flag as well:
```bash
cargo run --features=grpc --no-default-features  --bin solvio
```
Note that actix is not compiled in this case.

## GRPC Service Health Check
Execute the following command
```bash
grpcurl -plaintext -import-path ./src/tonic/proto -proto solvio.proto -d '{}' [::]:6334 solvio.Solvio/HealthCheck
```
Here and below the ```./src/tonic/proto``` should be a path to the folder with a probuf schemas.
Expected response:
```json
{
  "title": "solvio - vector search engine",
  "version": "<vesion>"
}
```