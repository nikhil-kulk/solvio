#!/bin/bash
# This runs validates the storage compatibility

set -ex
echo $PWD
# Ensure current path is project root
cd "$(dirname "$0")/../../"

solvio_HOST='localhost:6333'

# Build
cargo build

# Sync git large file
git lfs pull

# Uncompress snapshot storage
tar -xvjf ./tests/storage-compat/storage.tar.bz2

# Run in background
./target/debug/solvio &

# Sleep to make sure the process has started (workaround for empty pidof)
sleep 5

## Capture PID of the run
PID=$(pidof "./target/debug/solvio")
echo $PID

until curl --output /dev/null --silent --get --fail http://$solvio_HOST/collections; do
  printf 'waiting for server to start...'
  sleep 5
done

echo "server ready to serve traffic"

echo "server is going down"
kill -9 $PID
echo "END"


# Test recovering from an old snapshot
gzip -d --keep ./tests/storage-compat/collection.snapshot.gz

rm -rf ./storage
./target/debug/solvio --snapshot ./tests/storage-compat/collection.snapshot:test_collection &

# Sleep to make sure the process has started (workaround for empty pidof)
sleep 5

## Capture PID of the run
PID=$(pidof "./target/debug/solvio")
echo $PID

until curl --output /dev/null --silent --get --fail http://$solvio_HOST/collections/test_collection; do
  printf 'waiting for server to start...'
  sleep 5
done

echo "server ready to serve traffic"

echo "server is going down"
kill -9 $PID
echo "END"
