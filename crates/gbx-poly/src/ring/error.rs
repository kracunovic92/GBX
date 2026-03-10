use core::fmt;

/// Errors related to ring construction and ring safety checks.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum RingError {
    /// Invalid number of variables. A polynomial ring must have at least 1 variable.
    InvalidNvars { nvars: usize },

    /// Operation attempted with a polynomial created in a different ring context.
    ///
    /// This typically indicates user code mixed polynomials from different rings
    /// (e.g. different modulus/order/arity) in one operation.
    MismatchedRing { expected: u64, got: u64 },
}

impl fmt::Display for RingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RingError::InvalidNvars { nvars } => write!(f, "invalid nvars: {nvars} (must be > 0)"),
            RingError::MismatchedRing { expected, got } => {
                write!(
                    f,
                    "mismatched ring context: expected ring_id={expected}, got ring_id={got}"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RingError {}

/// Convenience alias for ring results.
pub type Result<T> = core::result::Result<T, RingError>;
