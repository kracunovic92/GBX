//! Terms `c * x^α` used in polynomial rings.
//!
//! A **term** is a coefficient multiplied by a monomial (an exponent vector).
//! This module is intentionally small and storage-agnostic.
//!
//! ## Design
//! - [`TermView`] is a read-only abstraction (borrow coefficient + monomial).
//! - [`Term`] is the canonical owned representation.
//! - Arithmetic is implemented as free functions in [`ops`] to keep the core type minimal.
//!
//! ## Ergonomics
//! Use the [`term!`] macro for convenient construction:
//!
//! ```
/// use crate::monomial::{FixedMonomial, DynamicMonomial, MonomialView};
/// use crate::term::Term;
///
/// let a: Term<u32, FixedMonomial<3>> = term![3u32, [1, 0, 2]];
/// let b: Term<u32, DynamicMonomial>  = term![7u32, [0, 1, 0, 3]];
///
/// assert_eq!(*a.coeff_ref(), 3);
/// assert_eq!(a.mono_ref().exponents(), &[1, 0, 2]);
/// assert_eq!(*b.coeff_ref(), 7);
/// assert_eq!(b.mono_ref().exponents(), &[0, 1, 0, 3]);
/// ```
pub mod error;
pub mod ops;
mod owned;
pub mod repr;
pub mod view;

mod display;
mod macros;

pub use display::{TermDisplay, TermStyle};
pub use error::{Result, TermError};
pub use ops::*;
pub use owned::TermOwned;
pub use repr::Term;
pub use view::TermView;

/// Common term type aliases.
///
/// These aliases describe common “dynamic vs static” combinations.
///
/// The coefficient type `F` is typically a field element (e.g. `Fp<P>`, `FpDynElem`),
/// while the monomial determines whether the number of variables is known at
/// compile time (`FixedMonomial<N>`) or only at runtime (`DynamicMonomial`).
pub mod aliases {
    use super::Term;
    use crate::monomial::{DynamicMonomial, FixedMonomial};

    /// Fully dynamic: runtime field element + runtime arity.
    pub type DynamicTerm<F> = Term<F, DynamicMonomial>;

    /// Semi-dynamic #1: dynamic field + fixed arity.
    pub type SemiFieldDyn<F, const N: usize> = Term<F, FixedMonomial<N>>;

    /// Semi-dynamic #2: static field + dynamic arity.
    pub type SemiVarsDyn<F> = Term<F, DynamicMonomial>;
}
