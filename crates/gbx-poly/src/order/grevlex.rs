//! Graded reverse lexicographic monomial order.

use core::cmp::Ordering;

use crate::monomial::MonomialView;
use crate::order::traits::MonomialOrder;

/// Graded reverse lexicographic order.
///
/// Comparison rule:
/// 1. Compare total degree.
/// 2. If degrees are equal, scan exponents from the last variable backwards.
/// 3. At the last differing variable, the monomial with the smaller exponent
///    is considered larger.
#[derive(Debug, Clone, Copy, Default)]
pub struct Grevlex;

/// Singleton value for graded reverse lexicographic order.
pub const GREVLEX: Grevlex = Grevlex;

impl Grevlex {
    #[inline]
    fn degree_from_exps(exps: &[u32]) -> u128 {
        exps.iter().map(|&x| u128::from(x)).sum()
    }

    #[inline]
    fn cmp_same_degree(a: &[u32], b: &[u32]) -> Ordering {
        for (&ai, &bi) in a.iter().rev().zip(b.iter().rev()) {
            if ai != bi {
                return bi.cmp(&ai);
            }
        }

        Ordering::Equal
    }
}

impl MonomialOrder for Grevlex {
    #[inline]
    fn cmp_exps(&self, a: &[u32], b: &[u32]) -> Ordering {
        assert_eq!(
            a.len(),
            b.len(),
            "Grevlex::cmp_exps called with exponent vectors of different lengths"
        );

        let deg_a = Self::degree_from_exps(a);
        let deg_b = Self::degree_from_exps(b);

        match deg_a.cmp(&deg_b) {
            Ordering::Equal => Self::cmp_same_degree(a, b),
            non_eq => non_eq,
        }
    }

    #[inline]
    fn cmp<M: MonomialView>(&self, a: &M, b: &M) -> Ordering {
        let ae = a.exponents();
        let be = b.exponents();

        assert_eq!(
            ae.len(),
            be.len(),
            "Grevlex::cmp called with exponent vectors of different lengths"
        );

        match a.degree().cmp(&b.degree()) {
            Ordering::Equal => Self::cmp_same_degree(ae, be),
            non_eq => non_eq,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GREVLEX;
    use crate::monomial::Monomial;
    use crate::order::MonomialOrder;
    use core::cmp::Ordering;

    #[test]
    fn grevlex_compares_by_total_degree_first() {
        let a = Monomial::from_slice(&[1, 0, 0]);
        let b = Monomial::from_slice(&[0, 2, 0]);

        assert_eq!(GREVLEX.cmp(&a, &b), Ordering::Less);
        assert_eq!(GREVLEX.cmp(&b, &a), Ordering::Greater);
    }

    #[test]
    fn grevlex_breaks_ties_with_reverse_lex_rule() {
        let a = Monomial::from_slice(&[3, 0, 0]);
        let b = Monomial::from_slice(&[2, 1, 0]);
        let c = Monomial::from_slice(&[1, 2, 0]);

        assert_eq!(GREVLEX.cmp(&a, &b), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&b, &c), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&a, &c), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&b, &b), Ordering::Equal);
    }

    #[test]
    fn grevlex_treats_identical_exponents_as_equal() {
        let a = Monomial::from_slice(&[2, 0, 5]);
        let b = Monomial::from_slice(&[2, 0, 5]);

        assert_eq!(GREVLEX.cmp(&a, &b), Ordering::Equal);
    }

    #[test]
    #[should_panic(expected = "different lengths")]
    fn grevlex_panics_on_mismatched_arity() {
        let a = Monomial::from_slice(&[1, 0]);
        let b = Monomial::from_slice(&[1, 0, 0]);

        let _ = GREVLEX.cmp(&a, &b);
    }
}
