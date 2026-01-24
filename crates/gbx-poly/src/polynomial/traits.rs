//! Core polynomial traits used by algorithms.
//!
//! The goal is to keep these traits **small and stable**:
//! - Algorithms (division, reduction, Buchberger/F4/F5) should depend on these traits,
//!   not on concrete polynomial types.
//! - Concrete polynomial types (fixed/dynamic, different storages) implement these traits.
//!
//! **Important:** do not bake heavy bounds (like `Clone`, `PartialEq`) into the trait
//! unless you truly need them. Put such bounds on the algorithms that require them.

extern crate alloc;

use alloc::vec::Vec;

use gbx_alg::Field;

use crate::monomial::MonomialOrder;
use crate::term::traits::TermView;

/// Read-only polynomial interface.
///
/// This is the main trait algorithms should accept for *inspection*.
///
/// Required invariant (recommended):
/// - `terms()` are sorted in descending order by `Order`, so `leading_term()` is O(1).
pub trait PolynomialView {
    /// Coefficient field.
    type Field: Field;

    /// Sparse term representation.
    type Term: TermView<Field = Self::Field>;

    /// Monomial order used by algorithms.
    type Order: MonomialOrder;

    /// Returns `true` if the polynomial is exactly zero.
    #[inline]
    fn is_zero(&self) -> bool {
        self.terms().is_empty()
    }

    /// Returns an immutable view of the internal term slice.
    fn terms(&self) -> &[Self::Term];

    /// Leading term with respect to `Order`.
    ///
    /// If the polynomial is normalized (sorted descending), this is the first term.
    #[inline]
    fn leading_term(&self) -> Option<&Self::Term> {
        self.terms().first()
    }

    /// Leading monomial (`lm`) with respect to `Order`.
    #[inline]
    fn leading_monomial(&self) -> Option<&<Self::Term as TermView>::Mono> {
        self.leading_term().map(|t| t.mono())
    }
    /// Leading coefficient (`lc`) with respect to `Order`.
    #[inline]
    fn leading_coefficient(&self) -> Option<&Self::Field> {
        self.leading_term().map(|t| t.coeff())
    }
}

/// Minimal mutation hooks needed by generic algorithms.
///
/// Algorithms typically need:
/// - a way to construct `0`
/// - a way to build from terms (normalizing internally)
/// - a way to append terms and normalize
pub trait PolynomialMut: PolynomialView {
    /// Create the zero polynomial.
    fn zero() -> Self;

    /// Build from raw terms (must normalize internally).
    fn from_terms(terms: Vec<Self::Term>) -> Self
    where
        Self::Field: gbx_alg::Zero + Clone;

    /// Push a single term (may temporarily violate invariants).
    fn push_term(&mut self, term: Self::Term);

    /// Normalize in-place:
    /// - drop zero coefficients
    /// - merge like monomials
    /// - sort by `Order` descending
    fn normalize_in_place(&mut self)
    where
        Self::Field: gbx_alg::Zero + Clone;
}
