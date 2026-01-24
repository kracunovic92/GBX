//! Prelude for `gbx-poly`.
//!
//! Import this to get the most commonly-used polynomial building blocks.
//!
//! ```rust
//! use gbx_poly::prelude::*;
//! ```
//!
//! The prelude is intentionally small and stable: it exports core traits and
//! the main monomial/order types. More specialized items should be imported
//! from their modules directly.

pub use crate::monomial::{DynamicMonomial, FixedMonomial, Grevlex, Lex, Monomial, MonomialError, MonomialOrder};

// If/when you add terms/polynomials later, you’ll extend this carefully:
// pub use crate::term::{Term, TermLike, TermError};
// pub use crate::polynomial::{Polynomial, PolynomialLike, PolynomialError};
