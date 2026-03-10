//! Prime field `Fp<P>` and `FpDyn`.
//!
//! `Fp<P>` is the field API for modulus `P`.
//! - Internally stores a `Zp<P>`.
//! - Provides inversion / division.
//!
//! Feature `prime-check` can validate that `P` is prime (once per concrete `P`).

mod dynamic_fp;
mod prime;
mod static_fp;

pub use dynamic_fp::{FpDyn, FpDynElem};
pub use static_fp::Fp;
