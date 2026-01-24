//! Monomial algorithms (divisibility, gcd/lcm, quotient).
//!
//! These routines are **storage-agnostic**: they operate only on the `MonomialView` / `Monomial`
//! traits, so they work for both fixed and dynamic monomials.
//!
//! Design:
//! - Read-only predicates use [`MonomialView`].
//! - Constructors return the crate-level [`MonomialError`] so callers get a consistent error model.
//! - `checked_quotient` returns `Ok(None)` when not divisible; `Err(_)` only for real errors.

use crate::monomial::{Monomial, MonomialError, MonomialView};

/// Returns `true` iff `a` divides `b` componentwise (i.e., `a_i <= b_i` for all i).
///
/// If variable counts differ, returns `false`.
#[inline]
pub fn divides<A: MonomialView, B: MonomialView>(a: &A, b: &B) -> bool {
    if a.n_vars() != b.n_vars() {
        return false;
    }
    a.exponents()
        .iter()
        .zip(b.exponents().iter())
        .all(|(&ea, &eb)| ea <= eb)
}

/// Computes `lcm(a, b)` componentwise (`max` per exponent).
///
/// # Errors
/// - [`MonomialError::MismatchedVariableCount`] if `a` and `b` have different arities.
/// - Any error produced by `M::try_from_exponents_iter` (converted into [`MonomialError`]).
///
/// # Type parameters
/// `MonomialError: From<M::Error>` lets `?` convert implementation-specific errors.
#[inline]
pub fn checked_lcm<M>(a: &M, b: &M) -> Result<M, MonomialError>
where
    M: Monomial,
    MonomialError: From<M::Error>,
{
    let n = a.n_vars();
    let m = b.n_vars();
    if n != m {
        return Err(MonomialError::MismatchedVariableCount { lhs: n, rhs: m });
    }

    let exps = a
        .exponents()
        .iter()
        .zip(b.exponents().iter())
        .map(|(&x, &y)| x.max(y));

    Ok(M::try_from_exponents_iter(n, exps)?)
}

/// Computes `gcd(a, b)` componentwise (`min` per exponent).
///
/// # Errors
/// - [`MonomialError::MismatchedVariableCount`] if `a` and `b` have different arities.
/// - Any error produced by `M::try_from_exponents_iter` (converted into [`MonomialError`]).
#[inline]
pub fn checked_gcd<M>(a: &M, b: &M) -> Result<M, MonomialError>
where
    M: Monomial,
    MonomialError: From<M::Error>,
{
    let n = a.n_vars();
    let m = b.n_vars();
    if n != m {
        return Err(MonomialError::MismatchedVariableCount { lhs: n, rhs: m });
    }

    let exps = a
        .exponents()
        .iter()
        .zip(b.exponents().iter())
        .map(|(&x, &y)| x.min(y));

    Ok(M::try_from_exponents_iter(n, exps)?)
}

/// Computes the quotient `dividend / divisor` if `divisor` divides `dividend`.
///
/// Returns:
/// - `Ok(Some(q))` if divisible (where `q * divisor = dividend` exponentwise)
/// - `Ok(None)` if not divisible
/// - `Err(_)` if arity mismatched or monomial construction fails
///
/// # Errors
/// - [`MonomialError::MismatchedVariableCount`] if variable counts differ.
/// - Any error produced by `M::try_from_exponents_iter` (converted into [`MonomialError`]).
#[inline]
pub fn checked_quotient<M>(divisor: &M, dividend: &M) -> Result<Option<M>, MonomialError>
where
    M: Monomial,
    MonomialError: From<M::Error>,
{
    let n = divisor.n_vars();
    let m = dividend.n_vars();
    if n != m {
        return Err(MonomialError::MismatchedVariableCount { lhs: n, rhs: m });
    }
    if !divides(divisor, dividend) {
        return Ok(None);
    }

    let exps = divisor
        .exponents()
        .iter()
        .zip(dividend.exponents().iter())
        .map(|(&a, &b)| b - a);

    Ok(Some(M::try_from_exponents_iter(n, exps)?))
}

