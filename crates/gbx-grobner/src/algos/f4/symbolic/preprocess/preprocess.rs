use crate::algos::f4::error::Result;
use crate::algos::f4::simplify::SimplifyIndex;
use crate::state::BatchHistory;
use crate::symbolic::{SymbolicPreprocessOutput, SymbolicPreprocessState, UnevaluatedProduct};
use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Builds the symbolic preprocessing row set `F_d` for one F4 batch.
///
/// Input:
/// - `l_d`: unevaluated products `L_d = Left(P_d) ∪ Right(P_d)`.
/// - `basis`: current basis `G`.
/// - `history`: previous batches, used to materialize historical row sources.
/// - `simplify_index`: rewrite rules compiled from previous batches.
///
/// Output:
/// - materialized rows of `F_d`,
/// - leading monomials `HT(F_d)`.
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
