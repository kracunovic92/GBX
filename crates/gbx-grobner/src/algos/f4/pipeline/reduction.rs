use crate::algos::f4::error::Result;
use crate::algos::f4::linear::reducer::BatchReducer;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::preprocess::symbolic_preprocess;
use crate::algos::f4::types::PolyMono;
use crate::extract::rows::extract_new_rows;
use crate::instrumentation::{f4_debug, f4_span};
use crate::symbolic::{SymbolicPreprocessOutput, SymbolicProduct};
use std::hash::Hash;

use crate::linear::DenseF4MatrixReducer;
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

pub struct ReductionPhase<P>
where
    P: PolynomialView,
    P::Term: TermView,
{
    /// Full symbolic preprocessing output for F_d.
    pub f_d: SymbolicPreprocessOutput<P, PolyMono<P>>,

    /// Row-echelon reduction of F_d: \tilde F_d
    pub f_d_tilde: Vec<P>,

    /// Extracted rows with new heads: \tilde F_d^+
    pub extracted_rows: Vec<P>,
}

#[cfg_attr(feature = "instrumentation", tracing::instrument(level = "debug", skip(ctx, opts, l_d, basis, history, reducer)))]
pub fn reduction_phase<P, F, O>(
    ctx: &RingCtx<F, O>,
    opts: &F4Options,
    l_d: &[SymbolicProduct<<P::Term as TermView>::Mono>],
    basis: &[P],
    history: &[BatchHistory<P, PolyMono<P>>],
    reducer: &DenseF4MatrixReducer,
) -> Result<ReductionPhase<P>>
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
    let f_d = {
        f4_span!("symbolic_preprocess");
        symbolic_preprocess(ctx, l_d, basis, history)?
    };

    f4_debug!(rows = f_d.rows.len(), "symbolic preprocessing complete");

    let f_d_rows: Vec<P> = f_d.rows.iter().map(|row| row.polynomial.clone()).collect();

    let f_d_tilde = {
        f4_span!("row_echelon_reduce");
        reducer.reduce(ctx, &f_d_rows)?
    };

    f4_debug!(rows = f_d_tilde.len(), "row echelon reduction complete");

    let extracted_rows = {
        f4_span!("extract_new_rows");
        extract_new_rows(&f_d_rows, &f_d_tilde, &ctx.order)?
    };

    f4_debug!(rows = extracted_rows.len(), "extracted new-head rows");

    let extracted_rows = {
        f4_span!("normalize_extracted_rows");
        normalize_extracted_rows(ctx, opts, extracted_rows)?
    };

    f4_debug!(rows = extracted_rows.len(), "normalized extracted rows");

    Ok(ReductionPhase { f_d, f_d_tilde, extracted_rows })
}

fn normalize_extracted_rows<P, F, O>(ctx: &RingCtx<F, O>, opts: &F4Options, mut rows: Vec<P>) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Default,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    if !opts.normalize_extracted {
        return Ok(rows);
    }

    for row in &mut rows {
        if !row.is_zero() {
            row.normalize_in_place(ctx)?;
        }
    }

    Ok(rows)
}
