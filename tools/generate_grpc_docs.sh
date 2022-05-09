#!/usr/bin/env bash

set -e

# Ensure current path is project root
cd "$(dirname "$0")/../"

# Create a temporary directory and store its name in a variable.
TEMPD=$(mktemp -d)

cp -r $(pwd)/lib/api/src/grpc/proto/* ${TEMPD}

# Do not generate docs for internal services
rm ${TEMPD}/'collections_internal_service.proto'
rm ${TEMPD}/'points_internal_service.proto'
rm ${TEMPD}/'raft_service.proto'

cat ${TEMPD}/solvio.proto \
  | grep -v 'collections_internal_service.proto' \
  | grep -v 'points_internal_service.proto' \
  | grep -v 'raft_service.proto'\
   > ${TEMPD}/solvio.proto.tmp
mv ${TEMPD}/solvio.proto.tmp ${TEMPD}/solvio.proto

docker run --rm \
  -v $(pwd)/docs/grpc:/out \
  -v ${TEMPD}:/protos \
  pseudomuto/protoc-gen-doc --doc_opt=markdown,docs.md

# <https://github.com/pseudomuto/protoc-gen-doc/issues/383>
sudo chown -R "$USER:$(id -g -n)" $(pwd)/docs/grpc
