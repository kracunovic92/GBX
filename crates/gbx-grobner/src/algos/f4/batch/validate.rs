use crate::algos::f4::batch::types::PairBatch;

#[cfg(debug_assertions)]
use std::collections::HashSet;

/// Debug-only validation for a pair batch.
///
/// This is intentionally conservative and only used to catch obvious bugs
/// during development.
pub fn debug_validate_batch<K>(batch: &PairBatch<K>) {
    #[cfg(debug_assertions)]
    {
        let mut seen = HashSet::new();

        for pair in batch.as_slice() {
            debug_assert!(pair.i != pair.j, "critical pair must satisfy i != j");
            debug_assert!(
                seen.insert((pair.i, pair.j)),
                "duplicate critical pair found in selected batch"
            );
        }
    }
}
