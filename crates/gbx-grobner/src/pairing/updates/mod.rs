//! Pair-set update strategies.
//!
//! Update strategies are responsible for introducing new critical pairs after a
//! new polynomial is appended to the Gröbner basis.
//!
//! # Architecture
//!
//! An update strategy combines two insertion-time decisions:
//!
//! - a local [`PairCriterion`](crate::pairing::PairCriterion)
//! - a [`PairKey`](crate::keys::PairKey) for queue priority
//!
//! In addition, the strategy itself decides how newly generated pairs are
//! pruned and inserted into the queue.
//!
//! State-aware pruning that depends on the current pending-pair set does **not**
//! belong here. Such logic should live in the pop-time
//! [`PairFilter`](crate::pairing::filters::PairFilter) layer used by the
//! Buchberger engine.

use crate::{GrobnerBasis, PairQueue};

mod gm_pair_updater;
mod naive_pair_updater;

use crate::pairing::filters::PairSetView;
pub use gm_pair_updater::*;
pub use naive_pair_updater::*;

/// Pair-set update hook.
///
/// Called whenever a new polynomial has just been appended to the Gröbner basis.
pub trait PairUpdate<P> {
    /// Update the active pair set after `gb[new_index]` was inserted.
    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize)
    where
        Q: PairQueue + PairSetView;
}
