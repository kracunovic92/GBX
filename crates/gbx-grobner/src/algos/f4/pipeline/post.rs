use std::time::Instant;

use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::basis::GrobnerBasis;
use crate::{minimize_in_place, reduce_in_place, BasisPostOptionsKind};

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

pub fn post_process_basis<P, F, O>(ctx: &RingCtx<F, O>, basis: Vec<P>, opts: &F4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Default,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    let mut gb = GrobnerBasis::new(ctx.id(), basis);

    let t0 = Instant::now();
    match opts.post {
        BasisPostOptionsKind::None => {}
        BasisPostOptionsKind::Minimal => minimize_in_place(ctx, &mut gb)?,
        BasisPostOptionsKind::Reduced => reduce_in_place(ctx, &mut gb)?,
    }

    Ok(gb)
}
