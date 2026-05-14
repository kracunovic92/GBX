//! Public symbolic preprocessing entry point.

use crate::algos::f4::error::Result;
use crate::algos::f4::simplify::SimplifyIndex;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::preprocess::state::SymbolicPreprocessState;
use crate::algos::f4::symbolic::{SymbolicPreprocessOutput, UnevaluatedProduct};

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Builds the symbolic row set for one F4 batch.
///
/// The input products are simplified before materialization. The resulting row
/// set is closed under top reduction by the current basis.
pub fn symbolic_preprocess<P, F, O>(
    ctx: &RingCtx<F, O>,
    l_d: &[UnevaluatedProduct<Monomial>],
    basis: &[P],
    history: &[BatchHistory<P>],
    simplify_index: &SimplifyIndex,
) -> Result<SymbolicPreprocessOutput<P, Monomial>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    let mut state = SymbolicPreprocessState::new(ctx, basis, history, simplify_index);

    state.add_initial_products(l_d)?;
    state.close_under_top_reduction()?;

    Ok(state.finish())
}
