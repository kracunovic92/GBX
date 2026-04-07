use std::collections::BTreeSet;

use crate::algos::f4::symbolic::ordered::OrderedMono;

use gbx_poly::monomial::MonomialView;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Incremental worklist for monomials in `T(F_d)`.
///
/// `seen` tracks monomials discovered at least once.
/// `pending` tracks discovered monomials not yet processed.
#[derive(Debug, Clone)]
pub struct MonomialWorklist<'a, M, O> {
    pending: BTreeSet<OrderedMono<'a, M, O>>,
    seen: BTreeSet<OrderedMono<'a, M, O>>,
}

impl<'a, M, O> MonomialWorklist<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32> + Clone,
{
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { pending: BTreeSet::new(), seen: BTreeSet::new() }
    }

    /// Insert a monomial into `T(F_d)` if it has not been seen before.
    #[inline]
    pub fn push(&mut self, mono: M, order: &'a O) {
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
        P::Term: TermView<Mono = M>,
    {
        for term in row.terms().iter() {
            self.push(term.mono().clone(), order);
        }
    }

    /// Pop the next pending monomial according to the active monomial order.
    pub fn pop_next(&mut self) -> Option<OrderedMono<'a, M, O>> {
        self.pending.pop_first()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    #[inline]
    #[must_use]
    pub fn has_seen(&self, mono: &OrderedMono<'a, M, O>) -> bool {
        self.seen.contains(mono)
    }
}

impl<'a, M, O> Default for MonomialWorklist<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32> + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
