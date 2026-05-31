//! Pending-pair updates after basis insertion.

use crate::algos::f4::error::Result;
use crate::algos::f4::pairs::criterion::PairCriterion;
use crate::algos::f4::pairs::pending::{PendingPairs, add_pairs_with_new_basis_element};

use gbx_poly::polynomial::PolynomialView;

/// Appends a polynomial to the basis and inserts its induced critical pairs.
///
/// The new polynomial is inserted at the next basis index. All admissible pairs
/// between the new element and older basis elements are added to `pending`.
///
/// # Errors
///
/// Returns a monomial error if critical-pair construction fails.
pub fn update_with_polynomial<P, C>(basis: &mut Vec<P>, pending: &mut PendingPairs, polynomial: P, criterion: &C) -> Result<()>
where
    P: PolynomialView,
    C: PairCriterion<P>,
{
    let new_index = basis.len();

    basis.push(polynomial);

    add_pairs_with_new_basis_element(basis, new_index, pending, criterion)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use crate::algos::f4::pairs::criterion::{NoCriterion, ProductCriterion};
    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::Polynomial;

    type P = Polynomial<FpElem>;

    #[test]
    fn update_with_polynomial_appends_to_basis() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();

        let mut basis = vec![f1];
        let mut pending = PendingPairs::new();

        update_with_polynomial(&mut basis, &mut pending, f2, &NoCriterion).expect("update should succeed");

        assert_eq!(basis.len(), 2);
    }

    #[test]
    fn update_with_polynomial_adds_pair_with_previous_basis_element() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();

        let mut basis = vec![f1];
        let mut pending = PendingPairs::new();

        update_with_polynomial(&mut basis, &mut pending, f2, &NoCriterion).expect("update should succeed");

        assert_eq!(pending.len(), 1);
        assert!(pending.contains(0, 1));
    }

    #[test]
    fn update_with_polynomial_adds_pairs_with_all_older_basis_elements() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();
        let f3: P = poly![&ring; (1, [0, 1])].unwrap();

        let mut basis = vec![f1, f2];
        let mut pending = PendingPairs::new();

        update_with_polynomial(&mut basis, &mut pending, f3, &NoCriterion).expect("update should succeed");

        assert_eq!(basis.len(), 3);
        assert_eq!(pending.len(), 2);
        assert!(pending.contains(0, 2));
        assert!(pending.contains(1, 2));
        assert!(!pending.contains(0, 1));
    }

    #[test]
    fn update_with_polynomial_respects_product_criterion() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [0, 1])].unwrap();

        let mut basis = vec![f1];
        let mut pending = PendingPairs::new();

        update_with_polynomial(&mut basis, &mut pending, f2, &ProductCriterion).expect("update should succeed");

        assert_eq!(basis.len(), 2);
        assert!(pending.is_empty());
    }

    #[test]
    fn update_with_zero_polynomial_appends_but_adds_no_pairs() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let zero = P::zero_in(&ring);

        let mut basis = vec![f1];
        let mut pending = PendingPairs::new();

        update_with_polynomial(&mut basis, &mut pending, zero, &NoCriterion).expect("update should succeed");

        assert_eq!(basis.len(), 2);
        assert!(pending.is_empty());
    }

    #[test]
    fn update_into_empty_basis_appends_without_pairs() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f: P = poly![&ring; (1, [2, 0])].unwrap();

        let mut basis = Vec::new();
        let mut pending = PendingPairs::new();

        update_with_polynomial(&mut basis, &mut pending, f, &NoCriterion).expect("update should succeed");

        assert_eq!(basis.len(), 1);
        assert!(pending.is_empty());
    }
}
