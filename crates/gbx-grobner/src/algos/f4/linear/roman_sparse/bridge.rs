//! Bridge from F4 batch reducer interface to Roman sparse-buffer reduction.
//!
//! This module should stay small:
//!
//! ```text
//! polynomial rows
//!     -> sparse matrix
//!     -> sparse echelon pivots
//!     -> extract new F4 rows
//! ```
//!
//! It must not call `build_dense_matrix` or perform dense F4 extraction.

use crate::algos::f4::error::Result;
use crate::linear::roman_sparse::build::build_sparse_matrix;
use crate::linear::roman_sparse::extract::extract_new_rows_from_sparse_pivots;
use crate::linear::roman_sparse::parallel::sparse_echelon_parallel;
use crate::linear::roman_sparse::sequential::sparse_echelon_sequential;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

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

    let sparse = build_sparse_matrix(rows, &ctx.order);

    let (columns, sparse_rows, input_lead_cols) = sparse.into_parts();

    let (pivots, stats) = sparse_echelon_sequential(&ctx.field, &sparse_rows, columns.len())?;

    drop(sparse_rows);

    extract_new_rows_from_sparse_pivots::<P, F, O>(ctx, &pivots, &columns, &input_lead_cols)
}

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

    let sparse = build_sparse_matrix(rows, &ctx.order);

    let (columns, sparse_rows, input_lead_cols) = sparse.into_parts();

    let (pivots, stats) = sparse_echelon_parallel(&ctx.field, &sparse_rows, columns.len())?;

    drop(sparse_rows);

    extract_new_rows_from_sparse_pivots::<P, F, O>(ctx, &pivots, &columns, &input_lead_cols)
}
