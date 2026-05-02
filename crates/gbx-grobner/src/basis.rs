//! Gröbner basis container.
//!
//! A Gröbner basis is mathematically a set, but we store an ordered list for
//! deterministic iteration and stable indexing.
//!
//! The container stores a [`RingId`](crate::ring::RingId) so it can be checked
//! against a [`RingCtx`](crate::ring::RingCtx) to prevent accidental mixing of rings.

extern crate alloc;

use alloc::vec::Vec;
use gbx_poly::polynomial::PolynomialError;
use gbx_poly::ring::{FieldCtx, RingCtx, RingId};

/// Ordered Gröbner basis container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrobnerBasis<P> {
    ring_id: RingId,
    polys: Vec<P>,
}

impl<P> GrobnerBasis<P> {
    /// Construct from ring id + polynomials (low-level).
    ///
    /// Prefer [`GrobnerBasis::empty_in`] + `push` in algorithms.
    #[inline]
    pub fn new(ring_id: RingId, polys: Vec<P>) -> Self {
        Self { ring_id, polys }
    }

    /// Construct an empty basis tagged with `ctx.id()`.
    #[inline]
    pub fn empty_in<F, O>(ctx: &RingCtx<F, O>) -> Self
    where
        F: FieldCtx,
    {
        Self { ring_id: ctx.id(), polys: Vec::new() }
    }

    /// Ring id tag.
    #[inline]
    pub fn ring_id(&self) -> RingId {
        self.ring_id
    }

    /// Ensure the basis matches the ring context.
    #[inline]
    pub fn assert_same_ring<F, O>(&self, ctx: &RingCtx<F, O>) -> Result<(), PolynomialError>
    where
        F: FieldCtx,
    {
        ctx.assert_same_ring_id(self.ring_id)
            .map_err(PolynomialError::from)
    }

    /// Borrow as slice.
    #[inline]
    pub fn as_slice(&self) -> &[P] {
        &self.polys
    }

    /// Mutable access (algorithm-internal).
    #[inline]
    pub fn as_mut_vec(&mut self) -> &mut Vec<P> {
        &mut self.polys
    }

    /// Number of polynomials.
    #[inline]
    pub fn len(&self) -> usize {
        self.polys.len()
    }

    /// Empty?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.polys.is_empty()
    }

    /// Get polynomial by index.
    #[inline]
    pub fn get(&self, i: usize) -> Option<&P> {
        self.polys.get(i)
    }

    /// Iterator.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, P> {
        self.polys.iter()
    }

    /// Push polynomial (no normalization / ring checks here).
    #[inline]
    pub fn push(&mut self, p: P) {
        self.polys.push(p);
    }

    /// Consume into vec.
    #[inline]
    pub fn into_vec(self) -> Vec<P> {
        self.polys
    }

    /// Moving out vectors
    pub fn take_polys(&mut self) -> Vec<P> {
        core::mem::take(&mut self.polys)
    }

    /// Mutable borrow as slice (algorithm-internal).
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [P] {
        &mut self.polys
    }

    /// Retain only the elements specified by the predicate.
    ///
    /// This is a thin wrapper over `Vec::retain`, useful for algorithm code.
    #[inline]
    pub fn retain<FN>(&mut self, mut f: FN)
    where
        FN: FnMut(&P) -> bool,
    {
        self.polys.retain(|p| f(p));
    }
}

impl<'a, P> IntoIterator for &'a GrobnerBasis<P> {
    type Item = &'a P;
    type IntoIter = core::slice::Iter<'a, P>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.polys.iter()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use gbx_field::prelude::Fp;
    use gbx_poly::order::Lex;
    use gbx_poly::ring::Ring;

    #[test]
    fn empty_in_tags_ring() {
        let ring = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let gb: GrobnerBasis<u32> = GrobnerBasis::empty_in(&ring);
        assert_eq!(gb.ring_id(), ring.id());
        assert!(gb.is_empty());
    }

    #[test]
    fn assert_same_ring_works() {
        let a = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();
        let b = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();
        let gb: GrobnerBasis<u32> = GrobnerBasis::new(a.id(), alloc::vec![1, 2, 3]);
        assert!(gb.assert_same_ring(&a).is_ok());
        assert!(gb.assert_same_ring(&b).is_err());
    }

    #[test]
    fn retain_filters_elements() {
        let ring = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let mut gb: GrobnerBasis<u32> = GrobnerBasis::new(ring.id(), alloc::vec![0, 1, 0, 2, 3, 0]);

        gb.retain(|p| *p != 0);

        assert_eq!(gb.as_slice(), &[1, 2, 3]);
        assert_eq!(gb.len(), 3);
        assert_eq!(gb.ring_id(), ring.id()); // tag unchanged
    }

    #[test]
    fn retain_can_clear_all() {
        let ring = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let mut gb: GrobnerBasis<u32> = GrobnerBasis::new(ring.id(), alloc::vec![1, 2, 3]);

        gb.retain(|_| false);

        assert!(gb.is_empty());
        assert_eq!(gb.as_slice(), &[] as &[u32]);
        assert_eq!(gb.ring_id(), ring.id());
    }

    #[test]
    fn retain_keeps_all() {
        let ring = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let mut gb: GrobnerBasis<u32> = GrobnerBasis::new(ring.id(), alloc::vec![1, 2, 3]);

        gb.retain(|_| true);

        assert_eq!(gb.as_slice(), &[1, 2, 3]);
        assert_eq!(gb.len(), 3);
    }
}
