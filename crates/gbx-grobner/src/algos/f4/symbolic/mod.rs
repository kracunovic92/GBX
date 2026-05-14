//! Symbolic preprocessing for F4 batches.
//!
//! Symbolic preprocessing builds the row set `F_d` for a selected batch of
//! critical pairs. Products are kept unevaluated until simplification has had
//! a chance to rewrite them using previous batch history.

mod materialize;
pub mod ordered;
pub mod preprocess;
pub mod reducers;
mod types;
mod worklist;

pub use preprocess::symbolic_preprocess;
pub use types::{PolyProduct, ProductSource, SymbolicPreprocessOutput, SymbolicRow, UnevaluatedProduct};
