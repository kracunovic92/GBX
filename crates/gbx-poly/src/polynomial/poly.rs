//! Sparse polynomial container.

use crate::monomial::MonomialError;
use crate::order::MonomialOrder;
use crate::polynomial::error::{PolynomialError, PolynomialResult};
use crate::polynomial::normalize::normalize_terms_in;
use crate::polynomial::traits::{PolynomialMut, PolynomialView};
use crate::ring::{FieldCtx, RingCtx, RingId};
use crate::term::Term;

/// Sparse polynomial with coefficient type `C`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<C> {
    ring_id: RingId,
    terms: Vec<Term<C>>,
}

impl<C: Copy + Eq> Polynomial<C> {
    /// Creates a polynomial from already-owned parts.
    ///
    /// This does not normalize.
    #[inline]
    #[must_use]
    pub const fn from_raw_parts(ring_id: RingId, terms: Vec<Term<C>>) -> Self {
        Self { ring_id, terms }
    }

    /// Borrows the underlying terms.
    #[inline]
    #[must_use]
    pub fn terms_slice(&self) -> &[Term<C>] {
        &self.terms
    }

    /// Mutably borrows the underlying terms.
    ///
    /// Direct mutation may break canonical invariants.
    #[inline]
    pub const fn terms_mut(&mut self) -> &mut Vec<Term<C>> {
        &mut self.terms
    }

    /// Returns this polynomial's ring id tag.
    #[inline]
    #[must_use]
    pub const fn ring_id_tag(&self) -> RingId {
        self.ring_id
    }

    /// Checks that this polynomial belongs to `ctx`.
    ///
    /// # Errors
    ///
    /// Returns [`PolynomialError::Ring`] when the polynomial was created in a
    /// different ring context.
    #[inline]
    pub fn assert_same_ring<F, O>(&self, ctx: &RingCtx<F, O>) -> PolynomialResult<()>
    where
        F: FieldCtx,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id)
            .map_err(PolynomialError::from)
    }

    /// Creates the zero polynomial in `ctx`.
    #[inline]
    pub fn zero_in<F, O>(ctx: &RingCtx<F, O>) -> Self
    where
        F: FieldCtx<Elem = C>,
    {
        <Self as PolynomialMut>::zero_in(ctx)
    }

    /// Builds and normalizes a polynomial in `ctx`.
    ///
    /// # Errors
    ///
    /// Returns an error if any term has the wrong number of variables or
    /// normalization fails.
    #[inline]
    pub fn from_terms_in<F, O>(ctx: &RingCtx<F, O>, terms: Vec<Term<C>>) -> PolynomialResult<Self>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
        C: Copy + Eq,
    {
        <Self as PolynomialMut>::from_terms_in(ctx, terms)
    }
}

impl<C> PolynomialView for Polynomial<C> {
    type Coeff = C;

    #[inline]
    fn ring_id(&self) -> RingId {
        self.ring_id
    }

    #[inline]
    fn terms(&self) -> &[Term<C>] {
        &self.terms
    }
}

impl<C> PolynomialMut for Polynomial<C>
where
    C: Copy + Eq,
{
    #[inline]
    fn zero_in<F, O>(ctx: &RingCtx<F, O>) -> Self
    where
        F: FieldCtx<Elem = C>,
    {
        Self { ring_id: ctx.id(), terms: Vec::new() }
    }

    #[inline]
    fn from_terms_in<F, O>(ctx: &RingCtx<F, O>, mut terms: Vec<Term<C>>) -> PolynomialResult<Self>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        let expected = ctx.nvars;

        for t in &terms {
            let got = t.mono().n_vars();

            if got != expected {
                return Err(PolynomialError::Monomial(MonomialError::MismatchedArity {
                    lhs: expected,
                    rhs: got,
                }));
            }
        }

        normalize_terms_in(ctx, &mut terms)?;

        Ok(Self { ring_id: ctx.id(), terms })
    }

    #[inline]
    fn push_term_raw(&mut self, t: Term<C>) {
        self.terms.push(t);
    }

    #[inline]
    fn normalize_in_place<F, O>(&mut self, ctx: &RingCtx<F, O>) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id)
            .map_err(PolynomialError::from)?;

        normalize_terms_in(ctx, &mut self.terms)
    }
}
