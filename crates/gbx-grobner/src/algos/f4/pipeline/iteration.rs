use crate::algos::f4::error::Result;
use crate::algos::f4::options::ValidatedF4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::selector::MinDegreeSelector;
use crate::algos::f4::pipeline::insert::insert_new_rows;
use crate::algos::f4::pipeline::reduction::{ReductionPhase, reduction_phase};
use crate::algos::f4::pipeline::select::select_pairs_phase;
use crate::algos::f4::state::F4State;
#[cfg(feature = "instrumentation")]
use crate::f4_info;
use crate::instrumentation::alloc::with_alloc_profile;
use crate::linear::BatchReducer;
use crate::types::IterationOutcome;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Runs one F4 pair-processing iteration.
pub fn run_iteration<P, F, O, R>(
    ctx: &RingCtx<F, O>,
    state: &mut F4State<P>,
    opts: &ValidatedF4Options,
    selector: &mut MinDegreeSelector,
    reducer: &R,
    criterion: ProductCriterion,
) -> Result<IterationOutcome>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
    R: BatchReducer<P, F, O>,
{
    state.advance_iteration();

    let batch = select_pairs_phase(state, selector);

    if batch.selected_pairs.is_empty() {
        return Ok(IterationOutcome::Done);
    }

    let phase = with_alloc_profile("f4.reduction_phase", || {
        reduction_phase(
            ctx,
            opts,
            &batch.l_d,
            &state.basis,
            &state.history,
            reducer,
            &state.simplify_index,
        )
    })?;

    apply_reduction_phase(ctx, state, phase, criterion)?;

    Ok(IterationOutcome::Progress)
}

fn apply_reduction_phase<P, F, O>(ctx: &RingCtx<F, O>, state: &mut F4State<P>, phase: ReductionPhase<P>, criterion: ProductCriterion) -> Result<()>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
{
    let ReductionPhase { f_d_products, f_d_heads, f_d_tilde, extracted_rows } = phase;

    state.push_history(f_d_products, f_d_heads, f_d_tilde);

    #[cfg(feature = "instrumentation")]
    let basis_before_insert = state.basis.len();

    #[cfg(feature = "instrumentation")]
    let pending_before_insert = state.pending.len();

    insert_new_rows(ctx, state, extracted_rows, criterion)?;

    #[cfg(feature = "instrumentation")]
    f4_info!(
        basis_before_insert,
        basis_after_insert = state.basis.len(),
        pending_before_insert,
        pending_after_insert = state.pending.len(),
        "f4.iteration.after_insert"
    );

    Ok(())
}
