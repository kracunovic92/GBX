use crate::pairing::filters::{PairFilter, PairSetView};
use crate::pairing::leading_mono_at;
use crate::GrobnerBasis;
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Buchberger chain-style pair filter.
///
/// This filter removes a pair `(i, j)` if there exists some `l != i, j` such that:
///
/// - `LT(g_l)` divides `lcm(LT(g_i), LT(g_j))`, and
/// - the supporting pairs `[i, l]` and `[j, l]` are no longer active/pending
///
/// Here, "active/pending" is determined by [`PairSetView`].
///
/// # Notes
///
/// This filter is state-aware and therefore does not belong in the local
/// `PairCriterion` layer.
///
/// Its exact behavior depends on the meaning of the pair-state view supplied
/// by the caller.
#[derive(Debug, Default, Clone, Copy)]
pub struct ChainPairFilter;

impl<P> PairFilter<P> for ChainPairFilter
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    fn keep_pair<S: PairSetView>(&mut self, gb: &GrobnerBasis<P>, state: &S, i: usize, j: usize) -> bool {
        let Some(lm_i) = leading_mono_at(gb, i) else {
            return false;
        };
        let Some(lm_j) = leading_mono_at(gb, j) else {
            return false;
        };
        let Ok(lcm_ij) = lm_i.checked_lcm(lm_j) else {
            return false;
        };

        for l in 0..gb.len() {
            if l == i || l == j {
                continue;
            }

            let Some(lm_l) = leading_mono_at(gb, l) else {
                continue;
            };

            let l_divides_lcm = matches!(lcm_ij.checked_div_by(lm_l), Ok(Some(_)));
            if !l_divides_lcm {
                continue;
            }

            let il_pending = state.contains_pair(i, l);
            let jl_pending = state.contains_pair(j, l);

            if !il_pending && !jl_pending {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GrobnerBasis;
    use gbx_field::fp::Fp;
    use gbx_poly::monomial::DynamicMonomial;
    use gbx_poly::order::Lex;
    use gbx_poly::polynomial::Polynomial;
    use gbx_poly::ring::{Ring, RingCtx, StaticFpCtx};
    use gbx_poly::term::Term;
    use gbx_storage::polynomial::VecTerms;
    use std::collections::BTreeSet;

    type F7 = Fp<7>;
    type Mono = DynamicMonomial;
    type T = Term<F7, Mono>;
    type P = Polynomial<T, VecTerms<T>>;

    #[derive(Debug, Default)]
    struct TestPairState {
        pairs: BTreeSet<(usize, usize)>,
    }

    impl TestPairState {
        fn with_pairs(pairs: &[(usize, usize)]) -> Self {
            let mut out = Self::default();
            for &(i, j) in pairs {
                let key = if i < j { (i, j) } else { (j, i) };
                out.pairs.insert(key);
            }
            out
        }
    }

    impl PairSetView for TestPairState {
        fn contains_pair(&self, i: usize, j: usize) -> bool {
            let key = if i < j { (i, j) } else { (j, i) };
            self.pairs.contains(&key)
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

    #[test]
    fn keeps_pair_when_no_chain_witness_exists() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]); // x^2
        let g1 = poly(&ring, vec![term(1, &[0, 2, 0])]); // y^2
        let g2 = poly(&ring, vec![term(1, &[0, 0, 1])]); // z

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1, g2]);
        let state = TestPairState::default();

        let mut filter = ChainPairFilter;
        assert!(filter.keep_pair(&gb, &state, 0, 1));
    }

    #[test]
    fn keeps_pair_when_supporting_pair_is_still_pending() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]); // x^2
        let g1 = poly(&ring, vec![term(1, &[1, 1, 0])]); // x y
        let g2 = poly(&ring, vec![term(1, &[1, 0, 0])]); // x

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1, g2]);

        // Pair (0,2) still pending, so chain should not eliminate (0,1).
        let state = TestPairState::with_pairs(&[(0, 2)]);

        let mut filter = ChainPairFilter;
        assert!(filter.keep_pair(&gb, &state, 0, 1));
    }

    #[test]
    fn rejects_pair_when_chain_witness_exists_and_supporting_pairs_are_gone() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]); // x^2
        let g1 = poly(&ring, vec![term(1, &[1, 1, 0])]); // x y
        let g2 = poly(&ring, vec![term(1, &[1, 0, 0])]); // x

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1, g2]);

        // Neither (0,2) nor (1,2) pending.
        let state = TestPairState::default();

        let mut filter = ChainPairFilter;
        assert!(!filter.keep_pair(&gb, &state, 0, 1));
    }
}
