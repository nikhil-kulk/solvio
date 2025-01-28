#!/bin/bash

WORKSPACE_CRATES=$(cargo metadata --format-version 1 | jq -r '.workspace_members[]' | awk -F '/' '{print $NF}' | cut -d '#' -f 1)

for CRATE in $WORKSPACE_CRATES; do
    if [ "$CRATE" == "solvio" ]; then
        continue
    fi

    echo "Testing crate with coverage: $CRATE"
    cargo llvm-cov nextest --profile ci --jobs=1 -p "$CRATE"
done
