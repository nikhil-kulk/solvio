#!/bin/bash

set -ex

# Ensure current path is project root
cd "$(dirname "$0")/../"

solvio_HOST='localhost:6335'

docker_grpcurl="docker run --rm --network=host -v ${PWD}/lib/api/src/grpc/proto:/proto fullstorydev/grpcurl -plaintext -import-path /proto -proto solvio.proto"

$docker_grpcurl $solvio_HOST solvio.SolvioInternal/GetHttpPort
