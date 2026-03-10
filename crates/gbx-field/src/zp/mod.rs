//! Integers modulo `P`: `Zp<P>` and `ZpDyn`.
//!
//! `Zp<P>` represents the quotient ring `ℤ / Pℤ`.
//!
//! - Always a **commutative ring** for `P >= 2`.
//! - Not always a field (composite `P` has non-units).
//!
//! If you need a field API, use [`crate::fp::Fp`].

mod dynamic_zp;
mod static_zp;

pub use dynamic_zp::{ZpDyn, ZpDynElem};
pub use static_zp::Zp;
