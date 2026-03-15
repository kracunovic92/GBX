use crate::pairing::{leading_mono_at, PairCriterion};
use crate::GrobnerBasis;
use gbx_poly::monomial::{gcd_is_one, Monomial, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Buchberger's product criterion.
///
/// This criterion eliminates a pair `(i, j)` when the leading monomials
/// `LM(g_i)` and `LM(g_j)` are relatively prime.
///
/// Equivalently, if
///
/// `gcd(LM(g_i), LM(g_j)) = 1`,
///
/// then the corresponding S-pair is known to admit a standard representation,
/// so the pair may be discarded.
///
/// # Behavior
///
/// - returns `true` if the pair should be kept
/// - returns `false` if the pair is eliminated
///
/// If either leading monomial is missing, this implementation conservatively
/// returns `false`.
///
/// # Notes
///
/// This criterion is purely local: it depends only on the two leading monomials.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProductCriterion;

impl<P> PairCriterion<P> for ProductCriterion
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: MonomialView<Word = u32> + Monomial,
{
    #[inline]
    fn keep_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> bool {
        let Some(lm_i) = leading_mono_at(gb, i) else {
            return false;
        };
        let Some(lm_j) = leading_mono_at(gb, j) else {
            return false;
        };

        !gcd_is_one(lm_i, lm_j)
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

    type F7 = Fp<7>;
    type Mono = DynamicMonomial;
    type T = Term<F7, Mono>;
    type P = Polynomial<T, VecTerms<T>>;

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
    fn eliminates_relatively_prime_leading_monomials() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 0, 0])]);
        let g1 = poly(&ring, vec![term(1, &[0, 3, 0])]);

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1]);

        let mut criterion = ProductCriterion;
        assert!(!criterion.keep_pair(&gb, 0, 1));
    }

    #[test]
    fn keeps_pairs_with_nontrivial_lm_gcd() {
        let ring = ring();

        let g0 = poly(&ring, vec![term(1, &[2, 1, 0])]);
        let g1 = poly(&ring, vec![term(1, &[1, 3, 0])]);

        let gb = GrobnerBasis::new(ring.id(), vec![g0, g1]);

        let mut criterion = ProductCriterion;
        assert!(criterion.keep_pair(&gb, 0, 1));
    }

    #[test]
    fn rejects_if_leading_monomial_missing() {
        let ring = ring();

        let zero = P::zero_in(&ring);
        let g1 = poly(&ring, vec![term(1, &[1, 0, 0])]);

        let gb = GrobnerBasis::new(ring.id(), vec![zero, g1]);

        let mut criterion = ProductCriterion;
        assert!(!criterion.keep_pair(&gb, 0, 1));
    }
}
