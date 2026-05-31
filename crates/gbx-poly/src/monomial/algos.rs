//! Monomial algorithms.

use crate::monomial::error::{MonomialError, MonomialResult};
use crate::monomial::{Monomial, MonomialView};

/// Returns `true` iff `a` divides `b` component-wise.
#[inline]
#[must_use]
pub fn divides(a: &Monomial, b: &Monomial) -> bool {
    if a.n_vars() != b.n_vars() {
        return false;
    }

    a.exponents()
        .iter()
        .zip(b.exponents().iter())
        .all(|(&ea, &eb)| ea <= eb)
}

/// Computes `lcm(a, b)` component-wise.
///
/// # Errors
///
/// Returns [`MonomialError::MismatchedArity`] when the monomials have different
/// variable counts, or [`MonomialError::DegreeOverflow`] when the lcm degree
/// overflows `u32`.
#[inline]
pub fn checked_lcm(a: &Monomial, b: &Monomial) -> MonomialResult<Monomial> {
    let n = a.n_vars();

    if n != b.n_vars() {
        return Err(MonomialError::MismatchedArity { lhs: n, rhs: b.n_vars() });
    }

    let exps = a
        .exponents()
        .iter()
        .zip(b.exponents().iter())
        .map(|(&x, &y)| x.max(y));

    Monomial::try_from_exponents_iter(n, exps)
}

/// Computes `deg(lcm(a, b))` without constructing the lcm monomial.
///
/// # Errors
///
/// Returns [`MonomialError::MismatchedArity`] when the monomials have different
/// variable counts, or [`MonomialError::DegreeOverflow`] when the lcm degree
/// overflows `u32`.
#[inline]
pub fn checked_lcm_degree(a: &Monomial, b: &Monomial) -> MonomialResult<u32> {
    let n = a.n_vars();

    if n != b.n_vars() {
        return Err(MonomialError::MismatchedArity { lhs: n, rhs: b.n_vars() });
    }

    let mut degree = 0u32;

    for (&x, &y) in a.exponents().iter().zip(b.exponents().iter()) {
        degree = degree
            .checked_add(x.max(y))
            .ok_or(MonomialError::DegreeOverflow)?;
    }

    Ok(degree)
}

/// Computes `gcd(a, b)` component-wise.
///
/// # Errors
///
/// Returns [`MonomialError::MismatchedArity`] when the monomials have different
/// variable counts.
#[inline]
pub fn checked_gcd(a: &Monomial, b: &Monomial) -> MonomialResult<Monomial> {
    let n = a.n_vars();

    if n != b.n_vars() {
        return Err(MonomialError::MismatchedArity { lhs: n, rhs: b.n_vars() });
    }

    let exps = a
        .exponents()
        .iter()
        .zip(b.exponents().iter())
        .map(|(&x, &y)| x.min(y));

    Monomial::try_from_exponents_iter(n, exps)
}

/// Returns `true` iff `gcd(a, b) == 1`.
#[inline]
#[must_use]
pub fn gcd_is_one(a: &Monomial, b: &Monomial) -> bool {
    debug_assert_eq!(a.n_vars(), b.n_vars());

    a.exponents()
        .iter()
        .zip(b.exponents().iter())
        .all(|(&x, &y)| x == 0 || y == 0)
}

/// Computes `dividend / divisor` if divisible.
///
/// # Errors
///
/// Returns [`MonomialError::MismatchedArity`] when the monomials have different
/// variable counts, or [`MonomialError::DegreeOverflow`] if the quotient degree
/// overflows `u32`.
#[inline]
pub fn checked_quotient(divisor: &Monomial, dividend: &Monomial) -> MonomialResult<Option<Monomial>> {
    let n = divisor.n_vars();

    if n != dividend.n_vars() {
        return Err(MonomialError::MismatchedArity { lhs: n, rhs: dividend.n_vars() });
    }

    if !divides(divisor, dividend) {
        return Ok(None);
    }

    let exps = divisor
        .exponents()
        .iter()
        .zip(dividend.exponents().iter())
        .map(|(&a, &b)| b - a);

    Ok(Some(Monomial::try_from_exponents_iter(n, exps)?))
}

/// Computes exact `dividend / divisor`.
///
/// # Errors
///
/// Returns [`MonomialError::NotDivisible`] when `divisor` does not divide
/// `dividend`. Also returns the errors from [`checked_quotient`].
#[inline]
pub fn checked_div_exact(divisor: &Monomial, dividend: &Monomial) -> MonomialResult<Monomial> {
    checked_quotient(divisor, dividend)?.ok_or(MonomialError::NotDivisible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_works() {
        let a = Monomial::from_slice(&[1, 0, 2]);
        let b = Monomial::from_slice(&[1, 5, 2]);
        let c = Monomial::from_slice(&[0, 5, 3]);

        assert!(divides(&a, &b));
        assert!(!divides(&b, &a));
        assert!(!divides(&c, &b));
    }

    #[test]
    fn lcm_gcd_work() {
        let a = Monomial::from_slice(&[1, 0, 2]);
        let b = Monomial::from_slice(&[3, 4, 1]);

        let Ok(l) = checked_lcm(&a, &b) else {
            panic!("lcm should succeed for matching arity");
        };
        let Ok(g) = checked_gcd(&a, &b) else {
            panic!("gcd should succeed for matching arity");
        };

        assert_eq!(l.exponents(), &[3, 4, 2]);
        assert_eq!(g.exponents(), &[1, 0, 1]);
    }

    #[test]
    fn quotient_works() {
        let divisor = Monomial::from_slice(&[1, 2, 0]);
        let dividend = Monomial::from_slice(&[4, 2, 3]);

        let Ok(Some(q)) = checked_quotient(&divisor, &dividend) else {
            panic!("quotient should exist for divisible monomials");
        };

        assert_eq!(q.exponents(), &[3, 0, 3]);

        let not_divisor = Monomial::from_slice(&[5, 0, 0]);
        assert!(matches!(
            checked_quotient(&not_divisor, &dividend),
            Ok(None)
        ));
    }

    #[test]
    fn exact_div_errors_when_not_divisible() {
        let divisor = Monomial::from_slice(&[2, 0]);
        let dividend = Monomial::from_slice(&[1, 0]);

        assert!(matches!(
            checked_div_exact(&divisor, &dividend),
            Err(MonomialError::NotDivisible)
        ));
    }
}
