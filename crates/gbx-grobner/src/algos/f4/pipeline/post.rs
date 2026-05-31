use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::basis::GrobnerBasis;
use crate::{BasisPostOptionsKind, minimize_in_place, reduce_in_place};

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialReduce, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

pub fn post_process_basis<P, F, O>(ctx: &RingCtx<F, O>, basis: Vec<P>, opts: &F4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialReduce + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    let mut gb = GrobnerBasis::new(ctx.id(), basis);

    match opts.post {
        BasisPostOptionsKind::None => {}
        BasisPostOptionsKind::Minimal => minimize_in_place(ctx, &mut gb)?,
        BasisPostOptionsKind::Reduced => reduce_in_place(ctx, &mut gb)?,
    }

    Ok(gb)
}
