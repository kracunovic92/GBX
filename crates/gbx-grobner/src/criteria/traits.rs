//! Core traits for Buchberger pair management.
//!
//! This module separates three different responsibilities that are often
//! mixed together in simpler Gröbner-basis implementations:
//!
//! - [`PairCriterion`]: decides whether a pair should be kept at all
//! - [`PairKey`]: computes the queue priority / ordering key for a pair
//! - [`PairUpdate`]: updates the active pair set when a new polynomial is added

use crate::{GrobnerBasis, PairQueue};

/// Pair-level elimination criterion.
/// Returning `true` means the pair survives the criterion and may be inserted
/// into the queue. Returning `false` means the pair is discarded.
pub trait PairCriterion<P> {
    /// Return `true` if pair `(i, j)` should be kept.
    fn keep_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> bool;
}

/// Queue key computation for a pair.
///
/// # Return value
///
/// - `Some(key)` if a key could be computed
/// - `None` if the pair should be skipped due to missing data or arithmetic
///   failure such as overflow
///
/// Returning `None` is preferred over panicking in generic infrastructure.
pub trait PairKey<P> {
    /// Compute the queue key for pair `(i, j)`.
    fn key_for_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> Option<u32>;
}

/// Pair-set update strategy.
///
/// This trait is responsible for updating the active pair set after a new
/// polynomial has been appended to the basis.
pub trait PairUpdate<P> {
    /// Update the active pair set after `gb[new_index]` was appended.
    fn on_new_poly<Q: PairQueue>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize);
}
