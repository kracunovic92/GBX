use crate::algos::f4::error::Result;
use crate::algos::f4::linear::reducer::BatchReducer;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::preprocess::symbolic_preprocess;
use crate::extract::rows::extract_new_rows;
use crate::instrumentation::alloc::with_alloc_profile;
use crate::linear::DenseF4MatrixReducer;
use crate::symbolic::{SymbolicPreprocessOutput, SymbolicProduct};

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

pub struct ReductionPhase<P>
where
    P: PolynomialView,
{
    /// Full symbolic preprocessing output for F_d.
    pub f_d: SymbolicPreprocessOutput<P, Monomial>,

    /// Row-echelon reduction of F_d: \tilde F_d.
    pub f_d_tilde: Vec<P>,

    /// Extracted rows with new heads: \tilde F_d^+.
    pub extracted_rows: Vec<P>,
}

pub fn reduction_phase<P, F, O>(
    ctx: &RingCtx<F, O>,
    opts: &F4Options,
    l_d: &[SymbolicProduct<Monomial>],
    basis: &[P],
    history: &[BatchHistory<P>],
    reducer: &DenseF4MatrixReducer,
) -> Result<ReductionPhase<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
{
    let f_d = with_alloc_profile("f4.symbolic_preprocess", || {
        symbolic_preprocess(ctx, l_d, basis, history)
    })?;

    let f_d_rows: Vec<P> = with_alloc_profile("f4.clone_symbolic_rows", || {
        f_d.rows.iter().map(|row| row.polynomial.clone()).collect()
    });

    let f_d_tilde = with_alloc_profile("f4.matrix_reduce", || reducer.reduce(ctx, &f_d_rows))?;

    let extracted_rows = with_alloc_profile("f4.extract_new_rows", || {
        extract_new_rows(&f_d_rows, &f_d_tilde, &ctx.order)
    })?;

    let extracted_rows = with_alloc_profile("f4.normalize_extracted_rows", || {
        normalize_extracted_rows(ctx, opts, extracted_rows)
    })?;

    Ok(ReductionPhase { f_d, f_d_tilde, extracted_rows })
}

fn normalize_extracted_rows<P, F, O>(ctx: &RingCtx<F, O>, opts: &F4Options, mut rows: Vec<P>) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
{
    if !opts.normalize_extracted {
        return Ok(rows);
    }

    normalize_extracted_rows_parallel(ctx, &mut rows)?;

    Ok(rows)
}

fn normalize_extracted_rows_parallel<P, F, O>(ctx: &RingCtx<F, O>, rows: &mut [P]) -> Result<()>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
{
    rows.iter_mut().try_for_each(|row| {
        if !row.is_zero() {
            row.normalize_in_place(ctx)?;
        }

        Ok(())
    })
}
