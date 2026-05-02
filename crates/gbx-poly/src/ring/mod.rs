//! Ring context for polynomial arithmetic.
//!
//! Polynomial objects in this crate do not store the full ring information.
//! Instead, arithmetic is performed with an explicit [`RingCtx`].
//!
//! A [`RingCtx`] contains:
//! - the coefficient field context,
//! - the monomial order,
//! - the number of variables,
//! - a unique [`RingId`] used to detect accidental mixing of polynomials from
//!   different rings.
//!
//! This keeps polynomial and term structures small while still making every
//! operation explicit about which ring it belongs to.
//!
//! # Example
//!
//! ```
//! use gbx_field::fp::Fp;
//! use gbx_poly::order::Lex;
//! use gbx_poly::ring::Ring;
//!
//! let field = Fp::prime(32003).unwrap();
//!
//! let ring = Ring::builder()
//!     .field(field)
//!     .order(Lex)
//!     .nvars(4)
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(ring.nvars, 4);
//! ```
//!
//! Polynomial operations should receive `&RingCtx` explicitly:
//!
//! ```ignore
//! let sum = p.add(&ring, &q)?;
//! let product = p.mul(&ring, &q)?;
//! ```

mod builder;
mod ctx;
mod error;
mod field;
mod ring;

pub use builder::RingBuilder;
pub use ctx::{RingCtx, RingId};
pub use error::{RingError, RingResult};
pub use field::FieldCtx;
pub use ring::Ring;
