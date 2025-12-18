//! Monomial orders (term orders) for Gröbner basis computations.
//!
//! This module defines:
//! - [`MonomialOrder`] – a trait abstracting over total orders on monomials.
//! - [`Lex`]          – lexicographic order.
//! - [`Grevlex`]      – graded reverse lexicographic order.
//!
//! Both fixed-size [`crate::monomial::Monomial<>`] and
//! dynamic [`crate::monomial::DynamicMonomial`] implement
//! [`MonomialLike`], so they can be ordered by these
//! implementations.

use super::MonomialLike;
use core::cmp::Ordering;

/// A monomial order on monomials of type `M`.
///
/// This trait defines how to compare two monomials. Gröbner basis algorithms
/// always assume a *fixed* monomial order, but the order itself may be
/// chosen by the caller (e.g. lex, grevlex, graded lex, etc.).
pub trait MonomialOrder<M: MonomialLike> {
    /// Compare monomials `a` and `b` according to this order.
    fn cmp(a: &M, b: &M) -> Ordering;
}

/// Lexicographic order ("lex") with the usual convention:
///
/// Compare exponents from the first variable to the last. That is, for
/// exponent vectors `α` and `β`, find the smallest index `i` with
/// `α[i] != β[i]` and compare `α[i]` and `β[i]`.
#[derive(Debug, Clone, Copy)]
pub struct Lex;

impl<M: MonomialLike> MonomialOrder<M> for Lex {
    #[inline]
    fn cmp(a: &M, b: &M) -> Ordering {
        let ae = a.exponents();
        let be = b.exponents();

        debug_assert_eq!(
            ae.len(),
            be.len(),
            "Lex::cmp called with monomials of differing variable counts"
        );

        let len = ae.len();
        let mut i = 0;
        while i < len {
            match ae[i].cmp(&be[i]) {
                Ordering::Equal => {
                    i += 1;
                }
                non_eq => return non_eq,
            }
        }
        Ordering::Equal
    }
}

/// Graded reverse lexicographic order ("grevlex").
///
/// 1. Compare total degrees (sum of exponents).
/// 2. If equal, compare from the last variable backwards, reversing
///    the comparison at the first difference.
///
/// Formally, for monomials `x^α` and `x^β`:
/// - If `|α| < |β|` then `x^α < x^β`.
/// - If `|α| == |β|` and `i` is the largest index with `α_i != β_i`,
///   then `x^α > x^β` iff `α_i < β_i`.
#[derive(Debug, Clone, Copy)]
pub struct Grevlex;

impl<M: MonomialLike> MonomialOrder<M> for Grevlex
where
    // We need to be able to unwrap degree_checked in `degree()`.
    M::Error: core::fmt::Debug,
{
    #[inline]
    fn cmp(a: &M, b: &M) -> Ordering {
        let deg_a = a.degree();
        let deg_b = b.degree();

        match deg_a.cmp(&deg_b) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let ae = a.exponents();
                let be = b.exponents();

                debug_assert_eq!(
                    ae.len(),
                    be.len(),
                    "Grevlex::cmp called with monomials of differing variable counts"
                );

                let mut i = ae.len();
                while i > 0 {
                    i -= 1;
                    match ae[i].cmp(&be[i]) {
                        Ordering::Equal => continue,
                        Ordering::Less => return Ordering::Greater, // reversed
                        Ordering::Greater => return Ordering::Less, // reversed
                    }
                }
                Ordering::Equal
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    use super::{Grevlex, Lex, MonomialOrder};
    use crate::monomial::{DynamicMonomial as DM, Monomial};

    type M2 = Monomial<2>;
    type M3 = Monomial<3>;

    #[test]
    fn lex_order_compares_from_first_variable_fixed() {
        let a = M2::from_exponents([1, 5]); // x1^1 x2^5
        let b = M2::from_exponents([2, 0]); // x1^2 x2^0

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
    fn grevlex_compares_by_total_degree_first_fixed() {
        let a = M3::from_exponents([1, 0, 0]); // deg = 1
        let b = M3::from_exponents([0, 2, 0]); // deg = 2

        assert_eq!(Grevlex::cmp(&a, &b), Ordering::Less);
        assert_eq!(Grevlex::cmp(&b, &a), Ordering::Greater);
    }

    #[test]
    fn grevlex_breaks_ties_with_reverse_lex_rule_fixed() {
        // All have total degree 3
        let a = M3::from_exponents([3, 0, 0]); // x1^3
        let b = M3::from_exponents([2, 1, 0]); // x1^2 x2^1
        let c = M3::from_exponents([1, 2, 0]); // x1^1 x2^2

        assert_eq!(Grevlex::cmp(&a, &b), Ordering::Greater);
        assert_eq!(Grevlex::cmp(&b, &c), Ordering::Greater);
        assert_eq!(Grevlex::cmp(&a, &c), Ordering::Greater);

        let d = M3::from_exponents([2, 1, 0]);
        assert_eq!(Grevlex::cmp(&b, &d), Ordering::Equal);
    }

    // --- dynamic monomials ---

    #[test]
    fn lex_order_works_for_dynamic() {
        let a = DM::from_slice(&[1, 5]);
        let b = DM::from_slice(&[2, 0]);

        assert_eq!(Lex::cmp(&a, &b), Ordering::Less);
        assert_eq!(Lex::cmp(&b, &a), Ordering::Greater);

        let c = DM::from_slice(&[3, 1]);
        let d = DM::from_slice(&[3, 4]);
        assert_eq!(Lex::cmp(&c, &d), Ordering::Less);
        assert_eq!(Lex::cmp(&d, &c), Ordering::Greater);
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
}
