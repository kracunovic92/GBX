//! Ordered monomial worklist used by symbolic preprocessing.

use std::collections::BTreeSet;

use crate::algos::f4::symbolic::ordered::OrderedMono;

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;

/// Ordered worklist of discovered monomials.
///
/// A monomial can be pushed many times but is only returned once. The next
/// monomial is chosen according to the active monomial order.
#[derive(Debug, Clone)]
pub struct MonomialWorklist<'a, O> {
    pending: BTreeSet<OrderedMono<'a, Monomial, O>>,
    seen: BTreeSet<OrderedMono<'a, Monomial, O>>,
}

impl<'a, O> MonomialWorklist<'a, O>
where
    O: MonomialOrder,
{
    /// Creates an empty worklist.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { pending: BTreeSet::new(), seen: BTreeSet::new() }
    }

    /// Returns `true` if no discovered monomials are pending.
    #[inline]
    #[must_use]
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Returns the number of pending monomials.
    #[inline]
    #[must_use]
    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    /// Inserts a monomial if it has not been seen before.
    #[inline]
    pub fn push(&mut self, mono: Monomial, order: &'a O) {
        let seen_key = OrderedMono::new(mono.clone(), order);

        if self.seen.insert(seen_key) {
            self.pending.insert(OrderedMono::new(mono, order));
        }
    }

    /// Inserts all monomials appearing in `row`.
    pub fn extend_from_row<P>(&mut self, row: &P, order: &'a O)
    where
        P: PolynomialView,
    {
        for term in row.terms() {
            self.push(term.mono().clone(), order);
        }
    }

    /// Removes and returns the next pending monomial.
    pub fn pop_next(&mut self) -> Option<OrderedMono<'a, Monomial, O>> {
        self.pending.pop_first()
    }
}

impl<O> Default for MonomialWorklist<'_, O>
where
    O: MonomialOrder,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::order::{Grevlex, Lex};
    use gbx_poly::poly;
    use gbx_poly::polynomial::Polynomial;

    type P = Polynomial<FpElem>;

    #[test]
    fn push_deduplicates_monomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let mut worklist = MonomialWorklist::new();

        worklist.push(Monomial::from_slice(&[1, 0]), &ring.order);
        worklist.push(Monomial::from_slice(&[1, 0]), &ring.order);

        assert_eq!(worklist.len(), 1);

        let popped = worklist.pop_next().unwrap().into_inner();

        assert_eq!(popped, Monomial::from_slice(&[1, 0]));
        assert!(worklist.is_empty());
    }

    #[test]
    fn pop_next_uses_lex_order() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let mut worklist = MonomialWorklist::new();

        worklist.push(Monomial::from_slice(&[0, 1]), &ring.order);
        worklist.push(Monomial::from_slice(&[1, 0]), &ring.order);

        assert_eq!(
            worklist.pop_next().unwrap().into_inner(),
            Monomial::from_slice(&[0, 1])
        );

        assert_eq!(
            worklist.pop_next().unwrap().into_inner(),
            Monomial::from_slice(&[1, 0])
        );
    }

    #[test]
    fn pop_next_uses_grevlex_order() {
        let ring = test_ring(7, 2, Grevlex).expect("test ring construction should succeed");

        let mut worklist = MonomialWorklist::new();

        worklist.push(Monomial::from_slice(&[2, 0]), &ring.order);
        worklist.push(Monomial::from_slice(&[0, 5]), &ring.order);

        assert_eq!(
            worklist.pop_next().unwrap().into_inner(),
            Monomial::from_slice(&[2, 0])
        );

        assert_eq!(
            worklist.pop_next().unwrap().into_inner(),
            Monomial::from_slice(&[0, 5])
        );
    }

    #[test]
    fn extend_from_row_inserts_all_row_monomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let row: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();

        let mut worklist = MonomialWorklist::new();

        worklist.extend_from_row(&row, &ring.order);

        assert_eq!(worklist.len(), 2);

        assert_eq!(
            worklist.pop_next().unwrap().into_inner(),
            Monomial::from_slice(&[0, 1])
        );

        assert_eq!(
            worklist.pop_next().unwrap().into_inner(),
            Monomial::from_slice(&[2, 0])
        );

        assert!(worklist.pop_next().is_none());
    }

    #[test]
    fn popped_monomials_are_not_reinserted() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let x = Monomial::from_slice(&[1, 0]);

        let mut worklist = MonomialWorklist::new();

        worklist.push(x.clone(), &ring.order);

        assert_eq!(worklist.pop_next().unwrap().into_inner(), x);

        worklist.push(Monomial::from_slice(&[1, 0]), &ring.order);

        assert!(worklist.is_empty());
    }
}
