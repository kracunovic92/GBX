//! Local pair-elimination pairing for Buchberger-style algorithms.
//!
//! This module contains implementations of [`PairCriterion`], a lightweight
//! interface for pair tests that depend only on the current basis and the
//! candidate pair `(i, j)`.
//!
//! Convention:
//! - `true`  => keep the pair
//! - `false` => eliminate the pair
//!
//! # Scope
//!
//! These pairing are intentionally **local**.
//! Criteria that depend on the state of other active, processed, or pending
//! pairs do not belong here and should live in a richer pair-filter or
//! pair-update layer instead.

use crate::GrobnerBasis;

mod and_criterion;
mod no_pair_criterion;
mod product_criterion;

pub use and_criterion::*;
pub use no_pair_criterion::*;
pub use product_criterion::*;

/// Local pair-elimination criterion.
///
/// Returning `true` means the pair survives the criterion and may be inserted
/// into the queue. Returning `false` means the pair is discarded.
///
/// Implementations should be:
///
/// - **sound**: they must not eliminate pairs required for correctness
/// - **cheap**: they are intended to avoid expensive S-polynomial reductions
/// - **local**: they should depend only on `gb`, `i`, and `j`
pub trait PairCriterion<P> {
    /// Return `true` if pair `(i, j)` should be kept.
    fn keep_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> bool;
}
