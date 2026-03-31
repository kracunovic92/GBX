use crate::pairing::updates::{PairUpdateError, Result};
use crate::pairing::{leading_mono_at, PairCriterion};
use crate::{GrobnerBasis, Pair, PairKey, PairQueue, PairUpdate};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Conservative Gebauer–Möller-style pair updater.
///
/// This strategy improves on [`NaivePairUpdater`](crate::pairing::updates::NaivePairUpdater)
/// by pruning newly generated candidate pairs `(i, new_index)` in two stages:
///
/// 1. deduplicate candidates with the same LCM, keeping a single representative,
/// 2. discard candidates whose LCM is strictly divisible by another surviving
///    candidate LCM for the same `new_index`.
///
/// Before those GM-style pruning steps, candidate pairs are filtered by:
///
/// - a local [`PairCriterion`],
/// - and a [`PairKey`] computation.
///
/// This updater only reasons about the freshly generated candidates associated
/// with the newly inserted basis element. It does not perform broader
/// state-aware pruning against the existing pending-pair set.
#[derive(Debug, Clone, Copy)]
pub struct GmPairUpdater<C, K> {
    criterion: C,
    keyer: K,
}

impl<C, K> GmPairUpdater<C, K> {
    /// Create a new GM-style pair updater from a local criterion and pair keyer.
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

impl<C, K> Default for GmPairUpdater<C, K>
where
    C: Default,
    K: Default,
{
    #[inline]
    fn default() -> Self {
        Self { criterion: C::default(), keyer: K::default() }
    }
}

/// Newly generated candidate pair associated with a fixed `new_index`.
///
/// The candidate stores:
///
/// - the older basis index `i`,
/// - the queue key that would be used for insertion,
/// - the LCM of leading monomials used for GM-style pruning.
#[derive(Debug, Clone)]
struct Candidate<M> {
    i: usize,
    key: u32,
    lcm: M,
}

impl<M> Candidate<M> {
    /// Return whether `self` is preferred over `other` when both represent the
    /// same LCM. The tie-breaker is lexicographic on `(key, i)`.
    #[inline]
    fn better_than(&self, other: &Self) -> bool {
        (self.key, self.i) < (other.key, other.i)
    }
}

impl<P, C, K> PairUpdate<P> for GmPairUpdater<C, K>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    C: PairCriterion<P>,
    K: PairKey<P, Key = u32>,
{
    type Key = u32;

    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize) -> Result<()>
    where
        Q: PairQueue<Key = Self::Key> + crate::pairing::filters::PairSetView,
    {
        if new_index >= gb.len() {
            return Err(PairUpdateError::InvariantViolation);
        }

        let Some(lm_new) = leading_mono_at(gb, new_index) else {
            return Ok(());
        };

        let mut candidates: Vec<Candidate<<P::Term as TermView>::Mono>> = Vec::new();

        for i in 0..new_index {
            if !self.criterion.keep_pair(gb, i, new_index) {
                continue;
            }

            let Some(key) = self.keyer.key_for_pair(gb, i, new_index) else {
                continue;
            };

            let Some(lm_i) = leading_mono_at(gb, i) else {
                continue;
            };

            let Ok(lcm) = lm_i.checked_lcm(lm_new) else {
                continue;
            };

            candidates.push(Candidate { i, key, lcm });
        }

        if candidates.is_empty() {
            return Ok(());
        }

        let mut deduped: Vec<Candidate<<P::Term as TermView>::Mono>> = Vec::new();

        'outer: for cand in candidates {
            for existing in &mut deduped {
                if existing.lcm == cand.lcm {
                    if cand.better_than(existing) {
                        *existing = cand;
                    }
                    continue 'outer;
                }
            }
            deduped.push(cand);
        }

        if deduped.len() <= 1 {
            for cand in deduped {
                pairs.push(Pair::new(cand.key, cand.i, new_index));
            }
            return Ok(());
        }

        let mut keep = vec![true; deduped.len()];

        for a in 0..deduped.len() {
            if !keep[a] {
                continue;
            }

            for b in 0..deduped.len() {
                if a == b || !keep[b] {
                    continue;
                }

                let la = &deduped[a].lcm;
                let lb = &deduped[b].lcm;

                let b_divides_a = matches!(la.checked_div_by(lb), Ok(Some(_)));
                let a_divides_b = matches!(lb.checked_div_by(la), Ok(Some(_)));

                if b_divides_a && !a_divides_b {
                    keep[a] = false;
                    break;
                }
            }
        }

