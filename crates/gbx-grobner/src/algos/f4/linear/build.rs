use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::algos::f4::linear::types::{DenseMatrix, F4Matrix, MatrixRowMeta};
use crate::algos::f4::symbolic::ordered::OrderedMono;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::FieldCtx;
use gbx_poly::term::{TermOwned, TermView};

/// Collect all monomials appearing in the symbolic rows and sort them
/// in descending monomial order, so the first nonzero column in a row
/// corresponds to its leading term.
pub fn collect_columns<P, O>(rows: &[P], order: &O) -> Vec<<P::Term as TermView>::Mono>
where
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq + Default,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Hash,
{
    let mut seen = HashSet::new();

    for row in rows {
        for term in row.terms() {
            seen.insert(term.mono().clone());
        }
    }

    let mut cols: Vec<_> = seen.into_iter().collect();
    cols.sort_by(|a, b| OrderedMono::new(b.clone(), order).cmp(&OrderedMono::new(a.clone(), order)));
    cols
}

/// Build monomial -> column index map.
pub fn make_col_index<M>(columns: &[M]) -> HashMap<M, usize>
where
    M: Clone + Eq + Hash,
{
    columns
        .iter()
        .cloned()
        .enumerate()
        .map(|(j, m)| (m, j))
        .collect()
}

/// Encode one polynomial row into a dense coefficient vector.
pub fn encode_dense_row<P>(row: &P, col_index: &HashMap<<P::Term as TermView>::Mono, usize>, ncols: usize) -> Vec<<P::Term as TermView>::Coeff>
where
    P: PolynomialMut + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq + Default,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Hash,
{
    let mut out = vec![<P::Term as TermView>::Coeff::default(); ncols];

    for term in row.terms() {
        if let Some(&j) = col_index.get(term.mono()) {
            out[j] = *term.coeff();
        }
    }

    out
}

/// Build a full dense F4 matrix from symbolic rows.
pub fn build_dense_matrix<P, O>(rows: &[P], order: &O) -> F4Matrix<P, <P::Term as TermView>::Mono, <P::Term as TermView>::Coeff>
where
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq + Default,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Hash,
{
    let columns = collect_columns(rows, order);
    let col_index = make_col_index(&columns);
    let ncols = columns.len();

    let mut dense_rows = Vec::with_capacity(rows.len());
    let mut metadata = Vec::with_capacity(rows.len());

    for (i, row) in rows.iter().enumerate() {
        dense_rows.push(encode_dense_row(row, &col_index, ncols));
        metadata.push(MatrixRowMeta::new(row.leading_mono().cloned(), i));
    }

    F4Matrix::new(DenseMatrix::new(dense_rows, ncols), columns, metadata)
}
