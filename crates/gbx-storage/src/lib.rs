//! `gbx-storage` provides small, reusable storage backends used by higher-level crates.
//!
//! The initial focus is exponent vectors for monomials/polynomials. The goal is to keep
//! storage concerns (heap vs inline, word size, conversions) out of `gbx-poly`.

#![forbid(unsafe_code)]
#![warn(clippy::as_conversions)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod exponents;
pub mod polynomial;
pub mod prelude;
