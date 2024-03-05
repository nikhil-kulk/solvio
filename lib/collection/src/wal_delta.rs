use std::fmt::Debug;
use std::sync::Arc;

use parking_lot::Mutex as ParkingMutex;
use thiserror::Error;
use tokio::sync::Mutex;

use crate::operations::OperationWithClockTag;
use crate::shards::local_shard::clock_map::{ClockMap, RecoveryPoint};
use crate::wal::SerdeWal;

pub type LockedWal = Arc<ParkingMutex<SerdeWal<OperationWithClockTag>>>;

/// A WAL that is recoverable, with operations having clock tags and a corresponding clock map.
pub struct RecoverableWal {
    pub(super) wal: LockedWal,
    /// Map of all last seen clocks for each peer and clock ID.
    pub(super) last_clocks: Arc<Mutex<ClockMap>>,
}

impl RecoverableWal {
    pub fn from(wal: LockedWal, last_clocks: Arc<Mutex<ClockMap>>) -> Self {
        Self { wal, last_clocks }
    }

    /// Write a record to the WAL but does guarantee durability.
    pub async fn lock_and_write(
        &self,
        operation: &mut OperationWithClockTag,
    ) -> crate::wal::Result<u64> {
        // Update last seen clock map and correct clock tag if necessary
        if let Some(clock_tag) = &mut operation.clock_tag {
            // TODO:
            //
            // Temporarily accept *all* operations, even if their `clock_tag` is older than
            // current clock tracked by the clock map

            // TODO: do not manually advance here!
            let _operation_accepted = self
                .last_clocks
                .lock()
                .await
                .advance_clock_and_correct_tag(clock_tag);

            // if !operation_accepted {
            //     return Ok(UpdateResult {
            //         operation_id: None,
            //         status: UpdateStatus::Acknowledged,
            //         clock_tag: Some(*clock_tag),
            //     });
            // }
        }

        // Write operation to WAL
        self.wal.lock().write(operation)
    }

    /// Get a recovery point for this WAL.
    pub async fn recovery_point(&self) -> RecoveryPoint {
        self.last_clocks.lock().await.to_recovery_point()
    }

    pub async fn resolve_wal_delta(
        &self,
        recovery_point: RecoveryPoint,
    ) -> Result<u64, WalDeltaError> {
        resolve_wal_delta(recovery_point, &self.wal, self.recovery_point().await)
    }
}

/// Resolve the WAL delta for the given `recovery_point`
///
/// A `local_wal` and `local_last_seen` are required to resolve the delta. These should be from the
/// node being the source of recovery, likely the current one. The `local_wal` is used to resolve
/// the diff. The `local_last_seen` is used to extend the given recovery point with clocks the
/// failed node does not know about.
///
/// The delta can be sent over to the node which the recovery point is from, to restore its
/// WAL making it consistent with the current shard.
///
/// On success, a WAL record number from which the delta is resolved in the given WAL is returned.
/// If a WAL delta could not be resolved, an error is returned describing the failure.
fn resolve_wal_delta(
    mut recovery_point: RecoveryPoint,
    local_wal: &LockedWal,
    local_recovery_point: RecoveryPoint,
) -> Result<u64, WalDeltaError> {
    if recovery_point.is_empty() {
        return Err(WalDeltaError::Empty);
    }

    // If our current node has any lower clock than the recovery point specifies,
    // we're missing essential operations and cannot resolve a WAL delta
    if recovery_point.has_any_higher(&local_recovery_point) {
        return Err(WalDeltaError::HigherThanCurrent);
    }

    // Extend clock map with missing clocks this node know about
    // Ensure the recovering node gets records for a clock it might not have seen yet
    recovery_point.extend_with_missing_clocks(&local_recovery_point);

    // Remove clocks that are equal to this node, we don't have to transfer records for them
    // TODO: do we want to remove higher clocks too, as the recovery node already has all data?
    recovery_point.remove_equal_clocks(&local_recovery_point);

    // TODO: check truncated clock values or each clock we have:
    // TODO: - if truncated is higher, we cannot resolve diff

    // Scroll back over the WAL and find a record that covered all clocks
    // Drain satisfied clocks from the recovery point until we have nothing left
    log::trace!("Resolving WAL delta for: {recovery_point}");
    let delta_from = local_wal
        .lock()
        .read_from_last(true)
        .filter_map(|(op_num, update)| update.clock_tag.map(|clock_tag| (op_num, clock_tag)))
        // Keep scrolling until we have no clocks left
        .find(|(_, clock_tag)| {
            recovery_point.remove_equal_or_lower(*clock_tag);
            recovery_point.is_empty()
        })
        .map(|(op_num, _)| op_num);

    delta_from.ok_or(WalDeltaError::NotFound)
}

#[derive(Error, Debug, Clone)]
#[error("cannot resolve WAL delta: {0}")]
pub enum WalDeltaError {
    #[error("recovery point has no clocks to resolve delta for")]
    Empty,
    #[error("recovery point has higher clocks than current WAL")]
    HigherThanCurrent,
    #[error("some recovery point clocks are truncated in our WAL")]
    Truncated,
    #[error("some recovery point clocks are not found in our WAL")]
    NotFound,
}
