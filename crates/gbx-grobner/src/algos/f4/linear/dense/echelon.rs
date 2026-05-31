//! Dense row-echelon reduction.

use crate::algos::f4::error::{F4Error, Result};

use gbx_poly::ring::FieldCtx;

/// Returns the first nonzero column in a dense row.
#[inline]
#[must_use]
pub fn first_nonzero_col<C>(row: &[C]) -> Option<usize>
where
    C: Copy + Eq + Default,
{
    let zero = C::default();

    row.iter().position(|coeff| *coeff != zero)
}

/// Performs in-place forward row-echelon reduction.
///
/// Pivot rows are normalized to leading coefficient one. This is forward
/// elimination only, not reduced row-echelon form.
///
/// # Errors
///
/// Returns [`F4Error::NonInvertibleLeadingCoefficient`] when a pivot cannot be
/// inverted in the coefficient field.
pub fn row_echelon_dense<F, C>(field: &F, rows: &mut [Vec<C>]) -> Result<Vec<usize>>
where
    F: FieldCtx<Elem = C>,
    C: Copy + Eq + Default,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let ncols = rows[0].len();
    let zero = C::default();

    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..ncols {
        if pivot_row >= rows.len() {
            break;
        }

        let Some(found_row) = find_pivot_row(rows, pivot_row, col, zero) else {
            continue;
        };

        if found_row != pivot_row {
            rows.swap(found_row, pivot_row);
        }

        normalize_pivot_row(field, &mut rows[pivot_row], col)?;

        eliminate_below(field, rows, pivot_row, col, zero);

        pivot_cols.push(col);
        pivot_row += 1;
    }

    Ok(pivot_cols)
}

fn find_pivot_row<C>(rows: &[Vec<C>], start_row: usize, col: usize, zero: C) -> Option<usize>
where
    C: Copy + Eq,
{
    (start_row..rows.len()).find(|&row| rows[row][col] != zero)
}

fn normalize_pivot_row<F, C>(field: &F, row: &mut [C], pivot_col: usize) -> Result<()>
where
    F: FieldCtx<Elem = C>,
    C: Copy + Eq + Default,
{
    let pivot = row[pivot_col];

    let Some(inv) = field.try_inv(pivot) else {
        return Err(F4Error::NonInvertibleLeadingCoefficient);
    };

    for coeff in &mut row[pivot_col..] {
        *coeff = field.mul(*coeff, inv);
    }

    Ok(())
}

fn eliminate_below<F, C>(field: &F, rows: &mut [Vec<C>], pivot_row: usize, pivot_col: usize, zero: C)
where
    F: FieldCtx<Elem = C>,
    C: Copy + Eq,
{
    let pivot_tail: Vec<C> = rows[pivot_row][pivot_col..].to_vec();

    for row in rows.iter_mut().skip(pivot_row + 1) {
        let factor = row[pivot_col];

        if factor == zero {
            continue;
        }

        for (offset, &pivot_coeff) in pivot_tail.iter().enumerate() {
            let col = pivot_col + offset;
            let sub = field.mul(factor, pivot_coeff);

            row[col] = field.sub(row[col], sub);
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use gbx_field::fp::Fp;
    fn field() -> Fp {
        Fp::prime(7).unwrap()
    }

    #[test]
    fn first_nonzero_col_finds_first_nonzero_entry() {
        assert_eq!(first_nonzero_col(&[0, 0, 3, 0]), Some(2));
    }

    #[test]
    fn first_nonzero_col_returns_none_for_zero_row() {
        assert_eq!(first_nonzero_col(&[0, 0, 0]), None);
    }

    #[test]
    fn row_echelon_returns_empty_for_empty_matrix() {
        let field = field();
        let mut rows: Vec<Vec<_>> = Vec::new();

        let pivots = row_echelon_dense(&field, &mut rows).unwrap();

        assert!(pivots.is_empty());
        assert!(rows.is_empty());
    }

    #[test]
    fn row_echelon_normalizes_pivot_rows() {
        let field = field();

        let mut rows = vec![vec![field.elem(2), field.elem(4)], vec![field.elem(0), field.elem(3)]];

        let pivots = row_echelon_dense(&field, &mut rows).unwrap();

        assert_eq!(pivots, vec![0, 1]);
        assert_eq!(rows[0][0], field.elem(1));
        assert_eq!(rows[1][1], field.elem(1));
    }

    #[test]
    fn row_echelon_eliminates_below_pivots() {
        let field = field();

        let mut rows = vec![vec![field.elem(1), field.elem(2), field.elem(0)], vec![field.elem(3), field.elem(4), field.elem(1)]];

        let pivots = row_echelon_dense(&field, &mut rows).unwrap();

        assert_eq!(pivots, vec![0, 1]);
        assert_eq!(rows[1][0], field.elem(0));
    }

    #[test]
    fn row_echelon_swaps_pivot_row_when_needed() {
        let field = field();

        let mut rows = vec![vec![field.elem(0), field.elem(1)], vec![field.elem(3), field.elem(4)]];

        let pivots = row_echelon_dense(&field, &mut rows).unwrap();

        assert_eq!(pivots, vec![0, 1]);
        assert_eq!(rows[0][0], field.elem(1));
    }

    #[test]
    fn row_echelon_reduces_dependent_rows_to_zero_tail() {
        let field = field();

        let mut rows = vec![vec![field.elem(1), field.elem(2)], vec![field.elem(2), field.elem(4)]];

        let pivots = row_echelon_dense(&field, &mut rows).unwrap();

        assert_eq!(pivots, vec![0]);
        assert_eq!(rows[1], vec![field.elem(0), field.elem(0)]);
    }
}
