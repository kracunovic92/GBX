use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::update::update_with_polynomial;
use crate::algos::f4::state::F4State;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Initializes the F4 state from the input polynomials.
///
/// Takes raw input generators, removes zero polynomials, optionally normalizes
/// them, inserts the surviving polynomials into the initial basis, and creates
/// the initial pending critical pairs.
///
/// Returns an `F4State` with initialized `basis`, `pending`, empty `history`,
/// and `iteration = 0`.
#[cfg_attr(feature = "instrumentation", tracing::instrument(level = "debug", name = "f4.initialize_state", skip(ctx, fs, opts, criterion),))]
pub fn initialize_state<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: &F4Options, criterion: ProductCriterion) -> Result<F4State<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
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

        update_with_polynomial(&mut state.basis, &mut state.pending, f, &criterion)?;
    }

    Ok(state)
}
