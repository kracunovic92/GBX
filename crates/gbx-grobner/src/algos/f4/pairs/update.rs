use crate::algos::f4::error::Result;
use crate::algos::f4::pairs::criterion::PairCriterion;
use crate::algos::f4::pairs::pending::{add_pairs_with_new_basis_element, PendingPairs};

use gbx_poly::polynomial::PolynomialView;

/// Insert one new polynomial into the basis and generate all newly induced critical pairs.
///
/// If the new polynomial is inserted at index `k`, this creates all admissible pairs
/// `(i, k)` with `0 <= i < k` and inserts them into the pending set.
pub fn update_with_polynomial<P, C>(basis: &mut Vec<P>, pending: &mut PendingPairs, polynomial: P, criterion: &C) -> Result<()>
where
    P: PolynomialView,
    C: PairCriterion<P>,
{
    let new_index = basis.len();

    basis.push(polynomial);

    add_pairs_with_new_basis_element(basis, new_index, pending, criterion)?;

    Ok(())
}
#[cfg(test)]
mod tests {
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
}
