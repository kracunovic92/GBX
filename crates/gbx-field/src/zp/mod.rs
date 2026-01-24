//! Integers modulo `P`: `Zp<P>`.
//!
//! `Zp<P>` represents the quotient ring `ℤ / Pℤ`.
//!
//! - Always a **commutative ring** for `P >= 2`.
//! - Not always a field (composite `P` has non-units).
//!
//! If you need a field API, use [`crate::fp::Fp`].

mod core;
mod ops;

#[cfg(test)]
mod tests;

pub use core::Zp;
