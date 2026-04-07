use crate::algos::f4::error::{F4Error, Result};
use gbx_poly::ring::FieldCtx;

/// Return the first nonzero column in a dense row.
pub fn first_nonzero_col<C>(row: &[C]) -> Option<usize>
where
    C: Copy + Eq + Default,
{
    let zero = C::default();
    row.iter().position(|c| *c != zero)
}

/// In-place forward row echelon reduction.
///
/// Rows are normalized to monic pivots.
/// This performs only forward elimination, not full reduced row echelon form.
pub fn row_echelon_dense<F, C>(field: &F, rows: &mut [Vec<C>]) -> Result<Vec<usize>>
where
    F: FieldCtx<Elem = C>,
    C: Copy + Eq + Default,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let nrows = rows.len();
    let ncols = rows[0].len();
    let zero = C::default();

    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..ncols {
        if pivot_row >= nrows {
            break;
        }

        let mut found = None;
        for r in pivot_row..nrows {
            if rows[r][col] != zero {
                found = Some(r);
                break;
            }
        }

        let Some(found_row) = found else {
            continue;
        };

        if found_row != pivot_row {
            rows.swap(found_row, pivot_row);
        }

        let pivot = rows[pivot_row][col];
        let Some(inv) = field.try_inv(pivot) else {
            return Err(F4Error::NonInvertibleLeadingCoefficient);
        };

        // Normalize pivot row.
        for j in col..ncols {
            rows[pivot_row][j] = field.mul(rows[pivot_row][j], inv);
        }

        // Eliminate rows below.
        for r in (pivot_row + 1)..nrows {
            let factor = rows[r][col];
            if factor == zero {
                continue;
            }

            for j in col..ncols {
                let sub = field.mul(factor, rows[pivot_row][j]);
                rows[r][j] = field.sub(rows[r][j], sub);
            }
        }

        pivot_cols.push(col);
        pivot_row += 1;
    }

    Ok(pivot_cols)
}
