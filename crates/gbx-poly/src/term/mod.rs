//! Terms `c * x^α` used in polynomial rings.
//!
//! A term is a coefficient multiplied by a monomial.
//!
//! In GBX, the monomial representation is concrete and runtime-sized:
//! [`crate::monomial::Monomial`].
//!
//! The coefficient type remains generic because it is determined by the field
//! context stored in [`crate::ring::RingCtx`].
//!
//! # Example
//!
//! ```
//! use gbx_poly::monomial::MonomialView;
//! use gbx_poly::term::Term;
//!
//! let t = Term::new(7u32, [1, 0, 2].into());
//!
//! assert_eq!(*t.coeff(), 7);
//! assert_eq!(t.mono().exponents(), &[1, 0, 2]);
//! ```

pub mod error;
pub mod ops;
mod repr;
pub mod view;

mod display;
mod macros;

pub use display::{TermDisplay, TermStyle};
pub use error::{TermError, TermResult};
pub use ops::*;
pub use repr::Term;
pub use view::TermView;
