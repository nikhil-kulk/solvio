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

if [[ -v RUST_LOG && -n $RUST_LOG ]]
then
	declare ENV="$(
		cat <<-EOF
		    environment:
		      - RUST_LOG=$RUST_LOG
		EOF
	)"
fi

for ((node = 0; node < NODES; node++))
do
	declare SERVICE_NAME=echo-$node

	declare ADDRESS=69.42.0.$((1 + node))
	declare PORT=$((8000 + node))

	cat <<-EOF
	  $SERVICE_NAME:
	    image: echo:latest
	    command: ./echo --http 0.0.0.0:8080 --grpc 0.0.0.0:8081
	    restart: always
	    networks:
	      echo:
	        ipv4_address: $ADDRESS
	    ports:
	      - "127.0.0.1:$PORT:8080"
	${ENV:-}

	EOF
done
