SET solvio__GPU__indexing=1
SET solvio__GPU__max_groups=2000
SET solvio__GPU__force_half_precision=0

SET solvio__storage__optimizers__max_optimization_threads=1

cargo run --release
