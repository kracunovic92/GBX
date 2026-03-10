use core::fmt;

use crate::monomial::MonomialError;

/// Errors that can occur when operating on terms.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TermError {
    /// Monomial arithmetic failed (overflow / mismatched arity, etc.).
    Monomial(MonomialError),
}

impl From<MonomialError> for TermError {
    #[inline]
    fn from(e: MonomialError) -> Self {
        TermError::Monomial(e)
    }
}

impl fmt::Display for TermError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermError::Monomial(e) => write!(f, "monomial error: {e}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TermError {}

/// Convenience alias for term results.
pub type Result<T> = core::result::Result<T, TermError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::MonomialError;

    #[test]
    fn monomial_error_converts() {
        let e = MonomialError::DegreeOverflow;
        let te: TermError = e.clone().into();
        assert_eq!(te, TermError::Monomial(e));
    }

    #[test]
    fn display_includes_monomial_prefix() {
        let te: TermError = MonomialError::DegreeOverflow.into();
        let s = te.to_string();
        assert!(s.contains("monomial error"));
    }
}
