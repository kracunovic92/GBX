//! Build dense F4 matrices from symbolic preprocessing output.

use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::matrix::types::{ColumnBasis, DenseMatrixData, MatrixRowMeta};
use crate::algos::f4::symbolic::SymbolicPreprocessing;

use gbx_poly::monomial::{Monomial, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::TermView;

/// Build a dense matrix representation from symbolic F4 rows.
///
/// This version:
/// - collects the union of monomials appearing in all symbolic rows,
/// - sorts columns by the ring monomial order,
/// - encodes each polynomial as a dense coefficient row,
/// - preserves per-row symbolic metadata.
pub fn build_dense_matrix<F, O, P>(ctx: &RingCtx<F, O>, symbolic: &SymbolicPreprocessing<P, <P::Term as TermView>::Mono>) -> Result<DenseMatrixData<F::Elem, <P::Term as TermView>::Mono>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialView + Clone,
    P::Term: TermView,
    <P::Term as TermView>::Coeff: Clone,
    <P::Term as TermView>::Mono: Monomial + MonomialView<Word = u32> + Clone + PartialEq,
{
    let columns = collect_columns(ctx, symbolic);

    let mut rows = Vec::with_capacity(symbolic.all_rows.len());
    let mut row_meta = Vec::with_capacity(symbolic.all_rows.len());

    for row in &symbolic.all_rows {
        rows.push(encode_row(ctx, &row.poly, &columns)?);
        row_meta.push(MatrixRowMeta { source_basis_index: row.source_basis_index, multiplier: row.multiplier.clone(), kind: row.kind });
    }

    Ok(DenseMatrixData { rows, columns, row_meta })
}

fn collect_columns<F, O, P>(ctx: &RingCtx<F, O>, symbolic: &SymbolicPreprocessing<P, <P::Term as TermView>::Mono>) -> ColumnBasis<<P::Term as TermView>::Mono>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialView<Word = u32> + Clone + PartialEq,
{
    let mut monomials = Vec::new();

    for row in &symbolic.all_rows {
        for term in row.poly.terms() {
            let mono = term.mono().clone();

            if !monomials.iter().any(|m| m == &mono) {
                monomials.push(mono);
            }
        }
    }

    // Earlier columns correspond to larger monomials.
    monomials.sort_by(|a, b| ctx.order.cmp(b, a));

    ColumnBasis { monomials }
}

fn encode_row<F, O, P>(ctx: &RingCtx<F, O>, poly: &P, columns: &ColumnBasis<<P::Term as TermView>::Mono>) -> Result<Vec<F::Elem>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Coeff: Clone,
    <P::Term as TermView>::Mono: Monomial + Clone + PartialEq,
{
    let mut row = vec![ctx.field.zero(); columns.monomials.len()];

    for term in poly.terms() {
        let col = columns
            .monomials
            .iter()
            .position(|m| m == term.mono())
            .ok_or(F4Error::MatrixBuildInvariant)?;

        row[col] = term.coeff().clone();
    }

    Ok(row)
}
