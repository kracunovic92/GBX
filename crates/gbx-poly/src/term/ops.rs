//! Term arithmetic as free functions.
//!
//! This keeps the core `Term` representation lightweight and makes it easier
//! to provide specialized implementations later (SIMD, packed exponents, etc.).

use core::ops::Mul;

use crate::monomial::Monomial;
use crate::term::error::{Result, TermError};
use crate::term::TermOwned;

/// Multiply a term by a scalar (coefficient only).
///
/// Returns a term of the **same concrete type** `T`.
#[inline]
pub fn mul_scalar<T>(t: &T, c: &T::Coeff) -> T
where
    T: TermOwned,
    T::Coeff: Clone + Mul<Output = T::Coeff>,
    T::Mono: Clone,
{
    T::from_parts(t.coeff().clone() * c.clone(), t.mono().clone())
}

/// Multiply a term by a monomial (monomial only), checked.
///
/// Returns a term of the **same concrete type** `T`.
///
/// # Errors
/// Returns [`TermError::Monomial`] if monomial multiplication fails.
#[inline]
pub fn checked_mul_monomial<T>(t: &T, m: &T::Mono) -> Result<T>
where
    T: TermOwned,
    T::Coeff: Clone,
    T::Mono: Monomial + Clone,
{
    let mono = t.mono().clone().checked_mul(m).map_err(TermError::from)?;
    Ok(T::from_parts(t.coeff().clone(), mono))
}

/// Multiply two terms, checked.
///
/// Returns a term of the **same concrete type** `T`.
///
/// # Errors
/// Returns [`TermError::Monomial`] if monomial multiplication fails.
#[inline]
pub fn checked_mul_term<T>(a: &T, b: &T) -> Result<T>
where
    T: TermOwned,
    T::Coeff: Clone + Mul<Output = T::Coeff>,
    T::Mono: Monomial + Clone,
{
    let coeff = a.coeff().clone() * b.coeff().clone();
    let mono = a
        .mono()
        .clone()
        .checked_mul(b.mono())
        .map_err(TermError::from)?;
    Ok(T::from_parts(coeff, mono))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::monomial::{DynamicMonomial, FixedMonomial};
    use crate::term::repr::Term;

    type StaticTerm = Term<u32, FixedMonomial<3>>;
    type DynVarsTerm = Term<u32, DynamicMonomial>;

    #[test]
    fn static_term_mul_scalar_and_mul_monomial() {
        let t: StaticTerm = Term::new(3, FixedMonomial::from_exponents([1, 0, 2]));

        let s = mul_scalar(&t, &5);
        assert_eq!(*s.coeff_ref(), 15);
        assert_eq!(s.mono_ref(), t.mono_ref());

        let m = FixedMonomial::<3>::from_exponents([0, 2, 1]);
        let tm = checked_mul_monomial(&t, &m).unwrap();
        assert_eq!(*tm.coeff_ref(), 3);
        assert_eq!(*tm.mono_ref(), FixedMonomial::from_exponents([1, 2, 3]));
    }

    #[test]
    fn dynamic_vars_term_mul_term_ok() {
        let a: DynVarsTerm = Term::new(2, DynamicMonomial::from_slice(&[1, 0, 2]));
        let b: DynVarsTerm = Term::new(7, DynamicMonomial::from_slice(&[0, 3, 1]));

        let c = checked_mul_term(&a, &b).unwrap();
        assert_eq!(*c.coeff_ref(), 14);
        assert_eq!(*c.mono_ref(), DynamicMonomial::from_slice(&[1, 3, 3]));
    }

    #[test]
    fn dynamic_vars_mul_monomial_errors_propagate() {
        let t: DynVarsTerm = Term::new(1, DynamicMonomial::from_slice(&[1, 2, 3]));
        let m = DynamicMonomial::from_slice(&[1, 2]); // arity mismatch

        let err = checked_mul_monomial(&t, &m).unwrap_err();
        assert!(matches!(err, TermError::Monomial(_)));
    }
}
