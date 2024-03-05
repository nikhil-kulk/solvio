#!/usr/bin/env bash
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

# Test it boots up fine with the old storage
./target/debug/solvio & PID=$!

sleep 1

declare retry=0
until curl --output /dev/null --silent --get --fail http://$solvio_HOST/collections; do
  if ((retry++ < 30)); then
      printf 'waiting for server to start...'
      sleep 1
  else
      echo "Collections failed to load in ~30 seconds" >&2
      exit 2
  fi
done

echo "server ready to serve traffic"

echo "server is going down"
kill -9 $PID
echo "END"


# Test recovering from an old snapshot
gzip -d --keep ./tests/storage-compat/full-snapshot.snapshot.gz

rm -rf ./storage
./target/debug/solvio \
  --storage-snapshot ./tests/storage-compat/full-snapshot.snapshot \
  & PID=$!

declare retry=0
until curl --output /dev/null --silent --get --fail http://$solvio_HOST/collections/test_collection; do
  if ((retry++ < 30)); then
      printf 'waiting for server to start...'
      sleep 1
  else
      echo "Collection failed to load in ~30 seconds" >&2
      exit 2
  fi
done

echo "server ready to serve traffic"

echo "server is going down"
kill -9 $PID

rm tests/storage-compat/full-snapshot.snapshot

echo "END"
