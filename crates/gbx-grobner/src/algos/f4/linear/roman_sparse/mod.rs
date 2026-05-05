pub mod bridge;
pub mod buffer;
pub mod parallel;
pub mod row;
pub mod sequential;

use crate::algos::f4::error::Result;
use crate::algos::f4::linear::reducer::BatchReducer;

use crate::linear::roman_sparse::bridge::{roman_sparse_buffer_reduce, roman_sparse_buffer_reduce_parallel};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

#[derive(Debug, Default, Clone, Copy)]
pub struct RomanSparseBufferReducer;

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
