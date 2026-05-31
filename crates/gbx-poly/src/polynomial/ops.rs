//! Polynomial arithmetic operations.

use crate::monomial::Monomial;
use crate::order::MonomialOrder;
use crate::polynomial::Polynomial;
use crate::polynomial::error::{PolynomialError, PolynomialResult};
use crate::polynomial::traits::{PolynomialMut, PolynomialView};
use crate::ring::{FieldCtx, RingCtx};
use crate::term::Term;

/// Minimal arithmetic hooks required by reduction and Gröbner algorithms.
#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
pub trait PolynomialOps: PolynomialMut + Clone
where
    Self::Coeff: Copy + Eq,
{
    fn pop_leading_term_raw<F, O>(&mut self, ctx: &RingCtx<F, O>) -> PolynomialResult<Option<Term<Self::Coeff>>>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder;

    fn add_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder;

    fn sub_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder;

    fn scale_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, c: Self::Coeff) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder;

    fn mul_monomial_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, m: &Monomial) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder;

    fn mul_raw<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<Self>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder;

    #[inline]
    fn add_canonical<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<Self>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder,
    {
        let mut out = self.clone();
        out.add_assign_raw(ctx, rhs)?;
        out.normalize_in_place(ctx)?;
        Ok(out)
    }

    #[inline]
    fn sub_canonical<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<Self>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder,
    {
        let mut out = self.clone();
        out.sub_assign_raw(ctx, rhs)?;
        out.normalize_in_place(ctx)?;
        Ok(out)
    }

    #[inline]
    fn mul_canonical<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<Self>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder,
    {
        let mut out = self.mul_raw(ctx, rhs)?;
        out.normalize_in_place(ctx)?;
        Ok(out)
    }

    #[inline]
    fn scale_in_place<F, O>(&mut self, ctx: &RingCtx<F, O>, c: Self::Coeff) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder,
    {
        self.scale_assign_raw(ctx, c)?;
        self.normalize_in_place(ctx)
    }

    fn sub_scaled_monomial_multiple_in_place<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self, mono_mul: &Monomial, coeff_mul: Self::Coeff) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder;
}

impl<C> PolynomialOps for Polynomial<C>
where
    C: Copy + Eq,
{
    #[inline]
    fn pop_leading_term_raw<F, O>(&mut self, ctx: &RingCtx<F, O>) -> PolynomialResult<Option<Term<C>>>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        self.assert_same_ring(ctx)?;

        if self.terms_mut().is_empty() { Ok(None) } else { Ok(Some(self.terms_mut().remove(0))) }
    }

    #[inline]
    fn add_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        self.assert_same_ring(ctx)?;
        rhs.assert_same_ring(ctx)?;

        self.terms_mut().extend(rhs.terms().iter().cloned());

        Ok(())
    }

    #[inline]
    fn sub_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        self.assert_same_ring(ctx)?;
        rhs.assert_same_ring(ctx)?;

        self.terms_mut().extend(
            rhs.terms()
                .iter()
                .map(|t| Term::new(ctx.field.neg(*t.coeff()), t.mono().clone())),
        );

        Ok(())
    }

    #[inline]
    fn scale_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, c: C) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        self.assert_same_ring(ctx)?;

        for t in self.terms_mut() {
            let coeff = ctx.field.mul(*t.coeff(), c);
            *t = Term::new(coeff, t.mono().clone());
        }

        Ok(())
    }

    #[inline]
    fn mul_monomial_assign_raw<F, O>(&mut self, ctx: &RingCtx<F, O>, m: &Monomial) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        self.assert_same_ring(ctx)?;

        for t in self.terms_mut() {
            let mono = t.mono().checked_mul(m).map_err(PolynomialError::from)?;
            *t = Term::new(*t.coeff(), mono);
        }

        Ok(())
    }

    #[inline]
    fn mul_raw<F, O>(&self, ctx: &RingCtx<F, O>, rhs: &Self) -> PolynomialResult<Self>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        self.assert_same_ring(ctx)?;
        rhs.assert_same_ring(ctx)?;

        if self.is_zero() || rhs.is_zero() {
            return Ok(Self::zero_in(ctx));
        }

        let mut out_terms = Vec::with_capacity(self.len() * rhs.len());

        for a in self.terms() {
            for b in rhs.terms() {
                let coeff = ctx.field.mul(*a.coeff(), *b.coeff());
                let mono = a
                    .mono()
                    .checked_mul(b.mono())
                    .map_err(PolynomialError::from)?;

                out_terms.push(Term::new(coeff, mono));
            }
        }

        Ok(Self::from_raw_parts(ctx.id(), out_terms))
    }

    #[inline]
    fn sub_scaled_monomial_multiple_in_place<F, O>(&mut self, ctx: &RingCtx<F, O>, rhs: &Self, mono_mul: &Monomial, coeff_mul: C) -> PolynomialResult<()>
    where
        F: FieldCtx<Elem = C>,
        O: MonomialOrder,
    {
        self.assert_same_ring(ctx)?;
        rhs.assert_same_ring(ctx)?;

        self.terms_mut().reserve(rhs.len());

        for t in rhs.terms() {
            let coeff = ctx.field.neg(ctx.field.mul(*t.coeff(), coeff_mul));
            let mono = t
                .mono()
                .checked_mul(mono_mul)
                .map_err(PolynomialError::from)?;

            self.terms_mut().push(Term::new(coeff, mono));
        }

        Ok(())
    }
}
