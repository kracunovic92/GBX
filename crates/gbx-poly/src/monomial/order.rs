//! Monomial orders (term orders) for Gröbner basis computations.
//!
//! This module defines:
//! - [`MonomialOrder`] – a trait for total orders on exponent vectors.
//! - [`Lex`]          – lexicographic order.
//! - [`Grevlex`]      – graded reverse lexicographic order.
//!
//! The orders are defined over **exponent slices** (`&[u64]`) rather than a
//! specific monomial type. Any monomial representation can use these orders
//! as long as it can expose its exponent vector (e.g. via [`MonomialLike::exponents`]).
//!
//! # Conventions
//! - All comparisons assume both exponent slices have the same length.
//!   A mismatch is considered a programmer error and will panic.
//! - These orders are *pure* and deterministic (no hashing / randomness).

use core::cmp::Ordering;

/// A monomial order (term order) on exponent vectors.
///
/// Gröbner basis algorithms assume a fixed monomial order (lex, grevlex, ...).
/// Here we define orders as comparisons on exponent slices, which makes them
/// usable for any monomial representation.
///
/// Implementations must define a **total order**.
pub trait MonomialOrder {
    /// Compare exponent vectors `a` and `b` according to this order.
    ///
    /// # Panics
    ///
    /// Panics if `a.len() != b.len()`. Comparing monomials with differing
    /// variable counts is a programmer error.
    fn cmp_exps(a: &[u32], b: &[u32]) -> Ordering;

    /// Convenience helper: compare monomials by calling `exponents()`.
    #[inline]
    fn cmp<M: crate::monomial::MonomialView>(a: &M, b: &M) -> Ordering {
        Self::cmp_exps(a.exponents(), b.exponents())
    }
}

/// Lexicographic order ("lex") with the usual convention:
///
/// Compare exponents from the first variable to the last. For exponent vectors
/// `α` and `β`, find the smallest index `i` with `α[i] != β[i]` and compare
/// `α[i]` and `β[i]`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Lex;

impl MonomialOrder for Lex {
    #[inline]
    fn cmp_exps(a: &[u32], b: &[u32]) -> Ordering {
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

impl MonomialOrder for Grevlex {
    #[inline]
    fn cmp_exps(a: &[u32], b: &[u32]) -> Ordering {
        assert_eq!(
            a.len(),
            b.len(),
            "Grevlex::cmp_exps called with exponent vectors of differing lengths"
        );

        let deg_a: u128 = a.iter().map(|&x| x as u128).sum();
        let deg_b: u128 = b.iter().map(|&x| x as u128).sum();

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
}

#[cfg(test)]
mod tests {
    use super::{Grevlex, Lex, MonomialOrder};
    use crate::monomial::fixed::FixedMonomial;
    use crate::monomial::{DynamicMonomial as DM, MonomialView};
    use core::cmp::Ordering;

    type M2 = FixedMonomial<2>;
    type M3 = FixedMonomial<3>;

    #[test]
    fn lex_order_compares_from_first_variable_fixed() {
        let a = M2::from_exponents([1, 5]);
        let b = M2::from_exponents([2, 0]);

        assert_eq!(Lex::cmp(&a, &b), Ordering::Less);
        assert_eq!(Lex::cmp(&b, &a), Ordering::Greater);

        let c = M2::from_exponents([3, 1]);
        let d = M2::from_exponents([3, 4]);
        assert_eq!(Lex::cmp(&c, &d), Ordering::Less);
        assert_eq!(Lex::cmp(&d, &c), Ordering::Greater);
    }

    #[test]
    fn lex_order_treats_identical_exponents_as_equal_fixed() {
        let a = M3::from_exponents([2, 0, 5]);
        let b = M3::from_exponents([2, 0, 5]);
        assert_eq!(Lex::cmp(&a, &b), Ordering::Equal);
        assert_eq!(Lex::cmp(&b, &a), Ordering::Equal);
    }

    #[test]
    fn lex_order_works_for_dynamic() {
        let a = DM::from_slice(&[1, 5]);
        let b = DM::from_slice(&[2, 0]);

        assert_eq!(Lex::cmp(&a, &b), Ordering::Less);
        assert_eq!(Lex::cmp(&b, &a), Ordering::Greater);
    }

    #[test]
    fn grevlex_compares_by_total_degree_first_fixed() {
        let a = M3::from_exponents([1, 0, 0]);
        let b = M3::from_exponents([0, 2, 0]);

        assert_eq!(Grevlex::cmp(&a, &b), Ordering::Less);
        assert_eq!(Grevlex::cmp(&b, &a), Ordering::Greater);
    }

    #[test]
    fn grevlex_breaks_ties_with_reverse_lex_rule_fixed() {
        let a = M3::from_exponents([3, 0, 0]);
        let b = M3::from_exponents([2, 1, 0]);
        let c = M3::from_exponents([1, 2, 0]);

        assert_eq!(Grevlex::cmp(&a, &b), Ordering::Greater);
        assert_eq!(Grevlex::cmp(&b, &c), Ordering::Greater);
        assert_eq!(Grevlex::cmp(&a, &c), Ordering::Greater);

        let d = M3::from_exponents([2, 1, 0]);
        assert_eq!(Grevlex::cmp(&b, &d), Ordering::Equal);
    }

    #[test]
    fn grevlex_works_for_dynamic() {
        let a = DM::from_slice(&[1, 0, 0]);
        let b = DM::from_slice(&[0, 2, 0]);

        assert_eq!(Grevlex::cmp(&a, &b), Ordering::Less);
        assert_eq!(Grevlex::cmp(&b, &a), Ordering::Greater);
    }

    #[test]
    fn grevlex_breaks_ties_for_dynamic() {
        let a = DM::from_slice(&[3, 0, 0]);
        let b = DM::from_slice(&[2, 1, 0]);
        let c = DM::from_slice(&[1, 2, 0]);

        assert_eq!(Grevlex::cmp(&a, &b), Ordering::Greater);
        assert_eq!(Grevlex::cmp(&b, &c), Ordering::Greater);
        assert_eq!(Grevlex::cmp(&a, &c), Ordering::Greater);

        let d = DM::from_slice(&[2, 1, 0]);
        assert_eq!(Grevlex::cmp(&b, &d), Ordering::Equal);
    }

    #[test]
    fn cmp_exps_matches_cmp_for_fixed() {
        let a = M3::from_exponents([2, 1, 0]);
        let b = M3::from_exponents([2, 0, 1]);

        assert_eq!(
            Lex::cmp(&a, &b),
            Lex::cmp_exps(a.exponents(), b.exponents())
        );
        assert_eq!(
            Grevlex::cmp(&a, &b),
            Grevlex::cmp_exps(a.exponents(), b.exponents())
        );
    }
}
