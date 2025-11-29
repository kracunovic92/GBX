#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! `algebra_field`: concrete fields like `Zp` (prime fields) built on
//! the algebraic traits in `algebra_core`.

pub mod zp;

/// Prime field `GF(P)` with a **type-level** modulus `P`.
///
/// See [`zp::Zp`] for details and examples.
pub use zp::Zp;

