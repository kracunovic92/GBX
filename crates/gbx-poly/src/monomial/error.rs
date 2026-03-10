use core::fmt;

/// Errors that can occur when operating on monomials.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MonomialError {
    /// Overflow during component-wise exponent addition.
    ExponentOverflow { index: usize, lhs: u32, rhs: u32 },

    /// Total degree computation overflowed `u32`.
    DegreeOverflow,

    /// The two monomials had different arities.
    MismatchedArity { lhs: usize, rhs: usize },

    /// Constructor received an unexpected exponent count.
    WrongLength { expected: usize, got: usize },

    /// Attempted exact division but the divisor does not divide the dividend.
    NotDivisible,
}

impl fmt::Display for MonomialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonomialError::ExponentOverflow { index, lhs, rhs } => {
                write!(f, "exponent overflow at index {index}: {lhs} + {rhs}")
            }
            MonomialError::DegreeOverflow => write!(f, "total degree overflow"),
            MonomialError::MismatchedArity { lhs, rhs } => {
                write!(f, "mismatched arity: {lhs} vs {rhs}")
            }
            MonomialError::WrongLength { expected, got } => {
                write!(f, "wrong exponent length: expected {expected}, got {got}")
            }
            MonomialError::NotDivisible => {
                write!(f, "not divisible")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MonomialError {}

/// Convenience alias for monomial results.
pub type Result<T> = core::result::Result<T, MonomialError>;
