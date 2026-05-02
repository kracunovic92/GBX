use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::selector::MinDegreeSelector;
use crate::algos::f4::pipeline::insert::insert_new_rows;
use crate::algos::f4::pipeline::reduction::{reduction_phase, ReductionPhase};
use crate::algos::f4::pipeline::select::select_pairs_phase;
use crate::algos::f4::state::F4State;

use crate::instrumentation::alloc::with_alloc_profile;
use crate::linear::DenseF4MatrixReducer;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

pub fn run_iteration<P, F, O>(
    ctx: &RingCtx<F, O>,
    state: &mut F4State<P>,
    opts: &F4Options,
    selector: &mut MinDegreeSelector,
    reducer: &DenseF4MatrixReducer,
    criterion: &ProductCriterion,
) -> Result<()>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
{
    state.advance_iteration();

    let pair_selection = with_alloc_profile("f4.select_pairs_phase", || {
        select_pairs_phase(state, selector)
    });

    if pair_selection.selected_pairs.is_empty() {
        return Ok(());
    }

    let ReductionPhase { f_d, f_d_tilde, extracted_rows } = with_alloc_profile("f4.reduction_phase", || {
        reduction_phase(
            ctx,
            opts,
            &pair_selection.l_d,
            &state.basis,
            &state.history,
            reducer,
        )
    })?;

    state.push_history(f_d.rows, f_d_tilde);

    insert_new_rows(ctx, state, extracted_rows, criterion)?;

    Ok(())
}
