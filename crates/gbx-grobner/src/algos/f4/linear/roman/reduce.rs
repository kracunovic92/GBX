//! F4 adapter for Roman/Pearce sparse-buffer reduction.
//!
//! This module converts polynomial rows into a sparse matrix, runs sparse
//! echelon reduction, and extracts only the pivot rows that are new F4 rows.

use crate::algos::f4::error::Result;
use crate::f4_info;
use crate::instrumentation::profile::{count, counters, with_profile_phase};
use crate::linear::roman::build::build_sparse_matrix;
use crate::linear::roman::extract::extract_new_rows_from_sparse_pivots;
use crate::linear::roman::parallel::sparse_echelon_parallel;
use crate::linear::roman::sequential::sparse_echelon_sequential;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Reduces an F4 batch using sequential sparse-buffer echelon reduction.
///
/// # Errors
///
/// Returns an error if sparse echelon reduction fails or extracted pivots cannot
/// be converted into polynomials.
pub fn roman_sparse_buffer_reduce<P, F, O>(ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let mut build_counters = counters();
    build_counters.insert("rows_in", count(rows.len()));
    build_counters.insert(
        "terms_in",
        count(rows.iter().map(PolynomialView::len).sum()),
    );

    let sparse = with_profile_phase("f4.roman.build_matrix", build_counters, || {
        build_sparse_matrix(rows, &ctx.order)
    });

    f4_info!(
        rows = sparse.nrows(),
        cols = sparse.ncols(),
        nnz = sparse.nnz(),
        density = sparse.density(),
        "roman.matrix.built"
    );

    let (columns, sparse_rows, input_lead_cols) = sparse.into_parts();

    let mut reduce_counters = counters();
    reduce_counters.insert("matrix_rows", count(sparse_rows.len()));
    reduce_counters.insert("matrix_cols", count(columns.len()));
    reduce_counters.insert(
        "matrix_nnz",
        count(sparse_rows.iter().map(|row| row.nnz()).sum()),
    );

    let pivots = with_profile_phase("f4.roman.sparse_echelon", reduce_counters, || {
        sparse_echelon_sequential(&ctx.field, &sparse_rows, columns.len())
    })?;

    let mut extract_counters = counters();
    extract_counters.insert("pivots", count(pivots.len()));
    extract_counters.insert("columns", count(columns.len()));

    with_profile_phase("f4.roman.extract", extract_counters, || {
        extract_new_rows_from_sparse_pivots::<P, F, O>(ctx, &pivots, &columns, &input_lead_cols)
    })
}

/// Reduces an F4 batch using parallel sparse-buffer echelon reduction.
///
/// # Errors
///
/// Returns an error if sparse echelon reduction fails or extracted pivots cannot
/// be converted into polynomials.
pub fn roman_sparse_buffer_reduce_parallel<P, F, O>(ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialView + Clone + Send + Sync,
    P::Coeff: Copy + Eq + Default + Send + Sync,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let mut build_counters = counters();
    build_counters.insert("rows_in", count(rows.len()));
    build_counters.insert(
        "terms_in",
        count(rows.iter().map(PolynomialView::len).sum()),
    );

    let sparse = with_profile_phase("f4.roman_parallel.build_matrix", build_counters, || {
        build_sparse_matrix(rows, &ctx.order)
    });

    f4_info!(
        rows = sparse.nrows(),
        cols = sparse.ncols(),
        nnz = sparse.nnz(),
        density = sparse.density(),
        "roman.matrix.built"
    );

    let (columns, sparse_rows, input_lead_cols) = sparse.into_parts();

    let mut reduce_counters = counters();
    reduce_counters.insert("matrix_rows", count(sparse_rows.len()));
    reduce_counters.insert("matrix_cols", count(columns.len()));
    reduce_counters.insert(
        "matrix_nnz",
        count(sparse_rows.iter().map(|row| row.nnz()).sum()),
    );

    let pivots = with_profile_phase("f4.roman_parallel.sparse_echelon", reduce_counters, || {
        sparse_echelon_parallel(&ctx.field, &sparse_rows, columns.len())
    })?;

    let mut extract_counters = counters();
    extract_counters.insert("pivots", count(pivots.len()));
    extract_counters.insert("columns", count(columns.len()));

    with_profile_phase("f4.roman_parallel.extract", extract_counters, || {
        extract_new_rows_from_sparse_pivots::<P, F, O>(ctx, &pivots, &columns, &input_lead_cols)
    })
}
