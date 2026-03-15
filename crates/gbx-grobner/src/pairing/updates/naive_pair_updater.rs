use crate::pairing::filters::{PairFilter, PairSetView};
use crate::pairing::PairCriterion;
use crate::{seed_pairs, GrobnerBasis, PairKey, PairQueue, PairUpdate};

/// Baseline pair-set update strategy for Buchberger's algorithm.
///
/// When a new polynomial `gb[new_index]` is appended to the basis, this strategy
/// seeds all candidate pairs `(i, new_index)` with `0 <= i < new_index` that:
///
/// - are kept by the configured local criterion,
/// - survive the configured state-aware filter,
/// - and have a queue key produced by the configured key strategy.
///
/// This strategy does not attempt to remove or replace existing pairs.
#[derive(Debug, Clone, Copy)]
pub struct NaivePairUpdater<C, F, K> {
    criterion: C,
    filter: F,
    keyer: K,
}

impl<C, F, K> NaivePairUpdater<C, F, K> {
    #[must_use]
    #[inline]
    pub fn new(criterion: C, filter: F, keyer: K) -> Self {
        Self { criterion, filter, keyer }
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
    pub fn filter(&self) -> &F {
        &self.filter
    }

    #[must_use]
    #[inline]
    pub fn filter_mut(&mut self) -> &mut F {
        &mut self.filter
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
    pub fn into_parts(self) -> (C, F, K) {
        (self.criterion, self.filter, self.keyer)
    }
}

impl<C, F, K> Default for NaivePairUpdater<C, F, K>
where
    C: Default,
    F: Default,
    K: Default,
{
    #[inline]
    fn default() -> Self {
        Self { criterion: C::default(), filter: F::default(), keyer: K::default() }
    }
}

impl<P, C, F, K> PairUpdate<P> for NaivePairUpdater<C, F, K>
where
    C: PairCriterion<P>,
    F: PairFilter<P>,
    K: PairKey<P, Key = u32>,
{
    #[inline]
    fn on_new_poly<Q: PairQueue>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize)
    where
        Q: PairSetView,
    {
        seed_pairs(
            gb,
            pairs,
            new_index,
            &mut self.criterion,
            &mut self.filter,
            &mut self.keyer,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pairing::PairCriterion;
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
            todo!()
        }

        fn is_empty(&self) -> bool {
            todo!()
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
    struct KeepAllFilter;

    impl<P> PairFilter<P> for KeepAllFilter {
        fn keep_pair<S: PairSetView>(&mut self, _gb: &GrobnerBasis<P>, _state: &S, _i: usize, _j: usize) -> bool {
            true
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct RejectIndexTwoFilter;

    impl<P> PairFilter<P> for RejectIndexTwoFilter {
        fn keep_pair<S: PairSetView>(&mut self, _gb: &GrobnerBasis<P>, _state: &S, i: usize, _j: usize) -> bool {
            i != 2
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

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, KeepAllFilter, SumKey);
        updater.on_new_poly(&gb, &mut queue, 3);

        assert_eq!(queue.pushed, vec![(3, 0, 3), (4, 1, 3), (5, 2, 3)]);
    }

    #[test]
    fn respects_local_criterion() {
        let gb = empty_gb();
        let mut queue = TestQueue::default();

        let mut updater = NaivePairUpdater::new(RejectIndexOneCriterion, KeepAllFilter, SumKey);
        updater.on_new_poly(&gb, &mut queue, 3);

        assert_eq!(queue.pushed, vec![(3, 0, 3), (5, 2, 3)]);
    }

    #[test]
    fn respects_state_aware_filter() {
        let gb = empty_gb();
        let mut queue = TestQueue::default();

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, RejectIndexTwoFilter, SumKey);
        updater.on_new_poly(&gb, &mut queue, 3);

        assert_eq!(queue.pushed, vec![(3, 0, 3), (4, 1, 3)]);
    }

    #[test]
    fn skips_pairs_when_key_is_missing() {
        let gb = empty_gb();
        let mut queue = TestQueue::default();

        let mut updater = NaivePairUpdater::new(KeepAllCriterion, KeepAllFilter, MissingKeyForZero);
        updater.on_new_poly(&gb, &mut queue, 3);

        assert_eq!(queue.pushed, vec![(4, 1, 3), (5, 2, 3)]);
    }
}
