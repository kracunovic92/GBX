//! Linear algebra backends used by the F4 algorithm.
//!
//! The F4 engine delegates matrix reduction to implementations of
//! [`BatchReducer`]. Concrete backends live in submodules.

pub mod dense;
pub mod reducer;
pub mod roman;

pub use dense::{DenseF4MatrixReducer, dense_matrix_reduce};
pub use reducer::BatchReducer;
pub use roman::*;
