#![forbid(unsafe_code)]
#![warn(clippy::as_conversions)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

//! # gbx-field
//!
//! Concrete number systems for the GBX ecosystem.
//!
//! Types:
//! - [`zp::Zp`]: integers mod `P` (a commutative ring)
//! - [`fp::Fp`]: prime field mod `P` (a field API, optional runtime prime validation)
//! - [`complex`]: floating complex numbers (std-only; not algebraically exact)

mod macros;

mod error;
pub mod fp;
/// Convenient re-exports for common GBX field types.
///
/// This is intended for ergonomic imports in downstream crates:
/// ```
/// use gbx_field::prelude::*;
/// ```
pub mod prelude;
pub mod zp;
