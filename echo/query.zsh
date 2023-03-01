#!/usr/bin/env zsh

set -euo pipefail

function main {
	declare -A opts=(
		[nodes]=5
		[local]=1
		[docker]=''
	)

	while (( $# ))
	do
		case $1 in
			--nodes)
				opts[nodes]=$2
				shift 2
			;;

			--local)
				opts[local]=1
				opts[docker]=''
				shift
			;;

			--docker)
				opts[docker]=1
				opts[local]=''
				shift
			;;

			*)
				echo "ERROR: Unexpected argument '$1'" >&2
				return 1
			;;
		esac
	done

	declare addrs=()

	for ((node = 0; node < opts[nodes]; node++))
	do
		if [[ -v opts[local] && -n $opts[local] ]]
		then
			addrs+=( 127.0.0.1:$((9000 + node)) )
		else
			addrs+=( 69.42.0.$((1 + node)):8081 )
		fi
	done

	declare data=${(j:, :)${:-\"${^addrs}\"}}

	for (( node = 0; node < opts[nodes]; node++))
	do
		curl -sS 127.0.0.1:$((8000 + node)) \
			-X POST \
			-H 'Content-Type: application/json' \
			--data-raw "{\"nodes\": [$data]}" | jq -Rr '. as $line | (fromjson? | length) // $line'
	done
}

main $@
