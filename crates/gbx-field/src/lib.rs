#![forbid(unsafe_code)]
#![warn(clippy::as_conversions)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

//! Runtime prime fields for the GBX ecosystem.
//!
//! This crate provides the coefficient field implementation used by
//! `gbx-poly` and Gröbner basis algorithms.
//!
//! The main type is [`fp::Fp`], a runtime prime-field context.
//!
//! Elements are represented by [`fp::FpElem`]. An element stores only its
//! reduced representative; it does not store the modulus. Arithmetic is
//! performed by the surrounding [`fp::Fp`] context.
//!
//! # Example
//!
//! ```
//! use gbx_field::fp::Fp;
//!
//! let f = Fp::prime(7).unwrap();
//!
//! let a = f.elem(5);
//! let b = f.elem(6);
//!
//! assert_eq!(f.repr_u32(f.add(a, b)), 4);
//! assert_eq!(f.repr_u32(f.mul(a, b)), 2);
//! ```

mod error;

pub mod fp;
pub mod prelude;

pub use error::*;
