//! Ordered keys for symbolic preprocessing.

use core::cmp::Ordering;

use crate::algos::f4::symbolic::types::ProductSource;

use gbx_poly::monomial::MonomialView;
use gbx_poly::order::MonomialOrder;

/// Monomial key ordered by a borrowed monomial order.
///
/// This wrapper is useful when monomials need to live inside ordered standard
/// collections such as `BTreeSet`.
#[derive(Debug, Clone, Hash)]
pub struct OrderedMono<'a, M, O> {
    mono: M,
    order: &'a O,
}

impl<'a, M, O> OrderedMono<'a, M, O> {
    /// Creates an ordered monomial key.
    #[inline]
    #[must_use]
    pub fn new(mono: M, order: &'a O) -> Self {
        Self { mono, order }
    }

    /// Borrows the wrapped monomial.
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> &M {
        &self.mono
    }

    /// Consumes the key and returns the wrapped monomial.
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

/// Product key ordered by source identity and multiplier.
///
/// This is used to deduplicate unevaluated products during symbolic
/// preprocessing.
#[derive(Debug, Clone)]
pub struct OrderedProduct<'a, M, O> {
    source: ProductSource,
    multiplier: OrderedMono<'a, M, O>,
}

impl<'a, M, O> OrderedProduct<'a, M, O> {
    /// Creates an ordered product key.
    #[inline]
    #[must_use]
    pub fn new(source: ProductSource, multiplier: M, order: &'a O) -> Self {
        Self { source, multiplier: OrderedMono::new(multiplier, order) }
    }

    /// Returns the product source.
    #[inline]
    #[must_use]
    pub fn source(&self) -> ProductSource {
        self.source
    }

    /// Returns the ordered multiplier key.
    #[inline]
    #[must_use]
    pub fn multiplier(&self) -> &OrderedMono<'a, M, O> {
        &self.multiplier
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::algos::f4::symbolic::types::ProductSource;
    use crate::test_utils::test_ring;

    use gbx_poly::monomial::Monomial;
    use gbx_poly::order::{Grevlex, Lex};

    #[test]
    fn ordered_mono_uses_lex_order() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let x = OrderedMono::new(Monomial::from_slice(&[1, 0]), &ring.order);
        let y = OrderedMono::new(Monomial::from_slice(&[0, 1]), &ring.order);

        assert!(x > y);
    }

    #[test]
    fn ordered_mono_uses_grevlex_order() {
        let ring = test_ring(7, 2, Grevlex).expect("test ring construction should succeed");

        let x2 = OrderedMono::new(Monomial::from_slice(&[2, 0]), &ring.order);
        let y5 = OrderedMono::new(Monomial::from_slice(&[0, 5]), &ring.order);

        assert!(y5 > x2);
    }

    #[test]
    fn ordered_mono_into_inner_returns_monomial() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");
        let mono = Monomial::from_slice(&[2, 1]);

        let key = OrderedMono::new(mono.clone(), &ring.order);

        assert_eq!(key.as_ref(), &mono);
        assert_eq!(key.into_inner(), mono);
    }

    #[test]
    fn ordered_product_orders_by_source_before_multiplier() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let basis_product = OrderedProduct::new(
            ProductSource::Basis(0),
            Monomial::from_slice(&[0, 0]),
            &ring.order,
        );

        let history_product = OrderedProduct::new(
            ProductSource::HistoryReducedRow { batch_index: 0, row_index: 0 },
            Monomial::from_slice(&[9, 9]),
            &ring.order,
        );

        assert!(basis_product < history_product);
    }

    #[test]
    fn ordered_product_orders_same_source_by_multiplier() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let x = OrderedProduct::new(
            ProductSource::Basis(0),
            Monomial::from_slice(&[1, 0]),
            &ring.order,
        );

        let y = OrderedProduct::new(
            ProductSource::Basis(0),
            Monomial::from_slice(&[0, 1]),
            &ring.order,
        );

        assert!(x > y);
    }

    #[test]
    fn ordered_product_accessors_return_parts() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");
        let source = ProductSource::Basis(3);
        let multiplier = Monomial::from_slice(&[1, 2]);

        let product = OrderedProduct::new(source, multiplier.clone(), &ring.order);

        assert_eq!(product.source(), source);
        assert_eq!(product.multiplier().as_ref(), &multiplier);
    }
}
