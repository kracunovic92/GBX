use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialError, PolynomialOps};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

use super::SPolyError;

/// Compute the S-polynomial `S(f, g)` in the given ring context.
///
/// The inputs must belong to the same ring as `ctx` and must be nonzero.
///
/// Mathematically:
///
/// `S(f, g) = (lcm(lm(f), lm(g)) / lt(f)) * f - (lcm(lm(f), lm(g)) / lt(g)) * g`
///
/// where division by a leading term includes:
///
/// - exact monomial division, and
/// - multiplication by the inverse of the leading coefficient.
///
/// # Canonical form
///
/// This function does **not** normalize the result before returning.
/// This avoids unnecessary work in Buchberger-style pipelines where the
/// S-polynomial is reduced immediately afterward.
///
/// If canonical form is required, call `normalize_in_place(ctx)` on the result.
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
    validate_ring_tags(ctx, f, g)?;

    let lt_f = f.leading_term().ok_or(SPolyError::ZeroInput)?;
    let lt_g = g.leading_term().ok_or(SPolyError::ZeroInput)?;

    let lm_f = lt_f.mono();
    let lm_g = lt_g.mono();
    let lc_f = *lt_f.coeff();
    let lc_g = *lt_g.coeff();

    let lcm = lm_f.checked_lcm(lm_g)?;
    let mult_f = lcm.checked_div_exact_by(lm_f)?;
    let mult_g = lcm.checked_div_exact_by(lm_g)?;

    let inv_lc_f = invert_leading_coeff(ctx, lc_f)?;
    let inv_lc_g = invert_leading_coeff(ctx, lc_g)?;

    let left = scaled_multiple(ctx, f, &mult_f, inv_lc_f)?;
    let right = scaled_multiple(ctx, g, &mult_g, inv_lc_g)?;

    subtract_raw(ctx, left, &right)
}

#[inline]
fn validate_ring_tags<P, F, O>(ctx: &RingCtx<F, O>, f: &P, g: &P) -> Result<(), SPolyError>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialOps + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    ctx.assert_same_ring_id(f.ring_id())
        .map_err(PolynomialError::from)?;
    ctx.assert_same_ring_id(g.ring_id())
        .map_err(PolynomialError::from)?;
    Ok(())
}

#[inline]
fn invert_leading_coeff<F, O>(ctx: &RingCtx<F, O>, coeff: F::Elem) -> Result<F::Elem, SPolyError>
where
    F: FieldCtx,
    O: MonomialOrder,
{
    ctx.field
        .try_inv(coeff)
        .ok_or(SPolyError::NonInvertibleLeadingCoefficient)
}

#[inline]
fn scaled_multiple<P, F, O>(ctx: &RingCtx<F, O>, poly: &P, monomial: &<P::Term as TermView>::Mono, scalar: <P::Term as TermView>::Coeff) -> Result<P, SPolyError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + Clone + Eq,
{
    let mut out = poly.clone();
    out.mul_monomial_assign_raw(ctx, monomial)?;
    out.scale_assign_raw(ctx, scalar)?;
    Ok(out)
}

#[inline]
fn subtract_raw<P, F, O>(ctx: &RingCtx<F, O>, mut left: P, right: &P) -> Result<P, SPolyError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + Clone + Eq,
{
    left.sub_assign_raw(ctx, right)?;
    Ok(left)
}
