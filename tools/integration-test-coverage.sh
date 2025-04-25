#!/bin/bash

# Usage: tools/integration-test-coverage.sh
#
# If using locally, occasionally run `cargo llvm-cov clean` to avoid bloating `target/llvm-cov-target` dir with .profraw files


# Check if target/llvm-cov-target/debug/solvio exists, if not build it:
if [ ! -f target/llvm-cov-target/debug/solvio ]; then
    echo "Building solvio with LLVM coverage instrumentation..."
    RUSTFLAGS="-C instrument-coverage" cargo build --features "service_debug data-consistency-check" --locked --target-dir target/llvm-cov-target
else
    echo "INFO: target/llvm-cov-target/debug/solvio already exists, skipping build step."
fi

export COVERAGE=1

poetry -C tests run ./tests/integration-tests.sh # generates solvio-openapi-*.profraw files
poetry -C tests run ./tests/integration-tests.sh distributed # generates solvio-openapi-*.profraw files
poetry -C tests run pytest tests/consensus_tests --durations=10 # generates solvio-consensus-tests-*.profraw files

# Merges all the .profraw files into a single .profdata file and generates the lcov report
cargo llvm-cov report --lcov --output-path integration-test-coverage.lcov
