use core::cmp::Ordering;

use crate::algos::f4::symbolic::types::SymbolicSource;

use gbx_poly::monomial::MonomialView;
use gbx_poly::order::MonomialOrder;

/// A monomial wrapper whose ordering is defined by a borrowed monomial order.
#[derive(Debug, Clone)]
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

impl<'a, M, O> PartialEq for OrderedMono<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.order.cmp(&self.mono, &other.mono) == Ordering::Equal
    }
}

impl<'a, M, O> Eq for OrderedMono<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
}

impl<'a, M, O> PartialOrd for OrderedMono<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, M, O> Ord for OrderedMono<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&self.mono, &other.mono)
    }
}

/// Key for deduplicating symbolic products using:
/// 1. source identity
/// 2. multiplier under the active monomial order
///
/// This is the generalized replacement for the old `(basis_index, multiplier)` key.
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

impl<'a, M, O> PartialEq for OrderedProduct<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a, M, O> Eq for OrderedProduct<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
}

impl<'a, M, O> PartialOrd for OrderedProduct<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, M, O> Ord for OrderedProduct<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.source
            .cmp(&other.source)
            .then_with(|| self.multiplier.cmp(&other.multiplier))
    }
}
