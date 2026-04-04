use crate::algos::f4::pairs::critical_pair::CriticalPair;
use gbx_poly::monomial::{gcd_is_one, Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Decide whether a newly formed critical pair should remain in the pending set.
///
/// A pair criterion is a pure filter:
/// - it does not construct pairs,
/// - it does not mutate the basis,
/// - it does not select batches,
/// - it only decides whether a pair is worth keeping for later processing.
pub trait PairCriterion<P>
where
    P: PolynomialView,
    P::Term: TermView,
{
    /// Return `true` if the pair should be kept.
    fn allows(&self, basis: &[P], pair: &CriticalPair<<<P as PolynomialView>::Term as TermView>::Mono>) -> bool;
}

/// Keep every critical pair.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoCriterion;

impl<P> PairCriterion<P> for NoCriterion
where
    P: PolynomialView,
    P::Term: TermView,
{
    fn allows(&self, _basis: &[P], _pair: &CriticalPair<<<P as PolynomialView>::Term as TermView>::Mono>) -> bool {
        true
    }
}

/// Buchberger's product criterion.
///
/// If `gcd(LM(f_i), LM(f_j)) = 1`, then the corresponding S-polynomial is known
/// to reduce to zero modulo `{f_i, f_j}`, so the pair can be discarded.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProductCriterion;

impl<P> PairCriterion<P> for ProductCriterion
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone,
{
    fn allows(&self, basis: &[P], pair: &CriticalPair<<<P as PolynomialView>::Term as TermView>::Mono>) -> bool {
        let Some(lm_i) = basis[pair.i()].leading_mono() else {
            return false;
        };
        let Some(lm_j) = basis[pair.j()].leading_mono() else {
            return false;
        };

        !gcd_is_one(lm_i, lm_j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::f4::pairs::critical_pair::CriticalPair;
    use crate::test_utils::test_ring;
    use gbx_field::fp::FpDynElem;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::{PolyDyn, PolynomialView};

    type P = PolyDyn<FpDynElem>;

    #[test]
    fn no_criterion_allows_any_valid_pair() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(); // x^2 + y
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap(); // x*y + 1
        let basis = vec![f1, f2];

        let lm1 = basis[0].leading_mono().expect("f1 should be nonzero");
        let lm2 = basis[1].leading_mono().expect("f2 should be nonzero");

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).expect("pair construction should succeed");

        assert!(NoCriterion.allows(&basis, &pair));
    }

    #[test]
    fn product_criterion_rejects_coprime_leading_monomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap(); // x^2 + 1
        let f2: P = poly![&ring; (1, [0, 1]), (1, [0, 0])].unwrap(); // y + 1
        let basis = vec![f1, f2];

        let lm1 = basis[0].leading_mono().expect("f1 should be nonzero");
        let lm2 = basis[1].leading_mono().expect("f2 should be nonzero");

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).expect("pair construction should succeed");

        assert!(!ProductCriterion.allows(&basis, &pair));
    }

    #[test]
    fn product_criterion_keeps_non_coprime_leading_monomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap(); // x^2 + 1
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap(); // x*y + 1
        let basis = vec![f1, f2];

        let lm1 = basis[0].leading_mono().expect("f1 should be nonzero");
        let lm2 = basis[1].leading_mono().expect("f2 should be nonzero");

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).expect("pair construction should succeed");

        assert!(ProductCriterion.allows(&basis, &pair));
    }
}
