#!/bin/bash

PACKAGES=($(cargo metadata --format-version 1 | jq -r '.workspace_members[] | split("/") | .[-1] | split("#")[0]' | sort))
EXPECTED_PACKAGES=("api" "blob_store" "cancel" "collection" "common" "dataset" "gpu" "io" "issues" "memory" "solvio" "quantization" "segment" "sparse" "storage")

if [ "${PACKAGES[*]}" != "${EXPECTED_PACKAGES[*]}" ]; then
    echo "Workspace packages have changed. Please update the expected and whitelisted packages in coverage.sh"
    exit 1
fi

WHITELISTED_PACKAGES=("solvio" "api" "blob_store" "collection" "common" "segment" "storage")
# EMPTY_PACKAGES=("cancel" "dataset" "gpu" "io")
# IGNORED_PACKAGES=("issues" "memory" "quantization" "sparse")

REPORT_DIR="target/llvm-cov/package-reports"

echo "All workspace packages: ${PACKAGES[*]}"
echo "Whitelisted workspace packages: ${WHITELISTED_PACKAGES[*]}"

mkdir -p "$REPORT_DIR"

LCOV_COMMAND_ARGS=""

for PACKAGE in "${WHITELISTED_PACKAGES[@]}"; do
    echo "Testing PACKAGE with coverage: $PACKAGE"
    # Profile "ci" is configured in .config/nextest.toml
    cargo llvm-cov --no-clean nextest --profile ci -p "$PACKAGE" --lcov --output-path "$REPORT_DIR/$PACKAGE.info"

    LCOV_COMMAND_ARGS="${LCOV_COMMAND_ARGS} -a $REPORT_DIR/$PACKAGE.info"
done

if [ -n "$LCOV_COMMAND_ARGS" ]; then
    lcov $LCOV_COMMAND_ARGS --output-file lcov.info
fi
