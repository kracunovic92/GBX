#![allow(missing_docs)]
use gbx_alg::{CheckedDiv, Field, One, Zero};
use gbx_field::fp::Fp;

/// Solves A x = b for x using Gaussian elimination.
/// Returns None if the matrix is singular (no unique solution).
fn solve_linear_system<F>(mut a: Vec<Vec<F>>, mut b: Vec<F>) -> Option<Vec<F>>
where
    F: Field + Copy + core::fmt::Debug,
{
    let n = a.len();
    if n == 0 || b.len() != n {
        return None;
    }
    for row in &a {
        if row.len() != n {
            return None;
        }
    }

    // Forward elimination
    for col in 0..n {
        // Find pivot row with a nonzero entry in this column
        let pivot = (col..n).find(|&r| !a[r][col].is_zero())?;

        if pivot != col {
            a.swap(pivot, col);
            b.swap(pivot, col);
        }

        // Normalize pivot row (make pivot == 1)
        let pivot_val = a[col][col];
        let inv_pivot = F::one().checked_div(pivot_val).ok()?; // 1 / pivot

        for j in col..n {
            a[col][j] = a[col][j] * inv_pivot;
        }
        b[col] = b[col] * inv_pivot;

        // Eliminate rows below
        for r in (col + 1)..n {
            let factor = a[r][col];
            if factor.is_zero() {
                continue;
            }
            for j in col..n {
                a[r][j] = a[r][j] + (-(factor * a[col][j]));
            }
            b[r] = b[r] + (-(factor * b[col]));
        }
    }

    // Back substitution
    let mut x = vec![F::zero(); n];

    for i in (0..n).rev() {
        let mut rhs = b[i];
        for j in (i + 1)..n {
            rhs = rhs + (-(a[i][j] * x[j]));
        }
        // Here a[i][i] should be 1 due to normalization
        x[i] = rhs;
    }

    Some(x)
}

fn main() {
    type F = Fp<7>;

    let a = vec![vec![F::new(2), F::new(3)], vec![F::new(4), F::new(1)]];
    let b = vec![F::new(1), F::new(6)];

    let x = solve_linear_system::<F>(a, b).expect("should have a unique solution");
    println!("solution: x={}, y={}", x[0].value(), x[1].value());
}