/// Convenience extension trait: method-style access to monomial algorithms.
///
/// Note: methods return the crate-level [`MonomialError`] for a consistent API.
pub trait MonomialAlgos: Monomial {
    /// `self | other` (componentwise).
    #[inline]
    fn divides(&self, other: &Self) -> bool {
        divides(self, other)
    }

    /// `lcm(self, other)`.
    #[inline]
    fn checked_lcm(&self, other: &Self) -> Result<Self, MonomialError>
    where
        MonomialError: From<Self::Error>,
    {
        checked_lcm(self, other)
    }

    /// `gcd(self, other)`.
    #[inline]
    fn checked_gcd(&self, other: &Self) -> Result<Self, MonomialError>
    where
        MonomialError: From<Self::Error>,
    {
        checked_gcd(self, other)
    }

    /// `dividend / self` if divisible.
    ///
    /// Returns `Ok(None)` if not divisible.
    #[inline]
    fn checked_quotient_of(&self, dividend: &Self) -> Result<Option<Self>, MonomialError>
    where
        MonomialError: From<Self::Error>,
    {
        checked_quotient(self, dividend)
    }

    /// `self / divisor` if divisible.
    ///
    /// Returns `Ok(None)` if not divisible.
    #[inline]
    fn checked_div_by(&self, divisor: &Self) -> Result<Option<Self>, MonomialError>
    where
        MonomialError: From<Self::Error>,
    {
        checked_quotient(divisor, self)
    }
}

impl<M: Monomial> MonomialAlgos for M {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{DynamicMonomial, FixedMonomial};

    #[test]
    fn divides_fixed() {
        let a = FixedMonomial::<3>::from_exponents([1, 0, 2]);
        let b = FixedMonomial::<3>::from_exponents([1, 5, 2]);
        let c = FixedMonomial::<3>::from_exponents([0, 5, 3]);

        assert!(divides(&a, &b));
        assert!(!divides(&b, &a));
        assert!(!divides(&c, &b));
    }

    #[test]
    fn lcm_gcd_fixed() {
        let a = FixedMonomial::<3>::from_exponents([1, 0, 2]);
        let b = FixedMonomial::<3>::from_exponents([3, 4, 1]);

        let l = checked_lcm(&a, &b).unwrap();
        let g = checked_gcd(&a, &b).unwrap();

        assert_eq!(l.exponents(), &[3, 4, 2]);
        assert_eq!(g.exponents(), &[1, 0, 1]);
    }

    #[test]
    fn quotient_fixed() {
        let d = FixedMonomial::<3>::from_exponents([1, 2, 0]); // divisor
        let n = FixedMonomial::<3>::from_exponents([4, 2, 3]); // dividend

        let q = checked_quotient(&d, &n).unwrap().unwrap();
        assert_eq!(q.exponents(), &[3, 0, 3]);

        let not_div = FixedMonomial::<3>::from_exponents([5, 0, 0]);
        assert!(checked_quotient(&not_div, &n).unwrap().is_none());
    }

    #[test]
    fn works_for_dynamic() {
        let a = DynamicMonomial::from_slice(&[1, 0, 2]);
        let b = DynamicMonomial::from_slice(&[3, 4, 1]);

        let l = checked_lcm(&a, &b).unwrap();
        assert_eq!(l.exponents(), &[3, 4, 2]);
    }

    #[test]
    fn extension_trait_methods_work() {
        let a = FixedMonomial::<2>::from_exponents([1, 0]);
        let b = FixedMonomial::<2>::from_exponents([2, 3]);

        assert!(a.divides(&b));

        let l = a.checked_lcm(&b).unwrap();
        assert_eq!(l.exponents(), &[2, 3]);

        let q = a.checked_quotient_of(&b).unwrap().unwrap();
        assert_eq!(q.exponents(), &[1, 3]);
    }
}
