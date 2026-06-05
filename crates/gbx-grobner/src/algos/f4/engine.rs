//! F4 engine driver.
//!
//! This module contains the reducer-independent control flow for the F4
//! implementation. Matrix reduction is delegated to [`BatchReducer`]
//! implementations so that dense, sparse, parallel, and experimental reducers
//! can share the same pipeline.

use crate::algos::f4::error::Result;
use crate::algos::f4::linear::DenseF4MatrixReducer;
use crate::algos::f4::options::{F4ReducerKind, ValidatedF4Options};
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::selector::MinDegreeSelector;
use crate::algos::f4::pipeline::{initialize_state, post_process_basis, run_iteration};
use crate::basis::GrobnerBasis;
use crate::f4_info;
use crate::instrumentation::profile::{count, counters, with_profile_phase};
use crate::linear::BatchReducer;
use crate::linear::roman::{RomanParallelSparseBufferReducer, RomanSparseBufferReducer};

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Computes a Gröbner basis using the F4 algorithm.
///
/// The reducer backend is selected from [`ValidatedF4Options`]. Use
/// [`run_f4_with_reducer`] when a specific reducer implementation should be
/// supplied explicitly, for example in benchmarks or reducer tests.
///
/// # Errors
///
/// Returns an error if state initialization, an F4 iteration, matrix reduction,
/// or basis post-processing fails.
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

/// Computes a Gröbner basis using an explicitly supplied F4 reducer.
///
/// This is the reducer-injection entry point used by tests, benchmarks, and
/// experimental reduction backends. The F4 control flow is identical to
/// [`run_f4`]; only the matrix-reduction implementation is provided by the
/// caller.
///
/// # Errors
///
/// Returns an error if state initialization, an F4 iteration, reducer execution,
/// or basis post-processing fails.
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

    let mut state = with_profile_phase("f4.initialize_state", counters(), || {
        initialize_state(ctx, fs, &opts, criterion)
    })?;

    if state.basis.is_empty() {
        return Ok(GrobnerBasis::empty_in(ctx));
    }

    while !state.is_done() {
        run_iteration(ctx, &mut state, &opts, &mut selector, reducer, criterion)?;
    }

    f4_info!("f4.engine.done");

    let mut post_counters = counters();
    post_counters.insert("basis_size", count(state.basis.len()));

    with_profile_phase("f4.post_process", post_counters, || {
        post_process_basis(ctx, state.basis, &opts)
    })
}
