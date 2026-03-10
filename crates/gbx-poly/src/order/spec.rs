//! Runtime-selectable monomial orders.
//!
//! This is for user input (CLI/config) where order is not known at compile time.

use core::cmp::Ordering;
use core::str::FromStr;

use crate::order::{MonomialOrder, GREVLEX, LEX};

/// Runtime-selectable monomial order.
///
/// Use this when the order is chosen from user input (CLI/config).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderSpec {
    /// Lexicographic order.
    Lex,
    /// Graded reverse lexicographic order.
    Grevlex,
}

impl OrderSpec {
    /// Canonical lowercase name.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            OrderSpec::Lex => "lex",
            OrderSpec::Grevlex => "grevlex",
        }
    }
}

/// Error returned when parsing an order from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct OrderParseError {
    got: String,
}

impl OrderParseError {
    /// Formating error
    #[inline]
    pub fn got(&self) -> &str {
        &self.got
    }
}

impl core::fmt::Display for OrderParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "unknown monomial order: {}", self.got)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OrderParseError {}

impl FromStr for OrderSpec {
    type Err = OrderParseError;

    /// Parse a monomial order from a string.
    ///
    /// Accepted forms (case-insensitive, surrounding whitespace ignored):
    /// - `"lex"`, `"lexicographic"`
    /// - `"grevlex"`, `"gradedrevlex"`, `"graded_reverse_lex"`, `"graded-reverse-lex"`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = s.trim().to_ascii_lowercase();
        match key.as_str() {
            "lex" | "lexicographic" => Ok(OrderSpec::Lex),
            "grevlex" | "gradedrevlex" | "graded_reverse_lex" | "graded-reverse-lex" | "graded reverse lex" | "graded reverse lexicographic" => Ok(OrderSpec::Grevlex),
            _ => Err(OrderParseError { got: key.to_string() }),
        }
    }
}

impl MonomialOrder for OrderSpec {
    #[inline]
    fn cmp_exps(&self, a: &[u32], b: &[u32]) -> Ordering {
        match self {
            OrderSpec::Lex => LEX.cmp_exps(a, b),
            OrderSpec::Grevlex => GREVLEX.cmp_exps(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::FixedMonomial;
    use crate::order::MonomialOrder;
    use core::cmp::Ordering;

    #[test]
    fn parse_lex_variants() {
        assert_eq!(OrderSpec::from_str("lex").unwrap(), OrderSpec::Lex);
        assert_eq!(OrderSpec::from_str(" Lex ").unwrap(), OrderSpec::Lex);
        assert_eq!(
            OrderSpec::from_str("lexicographic").unwrap(),
            OrderSpec::Lex
        );
    }

    #[test]
    fn parse_grevlex_variants() {
        assert_eq!(OrderSpec::from_str("grevlex").unwrap(), OrderSpec::Grevlex);
        assert_eq!(
            OrderSpec::from_str("GRADEDREVLEX").unwrap(),
            OrderSpec::Grevlex
        );
        assert_eq!(
            OrderSpec::from_str("graded_reverse_lex").unwrap(),
            OrderSpec::Grevlex
        );
        assert_eq!(
            OrderSpec::from_str("graded-reverse-lex").unwrap(),
            OrderSpec::Grevlex
        );
    }

    #[test]
    fn parse_unknown_is_error() {
        let err = OrderSpec::from_str("wat").unwrap_err();
        assert!(err.got().contains("wat"));
    }

    #[test]
    fn orderspec_compares_like_lex() {
        let o = OrderSpec::Lex;
        let a = FixedMonomial::<2>::from_exponents([1, 5]);
        let b = FixedMonomial::<2>::from_exponents([2, 0]);
        assert_eq!(o.cmp(&a, &b), Ordering::Less);
    }
}
