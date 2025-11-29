use crate::monomial::{MonomialError, MonomialLike, MonomialOrder};
use crate::term::{Term, TermError, TermLike};
use algebra_core::Field;

mod dynamic;
mod polynomial;

/// Errors that can occur when operating on polynomials.
///
/// For now this is mostly a wrapper around [`TermError`], but it gives
/// you a place to hang higher-level errors (e.g. division failures).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolynomialError {
    /// Error originating from term-level operations.
    Term(TermError),
}

impl From<TermError> for PolynomialError {
    #[inline]
    fn from(err: TermError) -> Self {
        PolynomialError::Term(err)
    }
}

/// Common interface for sparse polynomials.
///
/// Implemented by:
/// - [`Polynomial<F, N, O>`] for fixed-size monomials.
/// - [`DynamicPolynomial<F, O>`] for dynamic monomials.
///
/// This trait is *intentionally* read-only: mutation-oriented APIs
/// (like `add_ref`, `neg_ref`) live on the concrete types, while
/// algorithms can depend on `PolynomialLike + Clone + Add + Neg` etc.
pub trait PolynomialLike: Clone + PartialEq {
    /// The underlying field of coefficients.
    type Field: Field;

    /// The monomial representation.
    type Mono: MonomialLike<Error = MonomialError> + Clone;

    /// The monomial order used to keep terms sorted.
    type Order: MonomialOrder<Self::Mono>;

    /// The term type used in the internal sparse representation.
    type Term: TermLike<Field = Self::Field, Mono = Self::Mono, Error = TermError>;

    /// Returns `true` if this is the zero polynomial.
    fn is_zero(&self) -> bool;

    /// Returns an immutable view of the internal term list
    /// (from highest to lowest w.r.t. [`Self::Order`]).
    fn terms(&self) -> &[Self::Term];

    /// Leading term w.r.t. the monomial order (if non-zero).
    fn leading_term(&self) -> Option<&Self::Term>;

    /// Leading monomial (`lm`) w.r.t. the monomial order.
    #[inline]
    fn leading_monomial(&self) -> Option<&Self::Mono> {
        self.leading_term()
            .map(|t| t.mono())
    }

    /// Leading coefficient (`LC`) w.r.t. the monomial order.
    #[inline]
    fn leading_coefficient(&self) -> Option<&Self::Field> {
        self.leading_term()
            .map(|t| t.coeff())
    }

    /// Total degree of the polynomial, if non-zero.
    ///
    /// Defined as the degree of the leading monomial.
    #[inline]
    fn degree(&self) -> Option<u64> {
        self.leading_monomial()
            .map(|m| m.degree())
    }
}

pub use dynamic::DynamicPolynomial;
pub use polynomial::Polynomial;
