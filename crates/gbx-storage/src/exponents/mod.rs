//! Exponent storage backends.
//!
//! This module contains minimal containers for exponent vectors and a trait [`Exps`].
//! `gbx-poly` can depend on these types without committing to a specific representation.

mod dynamic_exps;
mod error;
mod fixed_exps;
mod traits;

pub use dynamic_exps::DynExps;
pub use error::ExpsError;
pub use fixed_exps::{FixedExps, FixedExps32};
pub use traits::Exps;
