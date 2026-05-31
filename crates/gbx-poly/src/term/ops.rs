//! Term arithmetic helpers.

use core::ops::Mul;

use crate::monomial::Monomial;
use crate::term::Term;
use crate::term::error::{TermError, TermResult};

/// Multiplies a term by a scalar coefficient.
#[inline]
pub fn mul_scalar<C>(t: &Term<C>, c: &C) -> Term<C>
where
    C: Clone + Mul<Output = C>,
{
    Term::new(t.coeff().clone() * c.clone(), t.mono().clone())
}

/// Multiplies a term by a monomial.
///
/// # Errors
///
/// Returns [`TermError::Monomial`] when checked monomial multiplication fails.
#[inline]
pub fn checked_mul_monomial<C>(t: &Term<C>, m: &Monomial) -> TermResult<Term<C>>
where
    C: Clone,
{
    let mono = t.mono().checked_mul(m).map_err(TermError::from)?;

    Ok(Term::new(t.coeff().clone(), mono))
}

/// Multiplies two terms.
///
/// Coefficients are multiplied using `C::mul`.
/// Monomials are multiplied using checked monomial multiplication.
///
/// # Errors
///
/// Returns [`TermError::Monomial`] when checked monomial multiplication fails.
#[inline]
pub fn checked_mul_term<C>(a: &Term<C>, b: &Term<C>) -> TermResult<Term<C>>
where
    C: Clone + Mul<Output = C>,
{
    let coeff = a.coeff().clone() * b.coeff().clone();
    let mono = a.mono().checked_mul(b.mono()).map_err(TermError::from)?;

    Ok(Term::new(coeff, mono))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::MonomialView;

    #[test]
    fn mul_scalar_works() {
        let t = Term::new(3u32, Monomial::from_slice(&[1, 0, 2]));

        let s = mul_scalar(&t, &5);

        assert_eq!(*s.coeff(), 15);
        assert_eq!(s.mono(), t.mono());
    }

    #[test]
    fn checked_mul_monomial_works() {
        let t = Term::new(3u32, Monomial::from_slice(&[1, 0, 2]));
        let m = Monomial::from_slice(&[0, 2, 1]);

        let Ok(tm) = checked_mul_monomial(&t, &m) else {
            panic!("matching arity term/monomial multiplication should succeed");
        };

        assert_eq!(*tm.coeff(), 3);
        assert_eq!(tm.mono().exponents(), &[1, 2, 3]);
    }

    #[test]
    fn checked_mul_term_works() {
        let a = Term::new(2u32, Monomial::from_slice(&[1, 0, 2]));
        let b = Term::new(7u32, Monomial::from_slice(&[0, 3, 1]));

        let Ok(c) = checked_mul_term(&a, &b) else {
            panic!("matching arity term multiplication should succeed");
        };

        assert_eq!(*c.coeff(), 14);
        assert_eq!(c.mono().exponents(), &[1, 3, 3]);
    }

    #[test]
    fn checked_mul_monomial_errors_propagate() {
        let t = Term::new(1u32, Monomial::from_slice(&[1, 2, 3]));
        let m = Monomial::from_slice(&[1, 2]);

        assert!(matches!(
            checked_mul_monomial(&t, &m),
            Err(TermError::Monomial(_))
        ));
    }
}
