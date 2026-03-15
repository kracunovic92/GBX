//! Pair queue key strategies.
//!
//! This module contains [`PairKey`], which assigns queue priorities to
//! Buchberger pairs `(i, j)`.
//!
//! Keys affect processing order and therefore performance, but they should
//! not affect correctness as long as the surrounding pair-update logic is sound.

use crate::GrobnerBasis;

mod constant_pair_key;
mod lcm_degree_key;

pub use constant_pair_key::*;
pub use lcm_degree_key::*;

/// Queue key computation for a pair.
///
/// Keys are used by pair-update strategies to assign queue priorities to
/// candidate pairs.
///
/// # Return value
///
/// - `Some(key)` if a key could be computed
/// - `None` if the pair should be skipped due to missing data or arithmetic
///   failure such as overflow
///
/// Returning `None` is preferred over panicking in generic infrastructure.
pub trait PairKey<P> {
    /// Queue key type.
    ///
    /// This must be orderable because pair queues use it for priority.
    type Key: Ord;

    /// Compute the queue key for pair `(i, j)`.
    fn key_for_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> Option<Self::Key>;
}
