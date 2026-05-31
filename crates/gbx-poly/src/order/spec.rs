//! Runtime-selectable monomial orders.

use core::cmp::Ordering;
use core::str::FromStr;

use crate::order::{GREVLEX, LEX, MonomialOrder};

/// Runtime-selectable monomial order.
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
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lex => "lex",
            Self::Grevlex => "grevlex",
        }
    }
}

/// Error returned when parsing an unknown monomial order.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct OrderParseError {
    got: String,
}

impl OrderParseError {
    /// Returns the normalized input string that failed to parse.
    #[inline]
    #[must_use]
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = s.trim().to_ascii_lowercase();

        match key.as_str() {
            "lex" | "lexicographic" => Ok(Self::Lex),

            "grevlex" | "gradedrevlex" | "graded_reverse_lex" | "graded-reverse-lex" | "graded reverse lex" | "graded reverse lexicographic" => Ok(Self::Grevlex),

            _ => Err(OrderParseError { got: key }),
        }
    }
}

impl MonomialOrder for OrderSpec {
    #[inline]
    fn cmp_exps(&self, a: &[u32], b: &[u32]) -> Ordering {
        match self {
            Self::Lex => LEX.cmp_exps(a, b),
            Self::Grevlex => GREVLEX.cmp_exps(a, b),
        }
    }

    #[inline]
    fn cmp<M: crate::monomial::MonomialView>(&self, a: &M, b: &M) -> Ordering {
        match self {
            Self::Lex => LEX.cmp(a, b),
            Self::Grevlex => GREVLEX.cmp(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::Monomial;

    fn parsed(input: &str) -> OrderSpec {
        match OrderSpec::from_str(input) {
            Ok(order) => order,
            Err(err) => panic!("order should parse: {err}"),
        }
    }

    #[test]
    fn parse_lex_variants() {
        assert_eq!(parsed("lex"), OrderSpec::Lex);
        assert_eq!(parsed(" Lex "), OrderSpec::Lex);
        assert_eq!(parsed("lexicographic"), OrderSpec::Lex);
    }

    #[test]
    fn parse_grevlex_variants() {
        assert_eq!(parsed("grevlex"), OrderSpec::Grevlex);
        assert_eq!(parsed("GRADEDREVLEX"), OrderSpec::Grevlex);
        assert_eq!(parsed("graded_reverse_lex"), OrderSpec::Grevlex);
        assert_eq!(parsed("graded-reverse-lex"), OrderSpec::Grevlex);
    }

    #[test]
    fn parse_unknown_is_error() {
        match OrderSpec::from_str("wat") {
            Ok(order) => panic!("unknown order should not parse: {order:?}"),
            Err(err) => assert!(err.got().contains("wat")),
        }
    }

    #[test]
    fn orderspec_compares_like_lex() {
        let o = OrderSpec::Lex;

        let a = Monomial::from_slice(&[1, 5]);
        let b = Monomial::from_slice(&[2, 0]);

        assert_eq!(o.cmp(&a, &b), Ordering::Less);
    }

    #[test]
    fn orderspec_compares_like_grevlex() {
        let o = OrderSpec::Grevlex;

        let a = Monomial::from_slice(&[1, 0, 0]);
        let b = Monomial::from_slice(&[0, 2, 0]);

        assert_eq!(o.cmp(&a, &b), Ordering::Less);
    }
}
