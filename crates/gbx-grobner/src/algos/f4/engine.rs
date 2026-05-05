use crate::algos::f4::error::Result;
use crate::algos::f4::options::ValidatedF4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::selector::MinDegreeSelector;
use crate::algos::f4::pipeline::{initialize_state, post_process_basis, run_iteration};
use crate::basis::GrobnerBasis;

use crate::instrumentation::alloc::with_alloc_profile;

use crate::linear::roman_sparse::{RomanParallelSparseBufferReducer, RomanSparseBufferReducer};
use crate::linear::BatchReducer;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Run the F4 engine with validated options.
use crate::algos::f4::linear::DenseF4MatrixReducer;
use crate::algos::f4::options::F4ReducerKind;


pub fn run_f4<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: ValidatedF4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send + Sync,
    P::Coeff: Copy + Eq + Default + Send + Sync,
{
    match opts.reducer_kind {
        F4ReducerKind::Dense => {
            let reducer = DenseF4MatrixReducer;
            run_f4_with_reducer(ctx, fs, opts, &reducer)
        }

        F4ReducerKind::Roman => {
            let reducer = RomanSparseBufferReducer;
            run_f4_with_reducer(ctx, fs, opts, &reducer)
        }

        F4ReducerKind::RomanParallel => {
            let reducer = RomanParallelSparseBufferReducer;
            run_f4_with_reducer(ctx, fs, opts, &reducer)
        }
    }
}

pub fn run_f4_with_reducer<P, F, O, R>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: ValidatedF4Options, reducer: &R) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
    R: BatchReducer<P, F, O>,
{
    let criterion = ProductCriterion;
    let mut selector = MinDegreeSelector::new(opts.batch_size);

    let mut state = initialize_state(ctx, fs, &opts, &criterion)?;

    if state.basis.is_empty() {
        return Ok(GrobnerBasis::empty_in(ctx));
    }

    while !state.is_done() {
        run_iteration(ctx, &mut state, &opts, &mut selector, reducer, &criterion)?;
    }

    tracing::info!("f4.iteration.end");

    post_process_basis(ctx, state.basis, &opts)
}
