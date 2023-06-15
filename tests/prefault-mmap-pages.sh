#!/bin/bash

set -euo pipefail

declare ROOT=$PWD
declare solvio_DIR=$ROOT/$1
declare BFB_DIR=$ROOT/$2

if [[ ! -d $solvio_DIR ]]
then
	echo "$solvio_DIR is not a directory or does not exist" >&2
	exit 1
fi

if [[ ! -d $BFB_DIR ]]
then
	echo "$BFB_DIR is not a directory or does not exist" >&2
	exit 2
fi

function - {
	echo $@
}

cd $solvio_DIR
cargo build --release --bin solvio

cd $BFB_DIR
cargo build --release

cd $solvio_DIR

solvio__LOG_LEVEL=debug,raft=info,segment::common::mmap_ops=trace \
solvio__STORAGE__OPTIMIZERS__MEMMAP_THRESHOLD_KB=1 \
- ./target/release/solvio &

- $BFB_DIR/target/release/bfb -n 1000000 --indexing-threshold 1000000000

- kill %%

solvio__LOG_LEVEL=debug,raft=info,segment::common::mmap_ops=trace \
solvio__STORAGE__OPTIMIZERS__MEMMAP_THRESHOLD_KB=1 \
- ./target/release/solvio &

function search() {
	- time \
		curl localhost:6333/collections/benchmark/points/search \
		-X POST -H 'Content-Type: application/json' --data-raw '{
			"vector": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
			"limit": 10,
			"with_vectors": false,
			"with_payload": true
		}'
}

search
search
