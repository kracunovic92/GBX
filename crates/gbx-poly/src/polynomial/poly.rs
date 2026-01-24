//! Generic sparse polynomial engine.
//!
//! This module provides [`Polynomial`], an internal generic engine type that powers
//! the public concrete types (typically type aliases):
//!
//! - `FixedPolynomial<F, O, N>`   (compile-time arity via `FixedTerm<F, N>`)
//! - `DynamicPolynomial<F, O>`    (runtime arity via `DynamicTerm<F>`)
//!
//! This engine enforces those invariants when you call:
//! - [`PolynomialMut::from_terms`]
//! - [`PolynomialMut::normalize_in_place`]
//!
//! Note that [`PolynomialMut::push_term`] may temporarily violate invariants.

extern crate alloc;

use alloc::vec::Vec;
use core::marker::PhantomData;

use gbx_alg::Zero;
use gbx_storage::polynomial::TermStorage;

use crate::monomial::Monomial;
use crate::monomial::MonomialOrder;
use crate::polynomial::normalize::normalize_terms;
use crate::polynomial::traits::{PolynomialMut, PolynomialView};
use crate::term::traits::{Term, TermView};

/// Generic sparse polynomial.
///
/// This is the single “engine” type that implements [`PolynomialView`] and
/// [`PolynomialMut`] once, and can then be re-used for:
/// - fixed-arity polynomials (with `FixedTerm`),
/// - dynamic-arity polynomials (with `DynamicTerm`),
/// - alternative storage backends (packed/arena/etc.).
///
/// You typically do **not** expose this as the main public type; instead export
/// type aliases like `FixedPolynomial` and `DynamicPolynomial`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Polynomial<T, O, S> {
    storage: S,
    _order: PhantomData<O>,
    _term: PhantomData<T>,
}

impl<T, O, S> Polynomial<T, O, S>
where
    T: Term,
    T::Field: gbx_alg::Field,
    T::Mono: Monomial,
    O: MonomialOrder,
    S: TermStorage<T> + Default,
{
    /// Create a polynomial from an explicit storage backend.
    ///
    /// This does **not** normalize. Use this for advanced cases only.
    #[inline]
    pub fn from_storage(storage: S) -> Self {
        Self { storage, _order: PhantomData, _term: PhantomData }
    }

    /// Borrow the underlying storage.
    #[inline]
    pub fn storage(&self) -> &S {
        &self.storage
    }

    /// Mutably borrow the underlying storage.
    ///
    /// Note: direct mutation may violate invariants; call `normalize_in_place()`
    /// afterward if needed.
    #[inline]
    pub fn storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }

    /// The zero polynomial (no terms).
    #[inline]
    pub fn zero() -> Self {
        <Self as PolynomialMut>::zero()
    }

    /// Build from raw terms and normalize (drops zeros, merges like monomials, sorts).
    #[inline]
    pub fn from_terms(terms: Vec<T>) -> Self
    where
        T::Field: Zero + Clone,
    {
        <Self as PolynomialMut>::from_terms(terms)
    }

    /// Normalize the polynomial in-place.
    #[inline]
    pub fn normalize(&mut self)
    where
        T::Field: Zero + Clone,
    {
        <Self as PolynomialMut>::normalize_in_place(self)
    }
}

impl<T, O, S> PolynomialView for Polynomial<T, O, S>
where
    T: TermView,
    T::Field: gbx_alg::Field,
    O: MonomialOrder,
    S: TermStorage<T>,
{
    type Field = <T as TermView>::Field;
    type Term = T;
    type Order = O;

    #[inline]
    fn is_zero(&self) -> bool {
        self.storage.as_slice().is_empty()
    }

    #[inline]
    fn terms(&self) -> &[Self::Term] {
        self.storage.as_slice()
    }
}

impl<T, O, S> PolynomialMut for Polynomial<T, O, S>
where
    T: Term,
    <T as TermView>::Field: gbx_alg::Field,
    <T as TermView>::Mono: Monomial,
    O: MonomialOrder,
    S: TermStorage<T> + Default,
{
    #[inline]
    fn zero() -> Self {
        Self::from_storage(S::default())
    }

    #[inline]
    fn from_terms(mut terms: Vec<Self::Term>) -> Self
    where
        Self::Field: Zero + Clone,
    {
        normalize_terms::<T, O>(&mut terms);
        let mut storage = S::default();
        storage.set_from_vec(terms);
        Self::from_storage(storage)
    }

    #[inline]
    fn push_term(&mut self, term: Self::Term) {
        self.storage.push(term);
    }

    #[inline]
    fn normalize_in_place(&mut self)
    where
        Self::Field: Zero + Clone,
    {
        self.storage.with_vec(|v| normalize_terms::<T, O>(v));
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;
    use crate::monomial::{Lex, MonomialView};
    use crate::polynomial::traits::{PolynomialMut, PolynomialView};
    use crate::term::{FixedTerm, TermView};
    use gbx_field::fp::Fp;
    use gbx_storage::polynomial::VecTerms;

    type F7 = Fp<7>;
    type T2 = FixedTerm<F7, 2>;

    type P2Lex = Polynomial<T2, Lex, VecTerms<T2>>;

    fn t(coeff: u32, e0: u32, e1: u32) -> T2 {
        T2::from_coeff_and_exponents(F7::new(coeff), [e0, e1])
    }

    #[test]
    fn zero_has_no_terms_and_no_leading_term() {
        let p = P2Lex::zero();
        assert!(p.is_zero());
        assert_eq!(p.terms().len(), 0);
        assert!(p.leading_term().is_none());
        assert!(p.leading_monomial().is_none());
        assert!(p.leading_coefficient().is_none());
    }

    #[test]
    fn from_terms_normalizes_drop_zero_merge_and_sort_desc() {
        let terms = vec![t(0, 5, 0), t(3, 1, 0), t(4, 0, 7), t(5, 1, 0)];

        let p = P2Lex::from_terms(terms);

        assert_eq!(p.terms().len(), 2);

        let lt = p.leading_term().unwrap();
        assert_eq!(lt.mono().exponents(), &[1, 0]);
        assert_eq!(*lt.coeff(), F7::new(1));

        let second = &p.terms()[1];
        assert_eq!(second.mono().exponents(), &[0, 7]);
        assert_eq!(*second.coeff(), F7::new(4));
    }

    #[test]
    fn push_then_normalize_in_place() {
        let mut p = P2Lex::zero();

        p.push_term(t(2, 0, 3));
        p.push_term(t(6, 2, 0));
        p.push_term(t(5, 0, 3));

        assert_eq!(p.terms().len(), 3);

        p.normalize_in_place();

        assert_eq!(p.terms().len(), 1);
        let lt = p.leading_term().unwrap();
        assert_eq!(lt.mono().exponents(), &[2, 0]);
        assert_eq!(*lt.coeff(), F7::new(6));
    }
}
