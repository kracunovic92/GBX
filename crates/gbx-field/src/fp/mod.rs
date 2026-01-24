//! Prime field `Fp<P>`.
//!
//! `Fp<P>` is the field API for modulus `P`.
//! - Implements `TryInverse`, `CheckedDiv`, and thus `Field`.
//! - Internally stores a `Zp<P>`.
//!
//! Optional feature `prime-check` can validate that `P` is prime at runtime.

mod core;
mod inv;
mod ops;

#[cfg(any(test, feature = "prime-check"))]
mod validate;

#[cfg(test)]
mod tests;

pub use core::Fp;
