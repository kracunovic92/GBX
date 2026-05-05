//! Gröbner basis postprocessing.
//!
//! These operations are algorithm-independent and can be applied to Gröbner
//! bases produced by Buchberger, F4, or other Gröbner basis engines.

mod error;
mod minimize;
mod reduction;
mod types;

pub use error::PostError;
pub use minimize::{make_monic_in_place, minimize_in_place};
pub use reduction::reduce_in_place;

/// Requested level of Gröbner basis postprocessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasisPostOptionsKind {
    /// Skip postprocesssing
    None,
    /// Just do minimial base
    Minimal,
    /// Go to reduced base
    Reduced,
}
