//! Monomial algorithms (divisibility, gcd/lcm, quotient).
//!
//! These routines are **storage-agnostic**: they operate only on the `MonomialView` / `Monomial`
//! traits, so they work for both fixed and dynamic monomials.
//!
//! Note: these algorithms assume exponent words are `u32` (the library default).

use crate::monomial::{Monomial, MonomialError, MonomialView, Result};

/// Returns `true` iff `a` divides `b` componentwise (i.e., `a_i <= b_i` for all i).
///
/// If variable counts differ, returns `false`.
#[inline]
pub fn divides<A, B>(a: &A, b: &B) -> bool
where
    A: MonomialView<Word = u32>,
    B: MonomialView<Word = u32>,
{
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
/// - [`MonomialError::MismatchedArity`] if `a` and `b` have different arities.
/// - Any error produced by `M::try_from_exponents_iter`.
#[inline]
pub fn checked_lcm<M>(a: &M, b: &M) -> Result<M>
where
    M: Monomial<Word = u32>,
{
    let n = a.n_vars();
    let m = b.n_vars();
    if n != m {
        return Err(MonomialError::MismatchedArity { lhs: n, rhs: m });
    }

    let exps = a
        .exponents()
        .iter()
        .zip(b.exponents().iter())
        .map(|(&x, &y)| x.max(y));

    M::try_from_exponents_iter(n, exps)
}

/// Computes `deg(lcm(a,b))` without constructing the LCM monomial.
///
/// # Errors
/// - [`MonomialError::MismatchedArity`] if `a` and `b` have different arities.
#[inline]
pub fn checked_lcm_degree<M>(a: &M, b: &M) -> Result<u32>
where
    M: Monomial<Word = u32>,
{
    let n = a.n_vars();
    let m = b.n_vars();
    if n != m {
        return Err(MonomialError::MismatchedArity { lhs: n, rhs: m });
    }

    let mut deg = 0u32;
    for (&x, &y) in a.exponents().iter().zip(b.exponents().iter()) {
        deg = deg
            .checked_add(x.max(y))
            .ok_or(MonomialError::DegreeOverflow)?;
    }
    Ok(deg)
}

/// Computes `gcd(a, b)` componentwise (`min` per exponent).
///
/// # Errors
/// - [`MonomialError::MismatchedArity`] if `a` and `b` have different arities.
/// - Any error produced by `M::try_from_exponents_iter`.
#[inline]
pub fn checked_gcd<M>(a: &M, b: &M) -> Result<M>
where
    M: Monomial<Word = u32>,
{
    let n = a.n_vars();
    let m = b.n_vars();
    if n != m {
        return Err(MonomialError::MismatchedArity { lhs: n, rhs: m });
    }

    let exps = a
        .exponents()
        .iter()
        .zip(b.exponents().iter())
        .map(|(&x, &y)| x.min(y));

    M::try_from_exponents_iter(n, exps)
}
/// Fast check

#[inline]
pub fn gcd_is_one<M: Monomial<Word = u32>>(a: &M, b: &M) -> bool {
    debug_assert_eq!(a.n_vars(), b.n_vars());
    a.exponents()
        .iter()
        .zip(b.exponents().iter())
        .all(|(&x, &y)| x == 0 || y == 0)
}

/// Computes the quotient `dividend / divisor` if `divisor` divides `dividend`.
///
/// Returns:
/// - `Ok(Some(q))` if divisible (where `q * divisor = dividend` exponentwise)
/// - `Ok(None)` if not divisible
///
/// # Errors
/// - [`MonomialError::MismatchedArity`] if variable counts differ.
/// - Any error produced by `M::try_from_exponents_iter`.
#[inline]
pub fn checked_quotient<M>(divisor: &M, dividend: &M) -> Result<Option<M>>
where
    M: Monomial<Word = u32>,
{
    let n = divisor.n_vars();
    let m = dividend.n_vars();
    if n != m {
        return Err(MonomialError::MismatchedArity { lhs: n, rhs: m });
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

/// Computes the exact quotient `dividend / divisor`, erroring if not divisible.
///
/// # Errors
/// - [`MonomialError::MismatchedArity`] if arities differ.
/// - [`MonomialError::NotDivisible`] if `divisor` does not divide `dividend`.
/// - Any construction error from `M::try_from_exponents_iter`.
#[inline]
pub fn checked_div_exact<M>(divisor: &M, dividend: &M) -> Result<M>
where
    M: Monomial<Word = u32>,
{
    match checked_quotient(divisor, dividend)? {
        Some(q) => Ok(q),
        None => Err(MonomialError::NotDivisible),
    }
}

/// Convenience extension trait: method-style access to monomial algorithms.
///
/// Note: methods return the crate-level [`MonomialError`] for a consistent API.
pub trait MonomialAlgos: Monomial<Word = u32> {
    /// `self | other` (componentwise).
    #[inline]
    fn divides(&self, other: &Self) -> bool {
        divides(self, other)
    }

    /// `lcm(self, other)`.
    #[inline]
    fn checked_lcm(&self, other: &Self) -> Result<Self> {
        checked_lcm(self, other)
    }

    /// `gcd(self, other)`.
    #[inline]
    fn checked_gcd(&self, other: &Self) -> Result<Self> {
        checked_gcd(self, other)
    }

    /// `dividend / self` if divisible.
    ///
    /// Returns `Ok(None)` if not divisible.
    #[inline]
    fn checked_quotient_of(&self, dividend: &Self) -> Result<Option<Self>> {
        checked_quotient(self, dividend)
    }

    /// `self / divisor` if divisible.
    ///
    /// Returns `Ok(None)` if not divisible.
    #[inline]
    fn checked_div_by(&self, divisor: &Self) -> Result<Option<Self>> {
        checked_quotient(divisor, self)
    }

    /// `self / divisor` with an error if not divisible.
    #[inline]
    fn checked_div_exact_by(&self, divisor: &Self) -> Result<Self> {
        checked_div_exact(divisor, self)
    }

    /// `dividend / self` with an error if not divisible.
    #[inline]
    fn checked_exact_quotient_of(&self, dividend: &Self) -> Result<Self> {
        checked_div_exact(self, dividend)
    }
}

impl<M: Monomial<Word = u32>> MonomialAlgos for M {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::{DynamicMonomial, FixedMonomial};

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
