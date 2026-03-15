//! S-polynomial construction.
//!
//! This module provides context-driven S-polynomial computation.
//!
//! Public entry points:
//! - [`s_polynomial_in`] - compute the S-polynomial of two polynomials
//!   inside a ring context.
//!
//! Design notes:
//! - arithmetic is performed through the ring/field context,
//! - ring tags are validated before algebraic work begins,
//! - the returned polynomial is not required to be normalized;
//!   callers may normalize later if canonical form is needed.

mod compute;
mod error;

pub use compute::s_polynomial_in;
pub use error::SPolyError;

#[cfg(test)]
mod tests;
