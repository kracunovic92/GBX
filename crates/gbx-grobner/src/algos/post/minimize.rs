use crate::algos::post::PostError;
use crate::GrobnerBasis;
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialOps;
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Minimalize a Gröbner basis in place.
///
/// Removes every polynomial whose leading monomial is divisible by the leading
/// monomial of another basis element. The surviving basis elements are then
/// made monic.
pub fn minimize_in_place<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + Clone,
    P::Term: TermView + TermOwned + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
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

    let lms: Vec<<P::Term as TermView>::Mono> = polys
        .iter()
        .map(|p| {
            p.leading_term()
                .ok_or(PostError::InvariantViolation)
                .map(|lt| lt.mono().clone())
        })
        .collect::<Result<Vec<_>, PostError>>()?;

    let out: Vec<P> = polys
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let redundant = lms
                .iter()
                .enumerate()
                .any(|(j, lmj)| j != i && lmj.divides(&lms[i]));
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
pub fn make_monic_in_place<P, F, O>(ctx: &RingCtx<F, O>, p: &mut P) -> Result<(), PostError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + Clone,
    P::Term: TermView + TermOwned + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    if p.is_zero() {
        return Ok(());
    }

    let lt = p.leading_term().ok_or(PostError::InvariantViolation)?;
    let lc = lt.coeff();

    let one = ctx.field.one();
    if *lc == one {
        return Ok(());
    }

    let inv_lc = ctx
        .field
        .checked_div(one, *lc)
        .map_err(PostError::DivByZero)?;

    p.scale_in_place(ctx, inv_lc)?;

    Ok(())
}
