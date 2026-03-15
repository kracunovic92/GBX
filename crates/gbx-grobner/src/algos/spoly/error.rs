use gbx_poly::monomial::MonomialError;
use gbx_poly::polynomial::PolynomialError;
use thiserror::Error;

/// Errors that can occur while constructing an S-polynomial.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum SPolyError {
    /// Propagated polynomial, term, monomial, or ring error.
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    /// One input polynomial was zero.
    #[error("cannot form S-polynomial: one input polynomial is zero")]
    ZeroInput,

    /// A leading coefficient was not invertible in the field context.
    #[error("cannot form S-polynomial: leading coefficient is not invertible")]
    NonInvertibleLeadingCoefficient,
}

impl From<MonomialError> for SPolyError {
    #[inline]
    fn from(e: MonomialError) -> Self {
        Self::Poly(PolynomialError::from(e))
    }
}
