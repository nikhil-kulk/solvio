#!/usr/bin/env bash

set -euo pipefail

declare NODES="${1:-5}"

cat <<-EOF
version: "3.7"

networks:
  echo:
    driver: bridge
    ipam:
     config:
       - subnet: 69.42.0.0/16
         gateway: 69.42.0.255

services:
EOF

for ((node=0; node<NODES; node++))
do
	declare SERVICE_NAME=echo-$node

	declare ADDRESS=69.42.0.$((node + 1))
	declare PORT=$((8000 + node))

	cat <<-EOF
	  $SERVICE_NAME:
	    image: solvio:latest
	    environment:
	      - RUST_LOG=debug
	    networks:
	      echo:
	        ipv4_address: $ADDRESS
	    ports:
	      - "$PORT:8080"
	    restart: always

	EOF
done
