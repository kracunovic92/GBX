//! S-polynomial construction.
//!
//! This module provides [`s_polynomial_in`], which computes the S-polynomial
//! of two polynomials inside a ring context.
//!
//! The computation is context-driven:
//!
//! - coefficient arithmetic is performed via `ctx.field`,
//! - inputs are checked against the ring identity tag stored in each polynomial,
//! - the returned polynomial is normalized before being returned.

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialError};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialError, PolynomialOps};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};
use thiserror::Error;

/// Errors that can occur while constructing an S-polynomial.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum SPolyError {
    /// Propagated polynomial, term, monomial, or ring error.
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    /// One input polynomial was zero.
    #[error("cannot form S-polynomial: one input polynomial is zero")]
    ZeroInput,

    /// A leading coefficient was not invertible in `ctx.field`.
    #[error("cannot form S-polynomial: leading coefficient is not invertible")]
    NonInvertibleLeadingCoefficient,
}

impl From<MonomialError> for SPolyError {
    #[inline]
    fn from(e: MonomialError) -> Self {
        Self::Poly(PolynomialError::from(e))
    }
}

/// Compute the S-polynomial `S(f, g)` in the given ring context.
///
/// The inputs must belong to the same ring as `ctx`.
///
/// The result is normalized before being returned.
///
/// # Errors
///
/// Returns [`SPolyError`] if:
///
/// - `f` or `g` is the zero polynomial,
/// - a leading coefficient is not invertible in `ctx.field`,
/// - ring-tag validation fails,
/// - monomial or polynomial arithmetic fails.
pub fn s_polynomial_in<P, F, O>(ctx: &RingCtx<F, O>, f: &P, g: &P) -> Result<P, SPolyError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + Clone + Eq,
{
    // Validate ring tags.
    ctx.assert_same_ring_id(f.ring_id())
        .map_err(PolynomialError::from)?;
    ctx.assert_same_ring_id(g.ring_id())
        .map_err(PolynomialError::from)?;

    // Extract leading data.
    let lt_f = f.leading_term().ok_or(SPolyError::ZeroInput)?;
    let lt_g = g.leading_term().ok_or(SPolyError::ZeroInput)?;

    let lm_f = lt_f.mono();
    let lm_g = lt_g.mono();
    let lc_f = *lt_f.coeff();
    let lc_g = *lt_g.coeff();

    // Compute l = lcm(lm(f), lm(g)) and the monomial quotients.
    let l = lm_f.checked_lcm(lm_g)?;
    let mf = l.checked_div_exact_by(lm_f)?;
    let mg = l.checked_div_exact_by(lm_g)?;

    // Invert leading coefficients in the field context.
    let inv_lc_f = ctx
        .field
        .try_inv(lc_f)
        .ok_or(SPolyError::NonInvertibleLeadingCoefficient)?;
    let inv_lc_g = ctx
        .field
        .try_inv(lc_g)
        .ok_or(SPolyError::NonInvertibleLeadingCoefficient)?;

    // Form inv(lc(f)) * mf * f.
    let mut scaled_f = f.clone();
    scaled_f.mul_monomial_assign_raw(ctx, &mf)?;
    scaled_f.scale_assign_raw(ctx, inv_lc_f)?;

    // Form inv(lc(g)) * mg * g.
    let mut scaled_g = g.clone();
    scaled_g.mul_monomial_assign_raw(ctx, &mg)?;
    scaled_g.scale_assign_raw(ctx, inv_lc_g)?;

    // Subtract and normalize.
    scaled_f.sub_assign_raw(ctx, &scaled_g)?;
    scaled_f.normalize_in_place(ctx)?;

    Ok(scaled_f)
}
