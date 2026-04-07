use std::hash::Hash;
use std::time::Instant;

use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::options::F4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::selector::MinDegreeSelector;
use crate::algos::f4::pipeline::{initialize_state, post_process_basis, run_iteration};
use crate::basis::GrobnerBasis;
use crate::instrumentation::f4_info;

use crate::linear::DenseF4MatrixReducer;
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

pub fn run_f4<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: F4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Default,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
    <<P as PolynomialView>::Term as TermView>::Mono: Hash,
{
    if opts.batch_size == 0 {
        return Err(F4Error::InvalidBatchSize);
    }

    let run_started = Instant::now();

    let criterion = ProductCriterion;
    let mut selector = MinDegreeSelector::new(opts.batch_size);
    let reducer = DenseF4MatrixReducer;

    let init_started = Instant::now();
    let mut state = initialize_state(ctx, fs, &opts, &criterion)?;
    let init_dt = init_started.elapsed();

    f4_info!(
        init_ms = init_dt.as_secs_f64() * 1000.0,
        basis_len = state.basis.len(),
        pending_len = state.pending.len(),
        batch_size = opts.batch_size,
        "initialized state"
    );

    if state.basis.is_empty() {
        let gb = GrobnerBasis::empty_in(ctx);

        f4_info!(
            elapsed_ms = run_started.elapsed().as_secs_f64() * 1000.0,
            batch_size = opts.batch_size,
            "finished F4 with empty basis"
        );

        return Ok(gb);
    }

    while !state.is_done() {
        run_iteration(ctx, &mut state, &opts, &mut selector, &reducer, &criterion)?;
    }

    let iterations = state.iteration;

    let post_started = Instant::now();
    let gb = post_process_basis(ctx, state.basis, &opts)?;
    let post_dt = post_started.elapsed();

    let total_dt = run_started.elapsed();

    f4_info!(
        elapsed_ms = total_dt.as_secs_f64() * 1000.0,
        post_ms = post_dt.as_secs_f64() * 1000.0,
        post_pct = if total_dt.is_zero() { 0.0 } else { 100.0 * post_dt.as_secs_f64() / total_dt.as_secs_f64() },
        iterations,
        basis_len = gb.len(),
        batch_size = opts.batch_size,
        "finished F4"
    );

    Ok(gb)
}
