use std::time::Duration;

/// Default timeout for consensus meta operations.
pub const CONSENSUS_META_OP_WAIT: Duration = Duration::from_secs(10);

/// Default value of CPU budget parameter.
///
/// Dynamic based on CPU size.
///
/// On low CPU systems, we want to reserve the minimal amount of CPUs for other tasks to allow
/// efficient optimization. On high CPU systems we want to reserve more CPUs.
#[inline(always)]
pub fn default_cpu_budget_unallocated(num_cpu: usize) -> isize {
    match num_cpu {
        0..=2 => 0,
        3..=32 => -1,
        33..=48 => -2,
        49..=64 => -3,
        65..=96 => -4,
        97..=128 => -6,
        num_cpu @ 129.. => -(num_cpu as isize / 16),
    }
}

/// Default number of CPUs for HNSW graph building and optimization tasks in general.
///
/// Dynamic based on CPU size.
///
/// Even on high-CPU systems, a value higher than 16 is discouraged. It will most likely not
/// improve performance and is more likely to cause disconnected HNSW graphs.
/// Will be less if currently available CPU budget is lower.
#[inline(always)]
pub fn thread_count_for_hnsw(num_cpu: usize) -> usize {
    match num_cpu {
        0..=48 => 8.min(num_cpu).max(1),
        49..=64 => 12,
        65.. => 16,
    }
}
