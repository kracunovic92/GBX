use crate::pairing::updates::{PairUpdateError, Result};
use crate::pairing::PairCriterion;
use crate::{GrobnerBasis, Pair, PairKey, PairQueue, PairUpdate};

/// Baseline pair-set update strategy.
///
/// When a new polynomial `gb[new_index]` is appended to the basis, this strategy
/// generates all candidate pairs `(i, new_index)` with `0 <= i < new_index`
/// that:
///
/// - are kept by the configured local criterion,
/// - and receive a queue key from the configured key strategy.
///
/// This updater does not attempt to remove, replace, or deduplicate existing
/// pending pairs, and it does not inspect the current pair set for state-aware
/// pruning.
#[derive(Debug, Clone, Copy)]
pub struct NaivePairUpdater<C, K> {
    criterion: C,
    keyer: K,
}

impl<C, K> NaivePairUpdater<C, K> {
    /// Create a new naive pair updater from a local criterion and pair keyer.
    #[must_use]
    #[inline]
    pub fn new(criterion: C, keyer: K) -> Self {
        Self { criterion, keyer }
    }

    /// Borrow the local pair criterion.
    #[must_use]
    #[inline]
    pub fn criterion(&self) -> &C {
        &self.criterion
    }

    /// Mutably borrow the local pair criterion.
    #[must_use]
    #[inline]
    pub fn criterion_mut(&mut self) -> &mut C {
        &mut self.criterion
    }

    /// Borrow the pair keyer.
    #[must_use]
    #[inline]
    pub fn keyer(&self) -> &K {
        &self.keyer
    }

    /// Mutably borrow the pair keyer.
    #[must_use]
    #[inline]
    pub fn keyer_mut(&mut self) -> &mut K {
        &mut self.keyer
    }

    /// Consume the updater and return its components.
    #[must_use]
    #[inline]
    pub fn into_parts(self) -> (C, K) {
        (self.criterion, self.keyer)
    }
}

impl<C, K> Default for NaivePairUpdater<C, K>
where
    C: Default,
    K: Default,
{
    #[inline]
    fn default() -> Self {
        Self { criterion: C::default(), keyer: K::default() }
    }
}

impl<P, C, K> PairUpdate<P> for NaivePairUpdater<C, K>
where
    C: PairCriterion<P>,
    K: PairKey<P, Key = u32>,
{
    type Key = u32;

    #[inline]
    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize) -> Result<()>
    where
        Q: PairQueue<Key = Self::Key> + crate::pairing::filters::PairSetView,
    {
        if new_index >= gb.len() {
            return Err(PairUpdateError::InvariantViolation);
        }

        for i in 0..new_index {
            if !self.criterion.keep_pair(gb, i, new_index) {
                continue;
            }

            let Some(key) = self.keyer.key_for_pair(gb, i, new_index) else {
                continue;
            };

            pairs.push(Pair::new(key, i, new_index));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pairing::PairCriterion;
    use crate::FifoPairs;
    use gbx_poly::order::Lex;
    use gbx_poly::ring::{Ring, StaticFpCtx};

    #[derive(Debug, Default, Clone, Copy)]
    struct KeepAllCriterion;

    impl<P> PairCriterion<P> for KeepAllCriterion {
        fn keep_pair(&mut self, _gb: &GrobnerBasis<P>, _i: usize, _j: usize) -> bool {
            true
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct RejectIndexOneCriterion;

    impl<P> PairCriterion<P> for RejectIndexOneCriterion {
        fn keep_pair(&mut self, _gb: &GrobnerBasis<P>, i: usize, _j: usize) -> bool {
            i != 1
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct SumKey;

    impl<P> PairKey<P> for SumKey {
        type Key = u32;

        fn key_for_pair(&mut self, _gb: &GrobnerBasis<P>, i: usize, j: usize) -> Option<Self::Key> {
            Some((i + j) as u32)
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct MissingKeyForZero;

    impl<P> PairKey<P> for MissingKeyForZero {
        type Key = u32;

        fn key_for_pair(&mut self, _gb: &GrobnerBasis<P>, i: usize, j: usize) -> Option<Self::Key> {
            if i == 0 || j == 0 { None } else { Some((i + j) as u32) }
        }
    }

    fn empty_gb() -> GrobnerBasis<()> {
        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        GrobnerBasis::new(ring.id(), vec![(), (), (), ()])
    }

    fn drain_pairs<Q>(queue: &mut Q) -> Vec<Pair<u32>>
    where
        Q: PairQueue<Key = u32>,
    {
        let mut out = Vec::new();
        while let Some(pair) = queue.pop() {
            out.push(pair);
        }
        out
    }

    #[test]
    fn seeds_all_surviving_pairs() {
        let gb = empty_gb();
        let mut queue = FifoPairs::new();

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, SumKey);
        updater.on_new_poly(&gb, &mut queue, 3).unwrap();

        assert_eq!(
            drain_pairs(&mut queue),
            vec![Pair::new(3, 0, 3), Pair::new(4, 1, 3), Pair::new(5, 2, 3),]
        );
    }

    #[test]
    fn respects_local_criterion() {
        let gb = empty_gb();
        let mut queue = FifoPairs::new();

        let mut updater = NaivePairUpdater::new(RejectIndexOneCriterion, SumKey);
        updater.on_new_poly(&gb, &mut queue, 3).unwrap();

        assert_eq!(
            drain_pairs(&mut queue),
            vec![Pair::new(3, 0, 3), Pair::new(5, 2, 3),]
        );
    }

    #[test]
    fn skips_pairs_when_key_is_missing() {
        let gb = empty_gb();
        let mut queue = FifoPairs::new();

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, MissingKeyForZero);
        updater.on_new_poly(&gb, &mut queue, 3).unwrap();

        assert_eq!(
            drain_pairs(&mut queue),
            vec![Pair::new(4, 1, 3), Pair::new(5, 2, 3),]
        );
    }

    #[test]
    fn rejects_out_of_range_new_index() {
        let gb = empty_gb();
        let mut queue = FifoPairs::new();

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, SumKey);
        let err = updater.on_new_poly(&gb, &mut queue, gb.len()).unwrap_err();

        assert!(matches!(err, PairUpdateError::InvariantViolation));
    }

    #[test]
    fn seed_pairs_replays_incremental_insertion_order() {
        let gb = empty_gb();
        let mut queue = FifoPairs::new();

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, SumKey);
        updater.seed_pairs(&gb, &mut queue).unwrap();

        assert_eq!(
            drain_pairs(&mut queue),
            vec![Pair::new(1, 0, 1), Pair::new(2, 0, 2), Pair::new(3, 1, 2), Pair::new(3, 0, 3), Pair::new(4, 1, 3), Pair::new(5, 2, 3),]
        );
    }
}
