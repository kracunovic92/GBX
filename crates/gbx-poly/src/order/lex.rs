//! Lexicographic monomial order.

use core::cmp::Ordering;

use crate::order::traits::MonomialOrder;

/// Lexicographic order.
///
/// Compares exponents from the first variable to the last.
/// At the first differing index, the larger exponent gives the larger monomial.
#[derive(Debug, Clone, Copy, Default)]
pub struct Lex;

/// Singleton value for lexicographic order.
pub const LEX: Lex = Lex;

impl MonomialOrder for Lex {
    #[inline]
    fn cmp_exps(&self, a: &[u32], b: &[u32]) -> Ordering {
        assert_eq!(
            a.len(),
            b.len(),
            "Lex::cmp_exps called with exponent vectors of different lengths"
        );

        for (&ai, &bi) in a.iter().zip(b.iter()) {
            match ai.cmp(&bi) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::LEX;
    use crate::monomial::Monomial;
    use crate::order::MonomialOrder;
    use core::cmp::Ordering;

    #[test]
    fn lex_compares_from_first_variable() {
        let a = Monomial::from_slice(&[1, 5]);
        let b = Monomial::from_slice(&[2, 0]);

        assert_eq!(LEX.cmp(&a, &b), Ordering::Less);
        assert_eq!(LEX.cmp(&b, &a), Ordering::Greater);

        let c = Monomial::from_slice(&[3, 1]);
        let d = Monomial::from_slice(&[3, 4]);

        assert_eq!(LEX.cmp(&c, &d), Ordering::Less);
        assert_eq!(LEX.cmp(&d, &c), Ordering::Greater);
    }

    #[test]
    fn lex_treats_identical_exponents_as_equal() {
        let a = Monomial::from_slice(&[2, 0, 5]);
        let b = Monomial::from_slice(&[2, 0, 5]);

        assert_eq!(LEX.cmp(&a, &b), Ordering::Equal);
    }

    #[test]
    #[should_panic(expected = "different lengths")]
    fn lex_panics_on_mismatched_arity() {
        let a = Monomial::from_slice(&[1, 0]);
        let b = Monomial::from_slice(&[1, 0, 0]);

        let _ = LEX.cmp(&a, &b);
    }
}
