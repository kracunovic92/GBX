use std::collections::BTreeSet;

use crate::algos::f4::symbolic::ordered::OrderedMono;

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;

/// Incremental ordered worklist for monomials in `T(F_d)`.
///
/// `seen` contains monomials discovered from rows of `F_d`.
/// `pending` contains discovered monomials that have not yet been popped.
///
/// The worklist itself does not know about `Done`; the preprocessing state
/// decides whether a popped monomial has already been processed.
#[derive(Debug, Clone)]
pub struct MonomialWorklist<'a, O> {
    pending: BTreeSet<OrderedMono<'a, Monomial, O>>,
    seen: BTreeSet<OrderedMono<'a, Monomial, O>>,
}

impl<'a, O> MonomialWorklist<'a, O>
where
    O: MonomialOrder,
{
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { pending: BTreeSet::new(), seen: BTreeSet::new() }
    }

    /// Insert a monomial into `T(F_d)` if it has not been seen before.
    #[inline]
    pub fn push(&mut self, mono: Monomial, order: &'a O) {
        let seen_key = OrderedMono::new(mono.clone(), order);

        if self.seen.insert(seen_key) {
            let pending_key = OrderedMono::new(mono, order);
            self.pending.insert(pending_key);
        }
    }

    /// Insert all monomials appearing in one row.
    pub fn extend_from_row<P>(&mut self, row: &P, order: &'a O)
    where
        P: PolynomialView,
    {
        for term in row.terms().iter() {
            self.push(term.mono().clone(), order);
        }
    }

    /// Pop the next pending monomial according to the active monomial order.
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
