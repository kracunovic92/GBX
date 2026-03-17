#![allow(missing_docs)]
use crate::pairing::PairCriterion;
use crate::{GrobnerBasis, PairKey, PairQueue, PairUpdate};

/// Baseline pair-set update strategy for Buchberger's algorithm.
///
/// When a new polynomial `gb[new_index]` is appended to the basis, this strategy
/// seeds all candidate pairs `(i, new_index)` with `0 <= i < new_index` that:
///
/// - are kept by the configured local criterion,
/// - and have a queue key produced by the configured key strategy.
///
/// This strategy does not attempt to remove or replace existing pairs, and does
/// not perform any state-aware pruning based on the current pending-pair set.
#[derive(Debug, Clone, Copy)]
pub struct NaivePairUpdater<C, K> {
    criterion: C,
    keyer: K,
}

impl<C, K> NaivePairUpdater<C, K> {
    #[must_use]
    #[inline]
    pub fn new(criterion: C, keyer: K) -> Self {
        Self { criterion, keyer }
    }

    #[must_use]
    #[inline]
    pub fn criterion(&self) -> &C {
        &self.criterion
    }

    #[must_use]
    #[inline]
    pub fn criterion_mut(&mut self) -> &mut C {
        &mut self.criterion
    }

    #[must_use]
    #[inline]
    pub fn keyer(&self) -> &K {
        &self.keyer
    }

    #[must_use]
    #[inline]
    pub fn keyer_mut(&mut self) -> &mut K {
        &mut self.keyer
    }

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
    #[inline]
    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize)
    where
        Q: PairQueue + crate::PairSetView,
    {
        for i in 0..new_index {
            if !self.criterion.keep_pair(gb, i, new_index) {
                continue;
            }

            let Some(key) = self.keyer.key_for_pair(gb, i, new_index) else {
                continue;
            };

            pairs.push((key, i, new_index));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pairing::PairCriterion;
    use crate::GrobnerBasis;
    use crate::PairSetView;
    use gbx_poly::order::Lex;
    use gbx_poly::ring::{Ring, StaticFpCtx};

    #[derive(Debug, Default)]
    struct TestQueue {
        pushed: Vec<(u32, usize, usize)>,
    }

    impl PairQueue for TestQueue {
        fn new() -> Self
        where
            Self: Sized,
        {
            Self::default()
        }

        fn is_empty(&self) -> bool {
            self.pushed.is_empty()
        }

        fn push(&mut self, pair: (u32, usize, usize)) {
            self.pushed.push(pair);
        }

        fn pop(&mut self) -> Option<(u32, usize, usize)> {
            self.pushed.pop()
        }

        fn len(&self) -> usize {
            self.pushed.len()
        }
    }

    impl PairSetView for TestQueue {
        fn contains_pair(&self, i: usize, j: usize) -> bool {
            let ij = if i < j { (i, j) } else { (j, i) };
            self.pushed.iter().any(|&(_, a, b)| {
                let ab = if a < b { (a, b) } else { (b, a) };
                ab == ij
            })
        }
    }

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

    #[test]
    fn seeds_all_surviving_pairs() {
        let gb = empty_gb();
        let mut queue = TestQueue::default();

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, SumKey);
        updater.on_new_poly(&gb, &mut queue, 3);

        assert_eq!(queue.pushed, vec![(3, 0, 3), (4, 1, 3), (5, 2, 3)]);
    }

    #[test]
    fn respects_local_criterion() {
        let gb = empty_gb();
        let mut queue = TestQueue::default();

        let mut updater = NaivePairUpdater::new(RejectIndexOneCriterion, SumKey);
        updater.on_new_poly(&gb, &mut queue, 3);

        assert_eq!(queue.pushed, vec![(3, 0, 3), (5, 2, 3)]);
    }

    #[test]
    fn skips_pairs_when_key_is_missing() {
        let gb = empty_gb();
        let mut queue = TestQueue::default();

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, MissingKeyForZero);
        updater.on_new_poly(&gb, &mut queue, 3);

        assert_eq!(queue.pushed, vec![(4, 1, 3), (5, 2, 3)]);
    }
}
