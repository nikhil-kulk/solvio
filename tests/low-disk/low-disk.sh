#!/usr/bin/env bash
# Run Solvio container with limited disk amount
# and verify that it doesn't crash when disk
# is running low during points insertion.

set -xeuo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

declare DOCKER_IMAGE_NAME=solvio-recovery

docker buildx build --build-arg=PROFILE=ci --load ../../ --tag=$DOCKER_IMAGE_NAME

declare OOD_CONTAINER_NAME=solvio-ood

docker rm -f ${OOD_CONTAINER_NAME} || true

declare container && container=$(
    docker run --rm -d \
      --mount type=tmpfs,target=/solvio/storage,tmpfs-size=10240000 \
      -p 127.0.0.1:6333:6333 \
      -p 127.0.0.1:6334:6334 \
      --name ${OOD_CONTAINER_NAME} \
      $DOCKER_IMAGE_NAME
)

function cleanup {
    docker stop $container || true
}

trap cleanup EXIT

# Wait (up to ~30 seconds) for the service to start
declare retry=0
while [[ $(curl -sS localhost:6333 -w ''%{http_code}'' -o /dev/null) != 200 ]]; do
    if ((retry++ < 30)); then
      sleep 1
    else
        echo "Service failed to start in ~30 seconds" >&2
        exit 7
    fi
done

#check that low disk is handled OK during points insertion
python3 create_items.py low-disk 2000 6333

sleep 5

declare EXPECTED_TEXT='"status":"red"'

# Check that curl returns collection's status as red
CURL_RESPONSE="$(curl -sS localhost:6333/collections/low-disk)"

if ! echo "$CURL_RESPONSE" | grep "${EXPECTED_TEXT}"; then
    echo "'${EXPECTED_TEXT}' not found in curl output" >&2
    exit 8
fi

# Check that there's an OOD log message in service logs
declare OUT_OF_DISK_MSG='No space left on device:'

if ! docker logs "$container" 2>&1 | grep "$OUT_OF_DISK_MSG"; then
    echo "'$OUT_OF_DISK_MSG' log message not found in $container container logs" >&2
    exit 9
fi

printf 'Insertion: OK\n\n'

echo "Success"