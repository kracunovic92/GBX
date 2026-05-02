use crate::algos::f4::error::Result;
use crate::algos::f4::options::ValidatedF4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::selector::MinDegreeSelector;
use crate::algos::f4::pipeline::{initialize_state, post_process_basis, run_iteration};
use crate::basis::GrobnerBasis;

use crate::instrumentation::alloc::with_alloc_profile;
use crate::linear::DenseF4MatrixReducer;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Run the F4 engine with validated options.
pub fn run_f4<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: ValidatedF4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
{
    let criterion = ProductCriterion;
    let mut selector = MinDegreeSelector::new(opts.batch_size);
    let reducer = DenseF4MatrixReducer;

    let mut state = with_alloc_profile("initialize_state", || {
        initialize_state(ctx, fs, &opts, &criterion)
    })?;

    if state.basis.is_empty() {
        return Ok(GrobnerBasis::empty_in(ctx));
    }

    while !state.is_done() {
        run_iteration(ctx, &mut state, &opts, &mut selector, &reducer, &criterion)?;
    }

    post_process_basis(ctx, state.basis, &opts)
}
