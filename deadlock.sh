#!/bin/zsh

trap 'exit $?' TERM INT ERR

function main {
	declare file; file=$(mktemp -u)

	trap 'rm -f $file' EXIT

	mkfifo $file
	exec {pipe}<>$file
	rm $file

	trap kill-jobs EXIT

	{
		trap 'exit $?' TERM INT ERR
		trap kill-jobs EXIT
		search-collections $pipe
	} &

	create-collections $pipe
}

function kill-jobs {
	if (( ${#jobstates} > 0 ))
	then
		kill ${${jobstates##*:*:}%=*}
	fi
}

function create-collections {
	declare pipe=$1

	declare collections_len=170
	declare collection_index=1
	declare initialized_all=0

	while true
	do
		if (( !initialized_all ))
		then
			echo $collection_index >&$pipe
		fi

		# http PUT localhost:6333/collections/benchmark-$collection_index \
		# 	'vectors[size]':=1280 'vectors[distance]'=Cosine \
		# 	'hnsw_config[on_disk]':=true on_disk_payload:=true \
		# 	>/dev/null

		../solvio-bfb/target/release/bfb \
			--collection-name benchmark-$collection_index -d 1280 -n 10000 \
			--on-disk-hnsw --on-disk-payload \
			--skip-wait-index

		if (( collection_index < collections_len ))
		then
			(( collection_index++ ))
		else
			collection_index=1
			initialized_all=1
		fi
	done
}

function search-collections {
	declare pipe=$1

	declare collections_len=0

	read -u $pipe collections_len

	while true
	do
		while read -u $pipe -t collections_len
		do
			:
		done

		while (( ${#jobstates} < 10 ))
		do
			(( collection_index = RANDOM % collections_len + 1 ))
			search-collection $collection_index >/dev/null &
		done

		sleep 0.1
	done
}

function search-collection {
	declare collection_index=$1

	http POST localhost:6333/collections/benchmark-$collection_index/points/search \
		vector:="[$(random-vector)]" \
		limit:=100 \
		--ignore-stdin
}

function random-vector {
	printf '0.%05d' $RANDOM

	for _ in $(seq 1279)
	do
		printf ', 0.%05d' $RANDOM
	done

	echo
}

function random-float {
	printf '0.%05d' $RANDOM
}

main $@
