use core::cmp::Ordering;

use crate::algos::f4::symbolic::types::SymbolicSource;

use gbx_poly::monomial::MonomialView;
use gbx_poly::order::MonomialOrder;

/// A monomial wrapper whose ordering is defined by a borrowed monomial order.
#[derive(Debug, Clone, Hash)]
pub struct OrderedMono<'a, M, O> {
    mono: M,
    order: &'a O,
}

impl<'a, M, O> OrderedMono<'a, M, O> {
    #[inline]
    #[must_use]
    pub fn new(mono: M, order: &'a O) -> Self {
        Self { mono, order }
    }

    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> &M {
        &self.mono
    }

    #[inline]
    #[must_use]
    pub fn into_inner(self) -> M {
        self.mono
    }
}

impl<M, O> PartialEq for OrderedMono<'_, M, O>
where
    O: MonomialOrder,
    M: MonomialView,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.order.cmp(&self.mono, &other.mono) == Ordering::Equal
    }
}

impl<M, O> Eq for OrderedMono<'_, M, O>
where
    O: MonomialOrder,
    M: MonomialView,
{
}

impl<M, O> PartialOrd for OrderedMono<'_, M, O>
where
    O: MonomialOrder,
    M: MonomialView,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M, O> Ord for OrderedMono<'_, M, O>
where
    O: MonomialOrder,
    M: MonomialView,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&self.mono, &other.mono)
    }
}

/// Key for deduplicating symbolic products using:
/// 1. source identity
/// 2. multiplier under the active monomial order
#[derive(Debug, Clone)]
pub struct OrderedProduct<'a, M, O> {
    pub source: SymbolicSource,
    pub multiplier: OrderedMono<'a, M, O>,
}

impl<'a, M, O> OrderedProduct<'a, M, O> {
    #[inline]
    #[must_use]
    pub fn new(source: SymbolicSource, multiplier: M, order: &'a O) -> Self {
        Self { source, multiplier: OrderedMono::new(multiplier, order) }
    }
}

impl<M, O> PartialEq for OrderedProduct<'_, M, O>
where
    O: MonomialOrder,
    M: MonomialView,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<M, O> Eq for OrderedProduct<'_, M, O>
where
    O: MonomialOrder,
    M: MonomialView,
{
}

impl<M, O> PartialOrd for OrderedProduct<'_, M, O>
where
    O: MonomialOrder,
    M: MonomialView,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M, O> Ord for OrderedProduct<'_, M, O>
where
    O: MonomialOrder,
    M: MonomialView,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.source
            .cmp(&other.source)
            .then_with(|| self.multiplier.cmp(&other.multiplier))
    }
}
