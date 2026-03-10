//! Generic sparse polynomial container (context-driven).
//!
//! `Polynomial<T, S>` stores:
//! - a [`RingId`](crate::ring::RingId) tag to detect mixing rings
//! - a storage backend `S` holding terms `T`
//!
//! All arithmetic and ordering is supplied by a [`RingCtx`](crate::ring::RingCtx).
//!
//! # Invariants (when normalized)
//! - no zero coefficients
//! - no duplicate monomials
//! - terms sorted descending by the ring order

use core::marker::PhantomData;
use std::vec::Vec;

use crate::monomial::{Monomial, MonomialError, MonomialView};
use crate::order::MonomialOrder;
use crate::polynomial::error::{PolynomialError, Result};
use crate::polynomial::normalize::normalize_terms_in;
use crate::polynomial::traits::{PolynomialMut, PolynomialView};
use crate::ring::{FieldCtx, RingCtx, RingId};
use crate::term::{TermOwned, TermView};
use gbx_storage::polynomial::TermStorage;

/// Generic sparse polynomial.
#[derive(Clone, Debug)]
pub struct Polynomial<T, S> {
    ring_id: RingId,
    storage: S,
    _term: PhantomData<T>,
}

impl<T, S> Polynomial<T, S>
where
    T: TermOwned + TermView,
    S: TermStorage<T> + Default,
{
    /// Create from a storage backend without normalization.
    ///
    /// # Safety
    /// Caller must ensure `ring_id` matches the `RingCtx` used later.
    #[inline]
    pub fn from_storage(ring_id: RingId, storage: S) -> Self {
        Self { ring_id, storage, _term: PhantomData }
    }

    /// Borrow the storage backend.
    #[inline]
    pub fn storage(&self) -> &S {
        &self.storage
    }

    /// Mutably borrow the storage backend.
    ///
    /// Direct modification may break invariants.
    #[inline]
    pub fn storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }

    /// Ring id tag stored in this polynomial.
    #[inline]
    pub fn ring_id_tag(&self) -> RingId {
        self.ring_id
    }

    /// Assert `ctx.id()` equals this polynomial's ring id.
    #[inline]
    pub fn assert_same_ring<F, O>(&self, ctx: &RingCtx<F, O>) -> Result<()>
    where
        F: FieldCtx,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id)
            .map_err(PolynomialError::from)
    }

    /// Provide raw mutable access to the underlying `Vec<T>` via the storage backend.
    #[inline]
    pub fn with_terms_vec_mut<R>(&mut self, f: impl FnOnce(&mut Vec<T>) -> R) -> R {
        self.storage.with_vec(f)
    }

    /// Convenience: construct the zero polynomial inside `ctx`.
    #[inline]
    pub fn zero_in<F, O>(ctx: &RingCtx<F, O>) -> Self
    where
        F: FieldCtx,
        O: MonomialOrder,
        Self: PolynomialMut<Term = T>,
    {
        <Self as PolynomialMut>::zero_in(ctx)
    }

    /// Convenience: construct from terms and normalize inside `ctx`.
    #[inline]
    pub fn from_terms_in<F, O>(ctx: &RingCtx<F, O>, terms: Vec<T>) -> Result<Self>
    where
        F: FieldCtx<Elem = T::Coeff>,
        O: MonomialOrder,
        Self: PolynomialMut<Term = T>,
    {
        <Self as PolynomialMut>::from_terms_in(ctx, terms)
    }
}

/* -------------------------------------------------------------------------- */
/* Traits                                                                      */
/* -------------------------------------------------------------------------- */

impl<T, S> PolynomialView for Polynomial<T, S>
where
    T: TermView,
    S: TermStorage<T>,
{
    type Term = T;

    #[inline]
    fn ring_id(&self) -> RingId {
        self.ring_id
    }

    #[inline]
    fn terms(&self) -> &[Self::Term] {
        self.storage.as_slice()
    }
}

impl<T, S> PolynomialMut for Polynomial<T, S>
where
    T: TermOwned + TermView,
    T::Coeff: Copy + Eq,
    T::Mono: Monomial + MonomialView<Word = u32> + Clone + Eq,
    S: TermStorage<T> + Default,
{
    #[inline]
    fn zero_in<F, O>(ctx: &RingCtx<F, O>) -> Self
    where
        F: FieldCtx,
        O: MonomialOrder,
    {
        Self::from_storage(ctx.id(), S::default())
    }

    #[inline]
    fn from_terms_in<F, O>(ctx: &RingCtx<F, O>, mut terms: Vec<Self::Term>) -> Result<Self>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder,
    {
        let n = ctx.nvars;

        for t in &terms {
            let got = t.mono().exponents().len();
            if got != n {
                return Err(PolynomialError::Monomial(MonomialError::MismatchedArity {
                    lhs: n,
                    rhs: got,
                }));
            }
        }

        normalize_terms_in(ctx, &mut terms)?;

        let mut storage = S::default();
        storage.set_from_vec(terms);

        Ok(Self::from_storage(ctx.id(), storage))
    }

    #[inline]
    fn push_term_raw(&mut self, t: Self::Term) {
        self.storage.push(t);
    }

    #[inline]
    fn normalize_in_place<F, O>(&mut self, ctx: &RingCtx<F, O>) -> Result<()>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id)
            .map_err(PolynomialError::from)?;

        self.storage.with_vec(|v| normalize_terms_in(ctx, v))?;
        Ok(())
    }
}

impl<T, S> PartialEq for Polynomial<T, S>
where
    S: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ring_id == other.ring_id && self.storage == other.storage
    }
}

impl<T, S> Eq for Polynomial<T, S> where S: Eq {}
