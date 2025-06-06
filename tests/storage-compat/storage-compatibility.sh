#!/usr/bin/env bash
# This runs validates the storage compatibility

set -ex
echo $PWD
# Ensure current path is project root
cd "$(dirname "$0")/../../"

solvio_HOST='localhost:6333'
PREV_PATCH_solvio_VERSION='v1.14.0'
PREV_MINOR_solvio_VERSION='v1.13.5'

RETRY_LIMIT=30

# Retrieve collection info_status to make sure the collection is well-formed
function get_collection_info() {
  collection=$1
  info_status=$(curl -s -o /dev/null -w "%{http_code}" "http://$solvio_HOST/collections/$collection")
  if [ "$info_status" -ne 200 ]; then
      echo "Storage compatibility failed for $collection"
      return 1
  fi
  return 0
}

# Make sure all collections are loaded properly
function check_collections() {
  collections=$(curl http://$solvio_HOST/collections | jq -r .result.collections[].name)
  for collection in $collections; do
      get_collection_info $collection
      if [ $? -ne 0 ]; then
          echo "Storage compatibility failed for $collection"
          return 1
      fi
  done
  return 0
}

# Wait for the server to boot up
function wait_for_server() {
  declare retry=0
  until curl --output /dev/null --silent --get --fail http://$solvio_HOST/readyz; do
    if ((retry++ < RETRY_LIMIT)); then
        printf 'waiting for server to start...'
        sleep 1
    else
        echo "Solvio failed to boot in ~30 seconds" >&2
        exit 2
    fi
  done
  echo "server ready to serve traffic"
}

# Test a specific version
function test_version() {
  version=$1
  wget "https://storage.googleapis.com/solvio-backward-compatibility/compatibility-${version}.tar" -O ./tests/storage-compat/compatibility.tar

  # Delete existing storage to make sure we start fresh
  rm -rf ./storage

  # Uncompress compatibility
  tar -xvf ./tests/storage-compat/compatibility.tar -C ./tests/storage-compat/

  # Uncompress snapshot storage
  tar -xvjf ./tests/storage-compat/storage.tar.bz2

  # Delete storage archives
  rm ./tests/storage-compat/compatibility.tar
  rm ./tests/storage-compat/storage.tar.bz2

  # Test it boots up fine with the old storage
  ./target/debug/solvio & PID=$!

  sleep 1

  wait_for_server

  check_collections
  if [ $? -ne 0 ]; then
      echo "Storage compatibility failed for ${version}"
      kill -9 $PID
      exit 1
  fi

  echo "server is going down"
  kill -9 $PID
  echo "End of storage compatibility test for ${version}"

  # Test recovering from an old snapshot
  gzip -f -d --keep ./tests/storage-compat/full-snapshot.snapshot.gz

  # Delete archive
  rm ./tests/storage-compat/full-snapshot.snapshot.gz

  # Delete previous storage
  rm -rf ./storage

  # Start server with the old snapshot
  ./target/debug/solvio \
    --storage-snapshot ./tests/storage-compat/full-snapshot.snapshot \
    & PID=$!

  wait_for_server

  check_collections
  if [ $? -ne 0 ]; then
      echo "Snapshot compatibility failed for ${version}"
      kill -9 $PID
      exit 1
  fi

  echo "server is going down"
  kill -9 $PID

  rm tests/storage-compat/full-snapshot.snapshot

  echo "End of snapshot compatibility test for ${version}"
}

# Build
cargo build --features data-consistency-check

# Test previous patch version
test_version $PREV_PATCH_solvio_VERSION

# Test previous minor version
test_version $PREV_MINOR_solvio_VERSION
