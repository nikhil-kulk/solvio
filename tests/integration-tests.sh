#!/bin/bash
# This runs all integration test in isolation

set -ex

# Ensure current path is project root
cd "$(dirname "$0")/../"

solvio_HOST='localhost:6333'
export solvio__SERVICE__GRPC_PORT="6334"

# Run in background
$(./target/debug/solvio) &

# Sleep to make sure the process has started (workaround for empty pidof)
sleep 5

## Capture PID of the run
PID=$(pidof "./target/debug/solvio")
echo $PID

until $(curl --output /dev/null --silent --get --fail http://$solvio_HOST/collections); do
  printf 'waiting for server to start...'
  sleep 5
done

echo "server ready to serve traffic"

./tests/openapi_integration_test.sh

./tests/basic_api_test.sh

./tests/basic_grpc_test.sh

echo "server is going down"
$(kill -9 $PID)
echo "END"
