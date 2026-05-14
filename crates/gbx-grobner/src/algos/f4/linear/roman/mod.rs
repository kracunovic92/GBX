//! Roman/Pearce-style sparse-buffer reduction for F4 matrices.
//!
//! This reducer keeps F4 rows sparse, reduces one active row through a dense
//! temporary buffer, and stores normalized sparse pivot rows. The public reducer
//! types in this module implement [`BatchReducer`] and can be selected as F4
//! matrix-reduction backends.
//!
//! The pivot table is internal reduction state. F4 extraction returns only
//! reduced rows whose leading column was not already the leading column of an
//! input row.

pub mod buffer;
pub mod build;
pub mod extract;
pub mod parallel;
pub mod reduce;
pub mod row;
pub mod sequential;

use crate::algos::f4::error::Result;
use crate::linear::roman::reduce::{roman_sparse_buffer_reduce, roman_sparse_buffer_reduce_parallel};
use crate::linear::BatchReducer;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Sequential Roman/Pearce-style sparse-buffer reducer.
#[derive(Debug, Default, Clone, Copy)]
pub struct RomanSparseBufferReducer;

/// Parallel Roman/Pearce-style sparse-buffer reducer.
#[derive(Debug, Default, Clone, Copy)]
pub struct RomanParallelSparseBufferReducer;

impl<P, F, O> BatchReducer<P, F, O> for RomanSparseBufferReducer
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>> {
        roman_sparse_buffer_reduce(ctx, rows)
    }
}

impl<P, F, O> BatchReducer<P, F, O> for RomanParallelSparseBufferReducer
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialView + Clone + Send + Sync,
    P::Coeff: Copy + Eq + Default + Send + Sync,
{
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>> {
        roman_sparse_buffer_reduce_parallel(ctx, rows)
    }
}
