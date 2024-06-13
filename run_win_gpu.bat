SET solvio__GPU__indexing=1
SET solvio__GPU__max_warps=512
SET solvio__GPU__force_half_precision=0

cargo run --release
