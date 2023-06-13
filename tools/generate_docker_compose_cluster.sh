#!/usr/bin/env bash

# Generate a docker-compose configuration for a cluster.
#
# Parameters:
# - $1: number of nodes (default: 5)
# - $2: base port (default: 6330)
#
# Example usage:
# ./generate_docker_compose_cluster.sh 5 > docker-compose.yaml

set -euo pipefail

declare NODES="${1:-5}"
declare BASE_PORT="${2:-6330}"

cat <<-EOF
version: "3.7"

services:
EOF

for ((node=0; node<NODES; node++))
do
	declare SERVICE_NAME=solvio_node_$node

	declare HTTP_PORT=$((BASE_PORT + node * 10 + 3))
	declare GRPC_PORT=$((BASE_PORT + node * 10 + 4))

	if ((node == 0))
	then
		declare COMMAND="./solvio --uri 'http://$SERVICE_NAME:6335'"
	else
		declare COMMAND="bash -c \"sleep $((10 + node / 10 + RANDOM % 10)) && ./solvio --bootstrap 'http://solvio_node_0:6335' --uri 'http://$SERVICE_NAME:6335'\""
	fi

	cat <<-EOF
	  $SERVICE_NAME:
	    image: solvio/solvio:latest
	    command: $COMMAND
	    restart: always
	    environment:
	      - solvio__SERVICE__GRPC_PORT=6334
	      - solvio__CLUSTER__ENABLED=true
	      - solvio__CLUSTER__P2P__PORT=6335
	      - solvio__CLUSTER__CONSENSUS__MAX_MESSAGE_QUEUE_SIZE=5000
	      - solvio__LOG_LEVEL=debug,raft=info
	    ports:
	      - "$HTTP_PORT:6333"
	      - "$GRPC_PORT:6334"
	    deploy:
	      resources:
	        limits:
	          cpus: "0.3"

	EOF
done
