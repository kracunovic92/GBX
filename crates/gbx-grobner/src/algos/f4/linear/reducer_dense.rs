use crate::algos::f4::error::Result;
use crate::algos::f4::linear::build::build_dense_matrix;
use crate::algos::f4::linear::dense::row_echelon_dense;
use crate::algos::f4::linear::extract::extract_new_rows_from_dense;
use crate::algos::f4::linear::reducer::BatchReducer;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Dense matrix-based F4 reducer.
#[derive(Debug, Default, Clone, Copy)]
pub struct DenseF4MatrixReducer;

impl<P, F, O> BatchReducer<P, F, O> for DenseF4MatrixReducer
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>> {
        dense_matrix_reduce(ctx, rows)
    }
}

pub fn dense_matrix_reduce<P, F, O>(ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let mut matrix = build_dense_matrix(rows, &ctx.order);

    row_echelon_dense(&ctx.field, &mut matrix.matrix.rows)?;

    extract_new_rows_from_dense::<P, F, O>(ctx, rows, &matrix.matrix.rows, &matrix.columns)
}
