//! Pair filters for Buchberger-style algorithms.
//!
//! A [`PairFilter`] is a pop-time pre-reduction check: it runs after a pair is
//! selected from the active pair set and decides whether the expensive
//! S-polynomial reduction should actually be performed.
//!
//! Filters may be:
//!
//! - purely local, depending only on the basis and pair `(i, j)`,
//! - or state-aware, depending also on the current active pair set.
//!
//! # Important
//!
//! Not every theorem-style Buchberger criterion is automatically sound in this
//! layer. In particular, filters that rely on the exact meaning of the pending
//! pair set require that the engine's pair-state semantics match the theorem's
//! assumptions.

use crate::GrobnerBasis;

/// Read-only view of active pair-state needed by pair filters.
pub trait PairSetView {
    /// Returns `true` if pair `(i, j)` is currently active/pending.
    ///
    /// Implementations should treat `(i, j)` and `(j, i)` as the same pair.
    fn contains_pair(&self, i: usize, j: usize) -> bool;
}

/// Pop-time pair filter.
///
/// Returning `true` means the pair survives the filter and should be processed.
/// Returning `false` means the pair is skipped before S-polynomial reduction.
pub trait PairFilter<P> {
    /// Return `true` if pair `(i, j)` should be processed.
    fn keep_pair<S: PairSetView>(&mut self, gb: &GrobnerBasis<P>, state: &S, i: usize, j: usize) -> bool;
}

mod and_pair_filter;
mod chain_pair_filter;
mod no_pair_filter;
mod product_pair_filter;

pub use and_pair_filter::*;
pub use chain_pair_filter::*;
pub use no_pair_filter::*;
pub use product_pair_filter::*;
