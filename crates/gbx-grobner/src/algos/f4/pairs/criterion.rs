//! Critical-pair filtering criteria.
//!
//! Criteria are pure filters: they decide whether a constructed critical pair
//! should remain pending, but they do not construct, store, or select pairs.

use crate::algos::f4::pairs::critical_pair::CriticalPair;

use gbx_poly::monomial::gcd_is_one;
use gbx_poly::polynomial::PolynomialView;

/// Decides whether a constructed critical pair should remain pending.
pub trait PairCriterion<P>
where
    P: PolynomialView,
{
    /// Returns `true` when `pair` should be kept.
    fn allows(&self, basis: &[P], pair: &CriticalPair) -> bool;
}

/// Criterion that keeps every critical pair.
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

/// Buchberger product criterion.
///
/// Pairs whose leading monomials are coprime are discarded.
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
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use crate::algos::f4::pairs::critical_pair::CriticalPair;
    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::order::{Grevlex, Lex};
    use gbx_poly::poly;
    use gbx_poly::polynomial::{Polynomial, PolynomialView};

    type P = Polynomial<FpElem>;

    fn pair_from_basis(basis: &[P], i: usize, j: usize) -> CriticalPair {
        CriticalPair::from_lms(
            i,
            j,
            basis[i]
                .leading_mono()
                .expect("basis element should be nonzero"),
            basis[j]
                .leading_mono()
                .expect("basis element should be nonzero"),
        )
        .expect("critical pair construction should succeed")
    }

    #[test]
    fn no_criterion_allows_any_valid_pair() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let basis = vec![f1, f2];
        let pair = pair_from_basis(&basis, 0, 1);

        assert!(NoCriterion.allows(&basis, &pair));
    }

    #[test]
    fn product_criterion_rejects_coprime_leading_monomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap();
        let f2: P = poly![&ring; (1, [0, 1]), (1, [0, 0])].unwrap();

        let basis = vec![f1, f2];
        let pair = pair_from_basis(&basis, 0, 1);

        assert!(!ProductCriterion.allows(&basis, &pair));
    }

    #[test]
    fn product_criterion_keeps_non_coprime_leading_monomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let basis = vec![f1, f2];
        let pair = pair_from_basis(&basis, 0, 1);

        assert!(ProductCriterion.allows(&basis, &pair));
    }

    #[test]
    fn product_criterion_uses_order_dependent_leading_monomials() {
        let lex_ring = test_ring(7, 2, Lex).expect("lex ring construction should succeed");
        let grevlex_ring = test_ring(7, 2, Grevlex).expect("grevlex ring construction should succeed");

        let f1_lex: P = poly![&lex_ring; (1, [2, 0]), (1, [0, 5])].unwrap();
        let f2_lex: P = poly![&lex_ring; (1, [1, 0])].unwrap();

        let lex_basis = vec![f1_lex, f2_lex];
        let lex_pair = pair_from_basis(&lex_basis, 0, 1);

        assert!(ProductCriterion.allows(&lex_basis, &lex_pair));

        let f1_grevlex: P = poly![&grevlex_ring; (1, [2, 0]), (1, [0, 5])].unwrap();
        let f2_grevlex: P = poly![&grevlex_ring; (1, [1, 0])].unwrap();

        let grevlex_basis = vec![f1_grevlex, f2_grevlex];
        let grevlex_pair = pair_from_basis(&grevlex_basis, 0, 1);

        assert!(!ProductCriterion.allows(&grevlex_basis, &grevlex_pair));
    }
}
