use super::bounds::BuchbergerTerm;
use super::options::BuchbergerOptions;
use crate::{BuchbergerError, GrobnerBasis};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialError, PolynomialOps, PolynomialReduce};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::TermView;

pub(crate) fn init_basis<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let mut polys: Vec<P> = Vec::new();

    for mut f in fs {
        ctx.assert_same_ring_id(f.ring_id())
            .map_err(PolynomialError::from)?;

        if opts.normalize_inputs {
            f.normalize_in_place(ctx)?;
        }

        if !f.is_zero() {
            polys.push(f);
        }
    }

    Ok(GrobnerBasis::new(ctx.id(), polys))
}
