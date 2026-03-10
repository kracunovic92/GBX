//! Canonical owned term representation.
//!
//! This is the type you store inside polynomials/bases.
//! Everything else (fixed/dyn wrappers) can be type aliases.

use core::fmt;

use crate::monomial::{MonomialView, MonomialViewExtU32};
use crate::term::view::TermView;

/// Owned term `c * x^α`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Term<C, M> {
    coeff: C,
    mono: M,
}

impl<C, M> Term<C, M> {
    /// Construct a term from its parts.
    #[inline]
    pub fn new(coeff: C, mono: M) -> Self {
        Self { coeff, mono }
    }

    /// Borrow coefficient.
    #[inline]
    pub fn coeff_ref(&self) -> &C {
        &self.coeff
    }

    /// Borrow monomial.
    #[inline]
    pub fn mono_ref(&self) -> &M {
        &self.mono
    }

    /// Consume into parts.
    #[inline]
    pub fn into_parts(self) -> (C, M) {
        (self.coeff, self.mono)
    }
}

impl<C, M> TermView for Term<C, M>
where
    M: MonomialView,
{
    type Coeff = C;
    type Mono = M;

    #[inline]
    fn coeff(&self) -> &Self::Coeff {
        &self.coeff
    }

    #[inline]
    fn mono(&self) -> &Self::Mono {
        &self.mono
    }
}

impl<C: fmt::Debug, M: fmt::Debug> fmt::Debug for Term<C, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Term")
            .field("coeff", &self.coeff)
            .field("mono", &self.mono)
            .finish()
    }
}

impl<C: fmt::Display, M: MonomialView<Word = u32> + fmt::Display> fmt::Display for Term<C, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Special-case "1" monomial: print just the coefficient.
        if self.mono.is_one() { write!(f, "{}", self.coeff) } else { write!(f, "{}*{}", self.coeff, self.mono) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::{DynamicMonomial, FixedMonomial, MonomialView};

    #[test]
    fn term_accessors_work() {
        let m = DynamicMonomial::from_slice(&[1, 0, 2]);
        let t = Term::new(7u32, m.clone());

        assert_eq!(*t.coeff_ref(), 7);
        assert_eq!(t.mono_ref().exponents(), &[1, 0, 2]);

        let (c2, m2) = t.clone().into_parts();
        assert_eq!(c2, 7);
        assert_eq!(m2, m);
    }

    #[test]
    fn term_works_for_fixed_and_dynamic_monomials() {
        let mf: FixedMonomial<3> = FixedMonomial::from_exponents([1, 2, 0]);
        let md: DynamicMonomial = DynamicMonomial::from_slice(&[1, 2, 0]);

        let _tf = Term::new(1u32, mf);
        let _td = Term::new(1u32, md);
    }

    #[test]
    fn display_hides_monomial_one() {
        let one = FixedMonomial::<3>::one();
        let t = Term::new(5u32, one);
        assert_eq!(t.to_string(), "5");
    }

    #[test]
    fn display_shows_monomial_when_not_one() {
        let m = FixedMonomial::<3>::from_exponents([0, 1, 2]);
        let t = Term::new(5u32, m);
        assert_eq!(t.to_string(), "5*x1*x2^2");
    }
}
