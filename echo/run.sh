#!/usr/bin/env zsh

set -euo pipefail

function main {
	declare root=${self:a:h}
	cd $root

	declare nodes=${1:-5}

	for ((node = 0; node < nodes; node++))
	do
		./target/debug/echo --http 127.0.0.1:$((8000 + node)) --grpc 127.0.0.1:$((9000 + node)) &
	done

	trap "trap - TERM && kill -- -$$" INT TERM EXIT

	wait
}

self=$0 main $@
