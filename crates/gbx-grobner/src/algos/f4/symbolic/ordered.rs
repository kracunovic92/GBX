use core::cmp::Ordering;

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
    pub fn new(mono: M, order: &'a O) -> Self {
        Self { mono, order }
    }

    #[inline]
    pub fn as_ref(&self) -> &M {
        &self.mono
    }

    #[inline]
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

/// Key for deduplicating `(basis_index, multiplier)` using the active monomial order.
#[derive(Debug, Clone)]
pub struct OrderedSeed<'a, M, O> {
    pub basis_index: usize,
    pub multiplier: OrderedMono<'a, M, O>,
}

impl<'a, M, O> OrderedSeed<'a, M, O> {
    #[inline]
    pub fn new(basis_index: usize, multiplier: M, order: &'a O) -> Self {
        Self { basis_index, multiplier: OrderedMono::new(multiplier, order) }
    }
}

impl<'a, M, O> PartialEq for OrderedSeed<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a, M, O> Eq for OrderedSeed<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
}

impl<'a, M, O> PartialOrd for OrderedSeed<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, M, O> Ord for OrderedSeed<'a, M, O>
where
    O: MonomialOrder,
    M: MonomialView<Word = u32>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.basis_index
            .cmp(&other.basis_index)
            .then_with(|| self.multiplier.cmp(&other.multiplier))
    }
}
