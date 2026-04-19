use crate::algos::post::minimize::minimize_in_place;
use crate::algos::post::PostError;
use crate::GrobnerBasis;
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialOps, PolynomialReduce};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Fully reduce a Gröbner basis in place.
///
/// This performs:
/// 1. removal of zero basis elements,
/// 2. minimization,
/// 3. reduction of each basis element by the others,
/// 4. final normalization of all surviving elements.
pub fn reduce_in_place<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    gb.retain(|p| !p.is_zero());

    if gb.is_empty() {
        return Ok(());
    }

    minimize_in_place(ctx, gb)?;

    let n = gb.len();
    let mut out = Vec::with_capacity(n);

    for i in 0..n {
        let gi = gb.as_slice()[i].clone();

        let it = gb
            .as_slice()
            .iter()
            .enumerate()
            .filter_map(|(j, g)| (j != i).then_some(g));

        let mut r = gi.normal_form(ctx, it)?;
        r.normalize_in_place(ctx)?;

        if !r.is_zero() {
            out.push(r);
        }
    }

    *gb = GrobnerBasis::new(ctx.id(), out);
    // Important: reduction may change leading monomials and create new redundancies.
    gb.retain(|p| !p.is_zero());
    minimize_in_place(ctx, gb)?;

    for p in gb.as_mut_vec() {
        p.normalize_in_place(ctx)?;
    }

    Ok(())
}
