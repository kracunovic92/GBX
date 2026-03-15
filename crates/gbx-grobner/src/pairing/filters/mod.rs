//! Pair-state-aware Buchberger filters.
//!
//! Unlike local pair pairing, filters in this module may depend on the state
//! of other candidate pairs. This makes them suitable for chain-style pruning
//! and related Buchberger improvements.

use crate::GrobnerBasis;

/// Read-only view of pair-state needed by pair filters.
pub trait PairSetView {
    /// Returns `true` if pair `(i, j)` is currently active/pending.
    ///
    /// Implementations should treat `(i, j)` and `(j, i)` as the same pair.
    fn contains_pair(&self, i: usize, j: usize) -> bool;
}

/// Pair-state-aware elimination filter.
///
/// Returning `true` means the pair survives the filter.
/// Returning `false` means the pair is discarded.
pub trait PairFilter<P> {
    /// Return `true` if pair `(i, j)` should be kept.
    fn keep_pair<S: PairSetView>(&mut self, gb: &GrobnerBasis<P>, state: &S, i: usize, j: usize) -> bool;
}

mod and_pair_filter;
mod chain_pair_filter;
mod no_pair_filter;

pub use and_pair_filter::*;
pub use chain_pair_filter::*;
pub use no_pair_filter::*;
