//! Exponent storage backends.
//!
//! This module provides minimal containers for exponent vectors plus tiny traits
//! ([`Exps`], [`ExpsMut`]) so higher layers (e.g. `gbx-poly`) can operate on
//! monomials without committing to a concrete representation.
//!
//! # Provided backends
//! - [`FixedExps`]: inline `[E; N]` storage for compile-time arity.
//! - [`DynExps`]: heap `Box<[E]>` storage for runtime arity.
//!
//! (You can add more later, e.g. inline dynamic buffers or packed exponent words.)

mod dynamic_exps;
mod error;
mod fixed_exps;
mod traits;

pub use dynamic_exps::DynExps;
pub use error::ExpsError;
pub use fixed_exps::{FixedExps, FixedExps32};
pub use traits::{Exps, ExpsMut};
