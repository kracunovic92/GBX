//! Dense matrix backend for F4 reduction.
//!
//! This backend constructs a full dense coefficient matrix, performs forward
//! row-echelon reduction, and extracts reduced rows whose leading monomials are
//! new relative to the symbolic input rows.

pub mod build;
pub mod echelon;
pub mod extract;
pub mod reducer;
pub mod types;

pub use reducer::{dense_matrix_reduce, DenseF4MatrixReducer};
pub use types::{DenseMatrix, F4Matrix, MatrixRowMeta};
