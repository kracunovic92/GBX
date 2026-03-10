use crate::{BuchbergerError, GrobnerBasis};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialOps;
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Minimalize: remove any polynomial whose LM is divisible by another LM.
/// Then make all remaining polynomials monic.
pub fn minimize_in_place<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), BuchbergerError>
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
                .ok_or(BuchbergerError::InvariantViolation)
                .map(|lt| lt.mono().clone())
        })
        .collect::<Result<_, _>>()?;

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

    // Make monic at the end.
    for p in gb.as_mut_vec() {
        make_monic_in_place(&ctx, p)?;
    }

    Ok(())
}
pub fn make_monic_in_place<P, F, O>(ctx: &RingCtx<F, O>, p: &mut P) -> Result<(), BuchbergerError>
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

    let lt = p
        .leading_term()
        .ok_or(BuchbergerError::InvariantViolation)?;
    let lc = lt.coeff();

    let one = ctx.field.one();
    if *lc == one {
        return Ok(());
    }

    let inv_lc = ctx
        .field
        .checked_div(one, *lc)
        .map_err(|_| BuchbergerError::InvariantViolation)?;

    p.scale_in_place(ctx, inv_lc)
        .map_err(BuchbergerError::from)?;

    Ok(())
}
