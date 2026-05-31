use crate::GrobnerBasis;
use crate::algos::post::PostError;

use gbx_poly::monomial::{Monomial, divides};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Minimalize a Gröbner basis in place.
///
/// Removes every polynomial whose leading monomial is divisible by the leading
/// monomial of another basis element. The surviving basis elements are then
/// made monic.
///
/// # Errors
///
/// Returns an error if leading monomials are missing unexpectedly or if making a
/// survivor monic fails.
pub fn minimize_in_place<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq,
{
    let polys: Vec<P> = gb
        .take_polys()
        .into_iter()
        .filter(|p| !p.is_zero())
        .collect();

    match polys.len() {
        0 => {
            *gb = GrobnerBasis::new(ctx.id(), Vec::new());
            return Ok(());
        }
        1 => {
            let mut out = polys;
            make_monic_in_place(ctx, &mut out[0])?;
            *gb = GrobnerBasis::new(ctx.id(), out);
            return Ok(());
        }
        _ => {}
    }

    let lms: Vec<Monomial> = polys
        .iter()
        .map(|p| {
            p.leading_mono()
                .ok_or(PostError::InvariantViolation)
                .cloned()
        })
        .collect::<Result<Vec<_>, PostError>>()?;

    let out: Vec<P> = polys
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let redundant = lms
                .iter()
                .enumerate()
                .any(|(j, lmj)| j != i && divides(lmj, &lms[i]));

            (!redundant).then_some(p)
        })
        .collect();

    *gb = GrobnerBasis::new(ctx.id(), out);

    for p in gb.as_mut_vec() {
        make_monic_in_place(ctx, p)?;
    }

    Ok(())
}

/// Make one polynomial monic in place.
///
/// # Errors
///
/// Returns an error if the leading coefficient is missing, not invertible, or
/// scaling fails.
pub fn make_monic_in_place<P, F, O>(ctx: &RingCtx<F, O>, p: &mut P) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq,
{
    if p.is_zero() {
        return Ok(());
    }

    let lc = *p.leading_coeff().ok_or(PostError::InvariantViolation)?;

    let one = ctx.field.one();

    if lc == one {
        return Ok(());
    }

    let inv_lc = ctx
        .field
        .checked_div(one, lc)
        .map_err(PostError::DivByZero)?;

    p.scale_in_place(ctx, inv_lc)?;

    Ok(())
}
