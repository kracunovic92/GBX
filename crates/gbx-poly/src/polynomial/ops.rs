//! Polynomial arithmetic operations (context-driven).
//!
//! These operations are intentionally minimal: they provide the hooks needed by
//! reduction / Gröbner basis algorithms.
//!
//! - `*_raw` methods may break canonical invariants.
//! - canonical helpers call `normalize_in_place(ctx)`.
//!
//! # Errors
//! Most methods check ring id consistency and return `PolynomialError::Ring` when mixing rings.
//!
//! # Panics
//! None.

use crate::monomial::{Monomial, MonomialAlgos, MonomialView};
use crate::order::MonomialOrder;
use crate::polynomial::error::{PolynomialError, Result};
use crate::polynomial::poly::Polynomial;
use crate::polynomial::traits::{PolynomialMut, PolynomialView};
use crate::ring::{FieldCtx, RingCtx};
use crate::term::{TermOwned, TermView};
use gbx_storage::polynomial::TermStorage;

/// Minimal arithmetic hooks required by reduction / Gröbner algorithms.
pub trait PolynomialOps: PolynomialMut
where
    Self: Sized + Clone,
    Self::Term: TermOwned + TermView + Clone,
    <Self::Term as TermView>::Mono: Monomial + MonomialAlgos + Clone + Eq,
    <Self::Term as TermView>::Coeff: Copy + Eq,
{
    /// Remove and return the current leading term (if any).
    fn pop_leading_term_raw<F, O>(&mut self, ctx: &RingCtx<F, O>) -> Result<Option<Self::Term>>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder;

    /// self += rhs (raw; may be unnormalized afterwards).
    fn add_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<()>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder;

    /// self -= rhs (raw; may be unnormalized afterwards).
    fn sub_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<()>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder;

    /// Multiply all coefficients by `c` (raw).
    fn scale_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, c: <Self::Term as TermView>::Coeff) -> Result<()>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder;

    /// Multiply all monomials by `m` (raw).
    fn mul_monomial_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, m: &<Self::Term as TermView>::Mono) -> Result<()>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder;

    /// Naive product (returns unnormalized polynomial).
    fn mul_raw<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<Self>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder;

    // ---- Canonical wrappers ----

    /// self + rhs with canonical output (normalized).
    #[inline]
    fn add_canonical<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<Self>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder,
    {
        let mut out = self.clone();
        out.add_assign_raw(ctx, rhs)?;
        out.normalize_in_place(ctx)?;
        Ok(out)
    }

    /// self - rhs with canonical output (normalized).
    #[inline]
    fn sub_canonical<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<Self>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder,
    {
        let mut out = self.clone();
        out.sub_assign_raw(ctx, rhs)?;
        out.normalize_in_place(ctx)?;
        Ok(out)
    }

    /// self * rhs with canonical output (normalized).
    #[inline]
    fn mul_canonical<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<Self>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder,
    {
        let mut out = self.mul_raw(ctx, rhs)?;
        out.normalize_in_place(ctx)?;
        Ok(out)
    }
    /// Multiply all coefficients by `c` (canonical; normalizes if needed).
    #[inline]
    fn scale_in_place<F, O>(&mut self, ctx: &RingCtx<F, O>, c: <Self::Term as TermView>::Coeff) -> Result<()>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder,
    {
        self.scale_assign_raw(ctx, c)?;
        self.normalize_in_place(ctx)?;
        Ok(())
    }
}
impl<T, S> PolynomialOps for Polynomial<T, S>
where
    T: TermOwned + TermView + Clone,
    T::Coeff: Copy + Eq,
    T::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    S: TermStorage<T> + Default,
    Self: Clone,
{
    #[inline]
    fn pop_leading_term_raw<F, O>(&mut self, ctx: &RingCtx<F, O>) -> Result<Option<Self::Term>>
    where
        F: FieldCtx<Elem = T::Coeff>,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id())
            .map_err(PolynomialError::from)?;

        self.storage_mut()
            .with_vec(|v| Ok(if v.is_empty() { None } else { Some(v.remove(0)) }))
    }

    #[inline]
    fn add_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<()>
    where
        F: FieldCtx<Elem = T::Coeff>,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id())
            .map_err(PolynomialError::from)?;
        ctx.assert_same_ring_id(rhs.ring_id())
            .map_err(PolynomialError::from)?;

        self.storage_mut().with_vec(|v| {
            v.extend(rhs.terms().iter().cloned());
            Ok(())
        })
    }

    #[inline]
    fn sub_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<()>
    where
        F: FieldCtx<Elem = T::Coeff>,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id())
            .map_err(PolynomialError::from)?;
        ctx.assert_same_ring_id(rhs.ring_id())
            .map_err(PolynomialError::from)?;

        self.storage_mut().with_vec(|v| {
            v.extend(rhs.terms().iter().cloned().map(|t| {
                let neg = ctx.field.neg(*t.coeff());
                T::from_parts(neg, t.mono().clone())
            }));
            Ok(())
        })
    }

    #[inline]
    fn scale_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, c: T::Coeff) -> Result<()>
    where
        F: FieldCtx<Elem = T::Coeff>,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id())
            .map_err(PolynomialError::from)?;

        self.storage_mut().with_vec(|v| {
            for i in 0..v.len() {
                let coeff = ctx.field.mul(*v[i].coeff(), c);
                let mono = v[i].mono().clone();
                v[i] = T::from_parts(coeff, mono);
            }
            Ok(())
        })
    }

    #[inline]
    fn mul_monomial_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, m: &T::Mono) -> Result<()>
    where
        F: FieldCtx<Elem = T::Coeff>,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id())
            .map_err(PolynomialError::from)?;

        self.storage_mut().with_vec(|v| {
            for i in 0..v.len() {
                let mono = v[i]
                    .mono()
                    .clone()
                    .checked_mul(m)
                    .map_err(PolynomialError::from)?;
                let coeff = *v[i].coeff();
                v[i] = T::from_parts(coeff, mono);
            }
            Ok(())
        })
    }

    #[inline]
    fn mul_raw<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> Result<Self>
    where
        F: FieldCtx<Elem = T::Coeff>,
        O: MonomialOrder,
    {
        ctx.assert_same_ring_id(self.ring_id())
            .map_err(PolynomialError::from)?;
        ctx.assert_same_ring_id(rhs.ring_id())
            .map_err(PolynomialError::from)?;

        if self.is_zero() || rhs.is_zero() {
            return Ok(Self::zero_in(ctx));
        }

        let mut out_terms: Vec<T> = Vec::with_capacity(self.terms().len() * rhs.terms().len());

        for a in self.terms().iter() {
            for b in rhs.terms().iter() {
                let coeff = ctx.field.mul(*a.coeff(), *b.coeff());
                let mono = a
                    .mono()
                    .clone()
                    .checked_mul(b.mono())
                    .map_err(PolynomialError::from)?;
                out_terms.push(T::from_parts(coeff, mono));
            }
        }

        let mut storage = S::default();
        storage.set_from_vec(out_terms);
        Ok(Self::from_storage(ctx.id(), storage))
    }
}
