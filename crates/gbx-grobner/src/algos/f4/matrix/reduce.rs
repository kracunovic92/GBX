use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::matrix::DenseMatrixData;

use crate::matrix::types::ReducedMatrixData;
use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Perform dense Gaussian elimination on an F4 matrix package.
///
/// This is a correctness-first row reduction:
/// - pivots are selected by scanning columns left-to-right,
/// - pivot rows are normalized to leading coefficient `1`,
/// - elimination is performed above and below each pivot.
///
/// The column basis is preserved unchanged.
/// Row metadata is carried through row swaps, but after elimination it should
/// only be interpreted as metadata for the original row layout, not exact
/// provenance of each reduced row.
pub fn row_reduce_dense<F, O, M>(ctx: &RingCtx<F, O>, mut data: DenseMatrixData<F::Elem, M>) -> Result<ReducedMatrixData<F::Elem, M>>
where
    F: FieldCtx,
    F::Elem: Clone,
    O: MonomialOrder,
    M: Monomial,
{
    let rows = &mut data.rows;

    if rows.is_empty() {
        return Ok(ReducedMatrixData { rows: Vec::new(), columns: data.columns, row_meta: data.row_meta });
    }

    let nrows = rows.len();
    let ncols = rows[0].len();
    let mut pivot_row = 0usize;

    for pivot_col in 0..ncols {
        let pivot = (pivot_row..nrows).find(|&r| !ctx.field.is_zero(rows[r][pivot_col].clone()));
        let Some(found) = pivot else {
            continue;
        };

        rows.swap(pivot_row, found);
        data.row_meta.swap(pivot_row, found);

        let inv = ctx
            .field
            .try_inv(rows[pivot_row][pivot_col].clone())
            .ok_or(F4Error::RowReductionInvariant)?;

        for c in pivot_col..ncols {
            rows[pivot_row][c] = ctx.field.mul(rows[pivot_row][c].clone(), inv.clone());
        }

        for r in 0..nrows {
            if r == pivot_row || ctx.field.is_zero(rows[r][pivot_col].clone()) {
                continue;
            }

            let factor = rows[r][pivot_col].clone();

            for c in pivot_col..ncols {
                let sub = ctx.field.mul(factor.clone(), rows[pivot_row][c].clone());
                rows[r][c] = ctx.field.sub(rows[r][c].clone(), sub);
            }
        }

        pivot_row += 1;
        if pivot_row == nrows {
            break;
        }
    }

    Ok(ReducedMatrixData { rows: data.rows, columns: data.columns, row_meta: data.row_meta })
}
