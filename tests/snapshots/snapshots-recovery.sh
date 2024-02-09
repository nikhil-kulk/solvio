#!/usr/bin/env bash

set -ex

docker volume create snapshots
docker volume create tempdir
docker volume create storage

declare DOCKER_IMAGE_NAME=solvio-snapshots
#declare DOCKER_IMAGE_NAME=solvio/solvio:snapshots
declare CONTAINER_NAME=solvio-snapshots-container

docker buildx build --build-arg=PROFILE=ci --load ../../ --tag=$DOCKER_IMAGE_NAME


docker run \
    --rm -d \
    -p 6333:6333 -p 6334:6334 \
    -v snapshots:/solvio/snapshots \
    -v tempdir:/solvio/tempdir \
    -v storage:/solvio/storage \
    -e solvio__STORAGE__TEMP_PATH=/solvio/tempdir \
    --name ${CONTAINER_NAME} \
    $DOCKER_IMAGE_NAME


function clear() {
    docker rm -f ${CONTAINER_NAME}
    docker volume rm snapshots
    docker volume rm tempdir
    docker volume rm storage
}

trap clear EXIT

# Wait (up to ~30 seconds) for the service to start
declare retry=0
while [[ $(curl -sS localhost:6333 -w ''%{http_code}'' -o /dev/null) != 200 ]]; do
    if ((retry++ < 30)); then
        sleep 1
    else
        echo "Service failed to start in ~30 seconds" >&2
        exit 1
    fi
done

# Testing scenario:
# - Create collection, insert points and make snapshot
# - Download snapshot
# - Upload snapshot via URL
# - Upload snapshot as file

solvio_HOST='localhost:6333'

# Create collection
curl -X PUT "http://${solvio_HOST}/collections/test_collection" \
  -H 'Content-Type: application/json' \
  --fail -s \
  --data-raw '{
      "vectors": {
        "size": 4,
        "distance": "Dot"
      }
    }'

# Insert points
PAYLOAD=$( jq -n \
   '{ "points": [
      {"id": 1, "vector": [0.19, 0.81, 0.75, 0.11], "payload": {"city":  "London" }},
      {"id": 2, "vector": [0.05, 0.61, 0.76, 0.74], "payload": {"city":  "Berlin" }}
    ]}')

# insert points
curl -L -X PUT "http://$solvio_HOST/collections/test_collection/points?wait=true" \
  -H 'Content-Type: application/json' \
  --fail -s \
  --data-raw "$PAYLOAD" | jq

# Make snapshot

declare SNAPSHOT_NAME=$(curl -X POST "http://${solvio_HOST}/collections/test_collection/snapshots" -H 'Content-Type: application/json' --data-raw '{}' | jq -r '.result.name')

declare SNAPSHOT_URL="http://${solvio_HOST}/collections/test_collection/snapshots/${SNAPSHOT_NAME}"

# Download snapshot
curl -X GET ${SNAPSHOT_URL} -H 'Content-Type: application/json' --fail -s -o test_collection.snapshot

# Upload snapshot via URL
curl -X PUT "http://${solvio_HOST}/collections/test_collection_recovered_1/snapshots/recover" \
     -H 'Content-Type: application/json' \
     --fail -s -d "{\"location\": \"${SNAPSHOT_URL}\"}" | jq

# Upload snapshot as file
curl -X POST "http://${solvio_HOST}/collections/test_collection_recovered_2/snapshots/upload" \
     -H 'Content-Type:multipart/form-data' \
     -F 'snapshot=@test_collection.snapshot' | jq

# Check that all collections are present
curl -X GET "http://${solvio_HOST}/collections/test_collection_recovered_1" --fail | jq

curl -X GET "http://${solvio_HOST}/collections/test_collection_recovered_2" --fail | jq

# Same for the shard snapshot

SHARD_SNAPSHOT_NAME=$(curl -X POST "http://${solvio_HOST}/collections/test_collection/shards/0/snapshots" --fail -H 'Content-Type: application/json' --data-raw '{}' | tee log.json | jq -r '.result.name')

declare SHARD_SNAPSHOT_URL="http://${solvio_HOST}/collections/test_collection/shards/0/snapshots/${SHARD_SNAPSHOT_NAME}"

# Download snapshot

curl -X GET "${SHARD_SNAPSHOT_URL}" -H 'Content-Type: application/json' --fail -s -o test_collection_shard.snapshot

# Upload snapshot via URL

curl -X PUT "http://${solvio_HOST}/collections/test_collection_recovered_1/shards/0/snapshots/recover" \
     -H 'Content-Type: application/json' \
     --fail -s -d "{\"location\": \"${SHARD_SNAPSHOT_URL}\"}" | jq

# Upload snapshot as file

curl -X POST "http://${solvio_HOST}/collections/test_collection_recovered_2/shards/0/snapshots/upload" \
     -H 'Content-Type:multipart/form-data' \
     -F 'snapshot=@test_collection_shard.snapshot' | jq

