use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::preprocess::symbolic_preprocess;
use crate::extract::rows::extract_new_rows;
use crate::symbolic::UnevaluatedProduct;

use crate::algos::f4::simplify::SimplifyIndex;
use crate::linear::BatchReducer;
use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Output of one F4 reduction phase.
///
/// This corresponds to the paper's:
///
/// - `F_d`: rows produced by symbolic preprocessing,
/// - `F̃_d`: row-echelon reduction of `F_d`,
/// - `F̃_d⁺`: reduced rows whose leading terms are new.
pub struct ReductionPhase<P>
where
    P: PolynomialView,
{
    /// Products used to create F_d rows.
    pub f_d_products: Vec<UnevaluatedProduct<Monomial>>,

    /// Heads HT(F_d), used for extraction/history.
    pub f_d_heads: Vec<Monomial>,

    /// Row-echelon reduction of F_d: F̃_d.
    pub f_d_tilde: Vec<P>,

    /// Extracted rows with new heads: F̃_d⁺.
    pub extracted_rows: Vec<P>,
}

/// Runs the F4 reduction phase for one selected pair batch.
///
/// Input:
/// - `l_d`: initial unevaluated products `L_d = Left(P_d) ∪ Right(P_d)`.
/// - `basis`: current basis `G`.
/// - `history`: previous symbolic/reduced batches `F_1, ..., F_{d-1}`.
/// - `reducer`: matrix reducer used to compute row-echelon form.
///
/// Output:
/// - `f_d`: symbolic preprocessing output `F_d`.
/// - `f_d_tilde`: row-echelon reduction `F̃_d`.
/// - `extracted_rows`: new rows `F̃_d⁺` to insert into the basis.
pub fn reduction_phase<P, F, O, R>(
    ctx: &RingCtx<F, O>,
    opts: &F4Options,
    l_d: &[UnevaluatedProduct<Monomial>],
    basis: &[P],
    history: &[BatchHistory<P>],
    reducer: &R,
    simplify_index: &SimplifyIndex,
) -> Result<ReductionPhase<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send,
    P::Coeff: Copy + Eq + Default,
    R: BatchReducer<P, F, O>,
{
    let f_d = symbolic_preprocess(ctx, l_d, basis, history, simplify_index)?;

    let (f_d_rows, f_d_products, f_d_heads) = f_d.into_rows_parts();

    let f_d_tilde = reducer.reduce(ctx, &f_d_rows)?;

    let extracted_rows = extract_new_rows(&f_d_rows, &f_d_tilde, &ctx.order)?;

    let extracted_rows = normalize_extracted_rows(ctx, opts, extracted_rows)?;

    Ok(ReductionPhase { f_d_products, f_d_heads, f_d_tilde, extracted_rows })
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