        for (cand, keep_it) in deduped.into_iter().zip(keep.into_iter()) {
            if keep_it {
                pairs.push(Pair::new(cand.key, cand.i, new_index));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pairing::PairCriterion;
    use crate::FifoPairs;
    use gbx_field::fp::Fp;
    use gbx_poly::monomial::DynamicMonomial;
    use gbx_poly::order::Lex;
    use gbx_poly::polynomial::Polynomial;
    use gbx_poly::ring::{Ring, RingCtx, StaticFpCtx};
    use gbx_poly::term::Term;
    use gbx_storage::polynomial::VecTerms;

    type F7 = Fp<7>;
    type Mono = DynamicMonomial;
    type T = Term<F7, Mono>;
    type P = Polynomial<T, VecTerms<T>>;

    #[derive(Debug, Default, Clone, Copy)]
    struct KeepAllCriterion;

    impl<P0> PairCriterion<P0> for KeepAllCriterion {
        fn keep_pair(&mut self, _gb: &GrobnerBasis<P0>, _i: usize, _j: usize) -> bool {
            true
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct RejectIndexOneCriterion;

    impl<P0> PairCriterion<P0> for RejectIndexOneCriterion {
        fn keep_pair(&mut self, _gb: &GrobnerBasis<P0>, i: usize, _j: usize) -> bool {
            i != 1
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct IndexKey;

    impl<P0> PairKey<P0> for IndexKey {
        type Key = u32;

        fn key_for_pair(&mut self, _gb: &GrobnerBasis<P0>, i: usize, _j: usize) -> Option<Self::Key> {
            Some(i as u32)
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct MissingKeyForZero;

    impl<P0> PairKey<P0> for MissingKeyForZero {
        type Key = u32;

        fn key_for_pair(&mut self, _gb: &GrobnerBasis<P0>, i: usize, _j: usize) -> Option<Self::Key> {
            if i == 0 { None } else { Some(i as u32) }
        }
    }

    fn ring() -> RingCtx<StaticFpCtx<7>, Lex> {
        Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(3)
            .build()
            .unwrap()
    }

    fn mono(exps: &[u32]) -> Mono {
        DynamicMonomial::from_slice(exps)
    }

    fn term(c: u32, exps: &[u32]) -> T {
        Term::new(F7::new(c), mono(exps))
    }

    fn poly(ctx: &RingCtx<StaticFpCtx<7>, Lex>, terms: Vec<T>) -> P {
        Polynomial::from_terms_in(ctx, terms).unwrap()
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
    fn respects_criterion_and_key_before_gm_pruning() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]);
        let g1 = poly(&ring, vec![term(1, &[1, 1, 0])]);
        let g2 = poly(&ring, vec![term(1, &[1, 0, 1])]);
        let g3 = poly(&ring, vec![term(1, &[0, 1, 1])]);

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1, g2, g3]);
        let mut queue = FifoPairs::new();

        let mut updater = GmPairUpdater::new(RejectIndexOneCriterion, MissingKeyForZero);
        updater.on_new_poly(&gb, &mut queue, 3).unwrap();

        assert_eq!(drain_pairs(&mut queue), vec![Pair::new(2, 2, 3)]);
    }

    #[test]
    fn deduplicates_equal_lcm_candidates() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]);
        let g1 = poly(&ring, vec![term(1, &[0, 2, 0])]);
        let g2 = poly(&ring, vec![term(1, &[2, 2, 0])]);

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1, g2]);
        let mut queue = FifoPairs::new();

        let mut updater = GmPairUpdater::new(KeepAllCriterion, IndexKey);
        updater.on_new_poly(&gb, &mut queue, 2).unwrap();

        assert_eq!(drain_pairs(&mut queue), vec![Pair::new(0, 0, 2)]);
    }

    #[test]
    fn prunes_dominated_lcm_candidates() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]);
        let g1 = poly(&ring, vec![term(1, &[2, 1, 0])]);
        let g2 = poly(&ring, vec![term(1, &[1, 0, 0])]);
        let g3 = poly(&ring, vec![term(1, &[0, 1, 0])]);
        let g4 = poly(&ring, vec![term(1, &[2, 2, 0])]);

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1, g2, g3, g4]);
        let mut queue = FifoPairs::new();

        let mut updater = GmPairUpdater::new(KeepAllCriterion, IndexKey);
        updater.on_new_poly(&gb, &mut queue, 4).unwrap();

        assert_eq!(drain_pairs(&mut queue), vec![Pair::new(0, 0, 4)]);
    }

    #[test]
    fn keeps_only_lcm_minimal_candidates_under_strict_divisibility() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]);
        let g1 = poly(&ring, vec![term(1, &[2, 2, 1])]);
        let g2 = poly(&ring, vec![term(1, &[2, 2, 0])]);

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1, g2]);
        let mut queue = FifoPairs::new();

        let mut updater = GmPairUpdater::new(KeepAllCriterion, IndexKey);
        updater.on_new_poly(&gb, &mut queue, 2).unwrap();

        assert_eq!(drain_pairs(&mut queue), vec![Pair::new(0, 0, 2)]);
    }

    #[test]
    fn rejects_out_of_range_new_index() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]);
        let g1 = poly(&ring, vec![term(1, &[0, 2, 0])]);
        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1]);
        let mut queue = FifoPairs::new();

        let mut updater = GmPairUpdater::new(KeepAllCriterion, IndexKey);
        let err = updater.on_new_poly(&gb, &mut queue, gb.len()).unwrap_err();

        assert!(matches!(err, PairUpdateError::InvariantViolation));
    }
}
