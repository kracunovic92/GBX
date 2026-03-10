//! Graded reverse lexicographic (grevlex) monomial order.

use core::cmp::Ordering;

use crate::monomial::MonomialView;
use crate::order::traits::MonomialOrder;

/// Graded reverse lexicographic order ("grevlex").
///
/// 1. Compare total degrees (sum of exponents).
/// 2. If equal, compare from the last variable backwards; at the first
///    difference reverse the comparison.
///
/// Formally, for exponent vectors `α` and `β`:
/// - If `|α| < |β|` then `x^α < x^β`.
/// - If `|α| == |β|` and `i` is the largest index with `α_i != β_i`,
///   then `x^α > x^β` iff `α_i < β_i`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Grevlex;

/// Ergonomic singleton for `Grevlex`.
///
/// This is zero-cost: `Grevlex` is a ZST.
pub const GREVLEX: Grevlex = Grevlex;

impl Grevlex {
    #[inline]
    fn degree_from_exps(exps: &[u32]) -> u128 {
        exps.iter().map(|&x| x as u128).sum()
    }
}

impl MonomialOrder for Grevlex {
    #[inline]
    fn cmp_exps(&self, a: &[u32], b: &[u32]) -> Ordering {
        assert_eq!(
            a.len(),
            b.len(),
            "Grevlex::cmp_exps called with exponent vectors of differing lengths"
        );

        let deg_a = Self::degree_from_exps(a);
        let deg_b = Self::degree_from_exps(b);

        match deg_a.cmp(&deg_b) {
            Ordering::Equal => {
                for (&ai, &bi) in a.iter().rev().zip(b.iter().rev()) {
                    if ai != bi {
                        return bi.cmp(&ai);
                    }
                }
                Ordering::Equal
            }
            non_eq => non_eq,
        }
    }

    /// Override `cmp` to use cached degree when available.
    #[inline]
    fn cmp<M: MonomialView<Word = u32>>(&self, a: &M, b: &M) -> Ordering {
        let ae = a.exponents();
        let be = b.exponents();

        assert_eq!(
            ae.len(),
            be.len(),
            "Grevlex::cmp called with exponent vectors of differing lengths"
        );

        let deg_a: u128 = a
            .degree_hint()
            .map(|d| d as u128)
            .unwrap_or_else(|| Self::degree_from_exps(ae));
        let deg_b: u128 = b
            .degree_hint()
            .map(|d| d as u128)
            .unwrap_or_else(|| Self::degree_from_exps(be));

        match deg_a.cmp(&deg_b) {
            Ordering::Equal => {
                for (&ai, &bi) in ae.iter().rev().zip(be.iter().rev()) {
                    if ai != bi {
                        return bi.cmp(&ai);
                    }
                }
                Ordering::Equal
            }
            non_eq => non_eq,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GREVLEX;
    use crate::monomial::{DynamicMonomial, FixedMonomial};
    use crate::order::MonomialOrder;
    use core::cmp::Ordering;

    type M3 = FixedMonomial<3>;
    type DM = DynamicMonomial;

    #[test]
    fn grevlex_compares_by_total_degree_first_fixed() {
        let a = M3::from_exponents([1, 0, 0]);
        let b = M3::from_exponents([0, 2, 0]);

        assert_eq!(GREVLEX.cmp(&a, &b), Ordering::Less);
        assert_eq!(GREVLEX.cmp(&b, &a), Ordering::Greater);
    }

    #[test]
    fn grevlex_breaks_ties_with_reverse_lex_rule_fixed() {
        let a = M3::from_exponents([3, 0, 0]);
        let b = M3::from_exponents([2, 1, 0]);
        let c = M3::from_exponents([1, 2, 0]);

        assert_eq!(GREVLEX.cmp(&a, &b), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&b, &c), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&a, &c), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&b, &b), Ordering::Equal);
    }

    #[test]
    fn grevlex_works_for_dynamic() {
        let a = DM::from_slice(&[1, 0, 0]);
        let b = DM::from_slice(&[0, 2, 0]);

        assert_eq!(GREVLEX.cmp(&a, &b), Ordering::Less);
        assert_eq!(GREVLEX.cmp(&b, &a), Ordering::Greater);
    }

    #[test]
    fn grevlex_breaks_ties_for_dynamic() {
        let a = DM::from_slice(&[3, 0, 0]);
        let b = DM::from_slice(&[2, 1, 0]);
        let c = DM::from_slice(&[1, 2, 0]);

        assert_eq!(GREVLEX.cmp(&a, &b), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&b, &c), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&a, &c), Ordering::Greater);
        assert_eq!(GREVLEX.cmp(&b, &b), Ordering::Equal);
    }
}
