use crate::algos::f4::pairs::critical_pair::CriticalPair;

use gbx_poly::monomial::gcd_is_one;
use gbx_poly::polynomial::PolynomialView;

/// Decides whether a newly formed critical pair should remain pending.
///
/// A pair criterion is only a filter:
/// - it does not construct pairs,
/// - it does not mutate the basis,
/// - it does not select batches,
/// - it only decides whether a pair should be kept.
pub trait PairCriterion<P>
where
    P: PolynomialView,
{
    /// Returns `true` if the pair should be kept.
    fn allows(&self, basis: &[P], pair: &CriticalPair) -> bool;
}

/// Keep every critical pair.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoCriterion;

impl<P> PairCriterion<P> for NoCriterion
where
    P: PolynomialView,
{
    #[inline]
    fn allows(&self, _basis: &[P], _pair: &CriticalPair) -> bool {
        true
    }
}

/// Buchberger's product criterion.
///
/// If `gcd(LM(f_i), LM(f_j)) = 1`, then the corresponding S-polynomial
/// can be discarded.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProductCriterion;

impl<P> PairCriterion<P> for ProductCriterion
where
    P: PolynomialView,
{
    #[inline]
    fn allows(&self, basis: &[P], pair: &CriticalPair) -> bool {
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

    use gbx_field::fp::FpElem;
    use gbx_poly::order::{Grevlex, Lex};
    use gbx_poly::poly;
    use gbx_poly::polynomial::{Polynomial, PolynomialView};

    type P = Polynomial<FpElem>;

    #[test]
    fn no_criterion_allows_any_valid_pair() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let basis = vec![f1, f2];

        let lm1 = basis[0].leading_mono().unwrap();
        let lm2 = basis[1].leading_mono().unwrap();

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).unwrap();

        assert!(NoCriterion.allows(&basis, &pair));
    }

    #[test]
    fn product_criterion_rejects_coprime_leading_monomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap();
        let f2: P = poly![&ring; (1, [0, 1]), (1, [0, 0])].unwrap();

        let basis = vec![f1, f2];

        let lm1 = basis[0].leading_mono().unwrap();
        let lm2 = basis[1].leading_mono().unwrap();

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).unwrap();

        assert!(!ProductCriterion.allows(&basis, &pair));
    }

    #[test]
    fn product_criterion_keeps_non_coprime_leading_monomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let basis = vec![f1, f2];

        let lm1 = basis[0].leading_mono().unwrap();
        let lm2 = basis[1].leading_mono().unwrap();

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).unwrap();

        assert!(ProductCriterion.allows(&basis, &pair));
    }

    #[test]
    fn product_criterion_can_depend_on_order_via_leading_monomials() {
        let lex_ring = test_ring(7, 2, Lex).expect("lex ring construction should succeed");
        let grevlex_ring = test_ring(7, 2, Grevlex).expect("grevlex ring construction should succeed");

        // f1 = x^2 + y^5
        // f2 = x
        //
        // Lex:
        //   LM(f1) = x^2, LM(f2) = x, gcd != 1 => keep
        //
        // Grevlex:
        //   LM(f1) = y^5, LM(f2) = x, gcd = 1 => reject
        let f1_lex: P = poly![&lex_ring; (1, [2, 0]), (1, [0, 5])].unwrap();
        let f2_lex: P = poly![&lex_ring; (1, [1, 0])].unwrap();

        let lex_basis = vec![f1_lex, f2_lex];

        let lex_pair = CriticalPair::from_lms(
            0,
            1,
            lex_basis[0].leading_mono().unwrap(),
            lex_basis[1].leading_mono().unwrap(),
        )
        .unwrap();

        assert!(ProductCriterion.allows(&lex_basis, &lex_pair));

        let f1_grevlex: P = poly![&grevlex_ring; (1, [2, 0]), (1, [0, 5])].unwrap();
        let f2_grevlex: P = poly![&grevlex_ring; (1, [1, 0])].unwrap();

        let grevlex_basis = vec![f1_grevlex, f2_grevlex];

        let grevlex_pair = CriticalPair::from_lms(
            0,
            1,
            grevlex_basis[0].leading_mono().unwrap(),
            grevlex_basis[1].leading_mono().unwrap(),
        )
        .unwrap();

        assert!(!ProductCriterion.allows(&grevlex_basis, &grevlex_pair));
    }
}
