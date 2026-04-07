use crate::algos::f4::error::Result;
use crate::algos::f4::linear::build::build_dense_matrix;
use crate::algos::f4::linear::dense::row_echelon_dense;
use crate::algos::f4::linear::extract::extract_new_rows_from_dense;
use crate::algos::f4::linear::reducer::BatchReducer;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};
use std::hash::Hash;
/// Dense matrix-based F4 reducer.
///
/// This replaces polynomial hot-loop row reduction with coefficient-matrix
/// elimination over the field.
#[derive(Debug, Default, Clone, Copy)]
pub struct DenseF4MatrixReducer;

impl<P, F, O> BatchReducer<P, F, O> for DenseF4MatrixReducer
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq + Default,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Hash,
{
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>> {
        dense_matrix_reduce(ctx, rows)
    }
}

pub fn dense_matrix_reduce<P, F, O>(ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq + Default,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Hash,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let mut matrix = build_dense_matrix(rows, &ctx.order);

    row_echelon_dense(&ctx.field, &mut matrix.matrix.rows)?;

    extract_new_rows_from_dense::<P, F, O>(ctx, rows, &matrix.matrix.rows, &matrix.columns)
}
