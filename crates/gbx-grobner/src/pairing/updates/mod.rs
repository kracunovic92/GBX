//! Pair-set update strategies.
//!
//! Update strategies are responsible for introducing new critical pairs after a
//! new polynomial is appended to the Gröbner basis.
//!
//! # Architecture
//!
//! An update strategy combines insertion-time decisions such as:
//!
//! - a local [`PairCriterion`](crate::pairing::PairCriterion),
//! - a [`PairKey`](crate::PairKey) used for queue priority,
//! - optional conservative pruning among the newly generated pairs.
//!
//! More global state-aware pruning can still live in a separate pop-time
//! [`PairFilter`](crate::pairing::filters::PairFilter) layer.

use crate::pairing::filters::PairSetView;
use crate::{GrobnerBasis, PairQueue};
use thiserror::Error;

mod gm_pair_updater;
mod gm_product_lcm_degree;
mod gm_product_lcm_degree_trace;
mod naive_pair_updater;

pub use gm_pair_updater::*;
pub use gm_product_lcm_degree::GmProductLcmDegreeUpdater;
pub use naive_pair_updater::*;

/// Result type used by pair-update strategies.
pub type Result<T> = core::result::Result<T, PairUpdateError>;

/// Errors that can occur while updating the active critical-pair set.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PairUpdateError {
    /// Internal invariant broken, for example an out-of-range basis index.
    #[error("pair-update invariant violation")]
    InvariantViolation,
}

/// Pair-set update hook.
///
/// This trait is shared by Gröbner basis engines that maintain a pending set of
/// critical pairs. It is called whenever a new basis polynomial has been
/// accepted and appended to the current Gröbner basis.
pub trait PairUpdate<P> {
    type Key;
    /// Update the active pair set after `gb[new_index]` was inserted.
    ///
    /// # Contract
    ///
    /// - `new_index < gb.len()`
    /// - `gb[new_index]` is the newly accepted basis element
    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize) -> Result<()>
    where
        Q: PairQueue<Key = Self::Key> + PairSetView;

    /// Seed the initial pair set from an already-built basis.
    ///
    /// The default implementation replays [`Self::on_new_poly`] for every basis
    /// index in insertion order. This matches the usual interpretation that the
    /// basis was built incrementally from indices `0..gb.len()`.
    fn seed_pairs<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q) -> Result<()>
    where
        Q: PairQueue<Key = Self::Key> + PairSetView,
    {
        for j in 0..gb.len() {
            self.on_new_poly(gb, pairs, j)?;
        }
        Ok(())
    }
}
