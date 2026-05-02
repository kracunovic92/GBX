//! Owned term representation.

use core::fmt;

use crate::monomial::Monomial;
use crate::term::view::TermView;

/// Owned term `c * x^α`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Term<C> {
    coeff: C,
    mono: Monomial,
}

impl<C> Term<C> {
    /// Constructs a term from coefficient and monomial.
    #[inline]
    pub fn new(coeff: C, mono: Monomial) -> Self {
        Self { coeff, mono }
    }

    /// Borrows the coefficient.
    #[inline]
    pub fn coeff(&self) -> &C {
        &self.coeff
    }

    /// Borrows the monomial.
    #[inline]
    pub fn mono(&self) -> &Monomial {
        &self.mono
    }

    /// Mutable coefficient access.
    #[inline]
    pub fn coeff_mut(&mut self) -> &mut C {
        &mut self.coeff
    }

    /// Mutable monomial access.
    #[inline]
    pub fn mono_mut(&mut self) -> &mut Monomial {
        &mut self.mono
    }

    /// Consumes the term into owned parts.
    #[inline]
    pub fn into_parts(self) -> (C, Monomial) {
        (self.coeff, self.mono)
    }

    /// Rebuilds a term from owned parts.
    #[inline]
    pub fn from_parts(coeff: C, mono: Monomial) -> Self {
        Self::new(coeff, mono)
    }
}

impl<C> TermView for Term<C> {
    type Coeff = C;

    #[inline]
    fn coeff(&self) -> &Self::Coeff {
        &self.coeff
    }

    #[inline]
    fn mono(&self) -> &Monomial {
        &self.mono
    }
}

impl<C> fmt::Debug for Term<C>
where
    C: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Term")
            .field("coeff", &self.coeff)
            .field("mono", &self.mono)
            .finish()
    }
}

impl<C> fmt::Display for Term<C>
where
    C: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mono.is_one() { write!(f, "{}", self.coeff) } else { write!(f, "{}*{}", self.coeff, self.mono) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::MonomialView;

    #[test]
    fn term_accessors_work() {
        let m = Monomial::from_slice(&[1, 0, 2]);
        let t = Term::new(7u32, m.clone());

        assert_eq!(*t.coeff(), 7);
        assert_eq!(t.mono().exponents(), &[1, 0, 2]);

        let (c2, m2) = t.clone().into_parts();

        assert_eq!(c2, 7);
        assert_eq!(m2, m);
    }

    #[test]
    fn display_hides_monomial_one() {
        let one = Monomial::one(3);
        let t = Term::new(5u32, one);

        assert_eq!(t.to_string(), "5");
    }

    #[test]
    fn display_shows_monomial_when_not_one() {
        let m = Monomial::from_slice(&[0, 1, 2]);
        let t = Term::new(5u32, m);

        assert_eq!(t.to_string(), "5*x1*x2^2");
    }
}
