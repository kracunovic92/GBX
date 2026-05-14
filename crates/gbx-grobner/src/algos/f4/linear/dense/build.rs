//! Dense F4 matrix construction.

use std::collections::{HashMap, HashSet};

use crate::linear::dense::types::{DenseMatrix, F4Matrix, MatrixRowMeta};

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;

/// Builds a full dense F4 matrix from symbolic rows.
///
/// Columns are sorted in descending monomial order, so the first nonzero
/// coefficient in each row corresponds to the row leading term.
pub fn build_dense_matrix<P, O>(rows: &[P], order: &O) -> F4Matrix<Monomial, P::Coeff>
where
    O: MonomialOrder,
    P: PolynomialView,
    P::Coeff: Copy + Eq + Default,
{
    let columns = collect_columns(rows, order);
    let col_index = make_col_index(&columns);
    let ncols = columns.len();

    let mut dense_rows = Vec::with_capacity(rows.len());
    let mut metadata = Vec::with_capacity(rows.len());

    for (source_row, row) in rows.iter().enumerate() {
        dense_rows.push(encode_dense_row(row, &col_index, ncols));
        metadata.push(MatrixRowMeta::new(row.leading_mono().cloned(), source_row));
    }

    F4Matrix::new(DenseMatrix::new(dense_rows, ncols), columns, metadata)
}

/// Collects all distinct monomials from `rows` in descending monomial order.
pub fn collect_columns<P, O>(rows: &[P], order: &O) -> Vec<Monomial>
where
    O: MonomialOrder,
    P: PolynomialView,
{
    let approx_terms = rows.iter().map(PolynomialView::len).sum();

    let mut seen = HashSet::with_capacity(approx_terms);
    let mut columns = Vec::new();

    for row in rows {
        for term in row.terms() {
            let mono = term.mono();

            if seen.insert(mono) {
                columns.push(mono.clone());
            }
        }
    }

    columns.sort_unstable_by(|a, b| order.cmp(b, a));
    columns.dedup();

    columns
}

/// Builds a monomial-to-column-index map.
pub fn make_col_index(columns: &[Monomial]) -> HashMap<&Monomial, usize> {
    columns
        .iter()
        .enumerate()
        .map(|(index, mono)| (mono, index))
        .collect()
}

/// Encodes one polynomial row into a dense coefficient vector.
pub fn encode_dense_row<P>(row: &P, col_index: &HashMap<&Monomial, usize>, ncols: usize) -> Vec<P::Coeff>
where
    P: PolynomialView,
    P::Coeff: Copy + Eq + Default,
{
    let mut dense = vec![P::Coeff::default(); ncols];

    for term in row.terms() {
        if let Some(&col) = col_index.get(term.mono()) {
            dense[col] = *term.coeff();
        }
    }

    dense
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::monomial::Monomial;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::Polynomial;
    use gbx_poly::ring::FieldCtx;

    type P = Polynomial<FpElem>;

    #[test]
    fn collect_columns_deduplicates_and_sorts_descending() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let rows = vec![
            // x^2 + y
            poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(),
            // x + 1
            poly![&ring; (1, [1, 0]), (1, [0, 0])].unwrap(),
        ];

        let columns = collect_columns(&rows, &ring.order);

        assert_eq!(
            columns,
            vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 1]), Monomial::from_slice(&[0, 0]),]
        );
    }

    #[test]
    fn make_col_index_maps_columns_to_indices() {
        let columns = vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 0])];

        let index = make_col_index(&columns);

        assert_eq!(index.get(&Monomial::from_slice(&[2, 0])), Some(&0));
        assert_eq!(index.get(&Monomial::from_slice(&[1, 0])), Some(&1));
        assert_eq!(index.get(&Monomial::from_slice(&[0, 0])), Some(&2));
    }

    #[test]
    fn encode_dense_row_places_coefficients_in_column_order() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let row: P = poly![&ring; (3, [1, 0]), (5, [0, 0])].unwrap();

        let columns = vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 0])];
        let index = make_col_index(&columns);

        let dense = encode_dense_row(&row, &index, columns.len());

        assert_eq!(
            dense,
            vec![FieldCtx::new(&ring.field, 0), FieldCtx::new(&ring.field, 3), FieldCtx::new(&ring.field, 5),]
        );
    }

    #[test]
    fn build_dense_matrix_constructs_rows_columns_and_metadata() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let rows: Vec<P> = vec![poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(), poly![&ring; (1, [1, 0]), (1, [0, 0])].unwrap()];

        let matrix = build_dense_matrix(&rows, &ring.order);

        assert_eq!(matrix.nrows(), 2);
        assert_eq!(matrix.ncols(), 4);

        assert_eq!(
            matrix.columns,
            vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 1]), Monomial::from_slice(&[0, 0]),]
        );

        assert_eq!(matrix.metadata.len(), 2);
        assert_eq!(matrix.metadata[0].source_row, 0);
        assert_eq!(matrix.metadata[1].source_row, 1);
        assert_eq!(
            matrix.metadata[0].leading_mono,
            Some(Monomial::from_slice(&[2, 0]))
        );
        assert_eq!(
            matrix.metadata[1].leading_mono,
            Some(Monomial::from_slice(&[1, 0]))
        );
    }

    #[test]
    fn empty_input_builds_empty_matrix() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let rows: Vec<P> = Vec::new();

        let matrix = build_dense_matrix(&rows, &ring.order);

        assert_eq!(matrix.nrows(), 0);
        assert_eq!(matrix.ncols(), 0);
        assert!(matrix.columns.is_empty());
        assert!(matrix.metadata.is_empty());
        assert!(matrix.is_empty());
    }
}
