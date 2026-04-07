use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::update::update_with_polynomial;
use crate::algos::f4::state::F4State;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

pub fn initialize_state<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: &F4Options, criterion: &ProductCriterion) -> Result<F4State<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Default,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    let mut state = F4State::new();

    for mut f in fs {
        if f.is_zero() {
            continue;
        }

        if opts.normalize_inputs {
            f.normalize_in_place(ctx)?;
        }

        if f.is_zero() {
            continue;
        }

        update_with_polynomial(&mut state.basis, &mut state.pending, f, criterion)?;
    }

    Ok(state)
}
