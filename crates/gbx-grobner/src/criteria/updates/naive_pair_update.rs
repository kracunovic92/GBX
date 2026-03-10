use crate::criteria::{seed_pairs, PairCriterion, PairKey, PairUpdate};
use crate::{GrobnerBasis, PairQueue};

/// Baseline pair-set update strategy for Buchberger's algorithm.
///
/// When a new polynomial `gb[new_index]` is appended to the basis, this update
/// strategy seeds all candidate pairs `(i, new_index)` with `0 <= i < new_index`
/// that:
///
/// - are kept by the configured [`PairCriterion`], and
/// - have a queue key produced by the configured [`PairKey`]
///
/// The actual insertion logic is delegated to [`seed_pairs`].
///
/// # Purpose
///
/// This is the standard baseline update rule for a simple Buchberger engine.
/// It does not attempt to remove or replace existing pairs, and it does not
/// perform any global pair-set minimization.
///
/// # Type parameters
///
/// - `C`: pair-level criterion used to decide whether a candidate pair is kept
/// - `K`: key strategy used to assign queue priority to kept pairs
///
/// # Notes
///
/// This update strategy is:
///
/// - simple
/// - sound
/// - easy to test
/// - a good baseline for future optimizations
///
/// Later, more sophisticated update strategies can be introduced without
/// changing the criterion or key abstractions.
#[derive(Debug, Clone, Copy)]
pub struct NaivePairUpdate<C, K> {
    criterion: C,
    keyer: K,
}

impl<C, K> NaivePairUpdate<C, K> {
    /// Creates a new naive pair-update strategy.
    ///
    /// The provided criterion decides which candidate pairs survive,
    /// and the provided keyer assigns queue keys to surviving pairs.
    #[must_use]
    #[inline]
    pub fn new(criterion: C, keyer: K) -> Self {
        Self { criterion, keyer }
    }

    /// Returns a shared reference to the inner criterion.
    #[must_use]
    #[inline]
    pub fn criterion(&self) -> &C {
        &self.criterion
    }

    /// Returns a mutable reference to the inner criterion.
    #[must_use]
    #[inline]
    pub fn criterion_mut(&mut self) -> &mut C {
        &mut self.criterion
    }

    /// Returns a shared reference to the inner key strategy.
    #[must_use]
    #[inline]
    pub fn keyer(&self) -> &K {
        &self.keyer
    }

    /// Returns a mutable reference to the inner key strategy.
    #[must_use]
    #[inline]
    pub fn keyer_mut(&mut self) -> &mut K {
        &mut self.keyer
    }

    /// Decomposes this update strategy into its parts.
    #[must_use]
    #[inline]
    pub fn into_parts(self) -> (C, K) {
        (self.criterion, self.keyer)
    }
}

impl<C, K> Default for NaivePairUpdate<C, K>
where
    C: Default,
    K: Default,
{
    #[inline]
    fn default() -> Self {
        Self { criterion: C::default(), keyer: K::default() }
    }
}

impl<P, C, K> PairUpdate<P> for NaivePairUpdate<C, K>
where
    C: PairCriterion<P>,
    K: PairKey<P>,
{
    #[inline]
    fn on_new_poly<Q: PairQueue>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize) {
        seed_pairs(gb, pairs, new_index, &mut self.criterion, &mut self.keyer);
    }
}
