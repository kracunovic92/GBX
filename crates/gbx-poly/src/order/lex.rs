//! Lexicographic monomial order.

use core::cmp::Ordering;

use crate::order::traits::MonomialOrder;

/// Lexicographic order ("lex") with the usual convention:
///
/// Compare exponents from the first variable to the last. For exponent vectors
/// `α` and `β`, find the smallest index `i` with `α[i] != β[i]` and compare
/// `α[i]` and `β[i]`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Lex;

/// Ergonomic singleton for `Lex`.
///
/// This is zero-cost: `Lex` is a ZST.
pub const LEX: Lex = Lex;

impl MonomialOrder for Lex {
    #[inline]
    fn cmp_exps(&self, a: &[u32], b: &[u32]) -> Ordering {
        assert_eq!(
            a.len(),
            b.len(),
            "Lex::cmp_exps called with exponent vectors of differing lengths"
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
    use crate::monomial::{DynamicMonomial, FixedMonomial};
    use crate::order::MonomialOrder;
    use core::cmp::Ordering;

    type M2 = FixedMonomial<2>;
    type M3 = FixedMonomial<3>;
    type DM = DynamicMonomial;

    #[test]
    fn lex_compares_from_first_variable_fixed() {
        let a = M2::from_exponents([1, 5]);
        let b = M2::from_exponents([2, 0]);

        assert_eq!(LEX.cmp(&a, &b), Ordering::Less);
        assert_eq!(LEX.cmp(&b, &a), Ordering::Greater);

        let c = M2::from_exponents([3, 1]);
        let d = M2::from_exponents([3, 4]);

        assert_eq!(LEX.cmp(&c, &d), Ordering::Less);
        assert_eq!(LEX.cmp(&d, &c), Ordering::Greater);
    }

    #[test]
    fn lex_treats_identical_exponents_as_equal_fixed() {
        let a = M3::from_exponents([2, 0, 5]);
        let b = M3::from_exponents([2, 0, 5]);
        assert_eq!(LEX.cmp(&a, &b), Ordering::Equal);
    }

    #[test]
    fn lex_works_for_dynamic() {
        let a = DM::from_slice(&[1, 5]);
        let b = DM::from_slice(&[2, 0]);

        assert_eq!(LEX.cmp(&a, &b), Ordering::Less);
        assert_eq!(LEX.cmp(&b, &a), Ordering::Greater);
    }
}
