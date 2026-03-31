//! Matrix representation and construction for F4.

pub mod build;
pub mod reduce;
pub mod types;

pub use build::build_dense_matrix;
pub use types::{ColumnBasis, DenseMatrixData, MatrixRowMeta};
