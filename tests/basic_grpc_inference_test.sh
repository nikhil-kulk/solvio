#!/usr/bin/env bash
# This test checks that Solvio answers to all API mentioned in README.md as expected

set -ex

# Ensure current path is project root
cd "$(dirname "$0")/../"

solvio_HOST=${solvio_HOST:-'127.0.0.1:6334'}
GRPCURL_PATH="/opt/homebrew/bin/grpcurl"

# Check if local grpcurl exists and is executable
if [ -x "$GRPCURL_PATH" ]; then
    echo "Using local grpcurl installation"
    grpcurl_base=(
        "$GRPCURL_PATH"
        "-plaintext"
        "-import-path"
        "./lib/api/src/grpc/proto"
        "-proto"
        "./lib/api/src/grpc/proto/solvio.proto"
    )
else
    echo "Local grpcurl not found, using Docker container"
    # Define base grpcurl command for Docker in case local grpcurl is not found
    grpcurl_base=(
        "docker" "run" "--rm" "--network=host"
        "-v" "${PWD}/lib/api/src/grpc/proto:/proto"
        "fullstorydev/grpcurl"
        "-plaintext"
        "-import-path" "/proto"
        "-proto" "solvio.proto"
    )
fi

# Add headers if they exist
if [ -n "${solvio_HOST_HEADERS}" ]; then
    while read h; do
        grpcurl_base+=("-H" "$h")
    done <<< $(echo "${solvio_HOST_HEADERS}" | jq -r 'to_entries|map("\(.key): \(.value)")[]')
fi

# Function to execute grpcurl commands
execute_grpcurl() {
    "${grpcurl_base[@]}" "$@"
}

# Upsert first point
execute_grpcurl -d '{
"collection_name": "sparse_charts",
"wait": true,
"ordering": null,
"points": [
 {
   "id": { "num": 1 },
   "vectors": {
     "vectors": {
       "vectors": {
         "keywords": {
           "document": {
             "text": "my text",
             "model": "Solvio/bm25"
           }
         }
       }
     }
   },
   "payload": {
     "city": { "string_value": "Berlin" }
   }
 }
]
}' $solvio_HOST solvio.Points/Upsert

# Upsert multiple points
execute_grpcurl -d '{
"collection_name": "sparse_charts",
"wait": true,
"ordering": null,
"points": [
 {
   "id": { "num": 1 },
   "vectors": {
     "vectors": {
       "vectors": {
         "keywords": {
           "document": {
             "text": "my text",
             "model": "Solvio/bm25"
           }
         }
       }
     }
   },
   "payload": {
     "city": { "string_value": "Berlin" }
   }
 },
 {
   "id": { "num": 2 },
   "vectors": {
     "vectors": {
       "vectors": {
         "keywords": {
           "document": {
             "text": "my text another",
             "model": "Solvio/bm25"
           }
         }
       }
     }
   },
   "payload": {
     "city": { "string_value": "Amsterdam" }
   }
 }
]
}' $solvio_HOST solvio.Points/Upsert
