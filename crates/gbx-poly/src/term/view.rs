//! Read-only term view.

use crate::monomial::Monomial;

/// Read-only view of a term `c * x^α`.
///
/// This trait allows algorithms to accept owned terms or lightweight wrappers
/// that expose coefficient and monomial by reference.
pub trait TermView {
    /// Coefficient type.
    type Coeff;

    /// Returns a reference to the coefficient.
    fn coeff(&self) -> &Self::Coeff;

    /// Returns a reference to the monomial.
    fn mono(&self) -> &Monomial;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::{Monomial, MonomialView};
    use crate::term::Term;

    #[derive(Clone)]
    struct Wrapped<C> {
        coeff: C,
        mono: Monomial,
    }

    impl<C> TermView for Wrapped<C> {
        type Coeff = C;

        fn coeff(&self) -> &C {
            &self.coeff
        }

        fn mono(&self) -> &Monomial {
            &self.mono
        }
    }

    #[test]
    fn termview_works_for_wrapper() {
        let w = Wrapped { coeff: 7u32, mono: Monomial::from_slice(&[1, 0]) };

        assert_eq!(*w.coeff(), 7);
        assert_eq!(w.mono().exponents(), &[1, 0]);
    }

    #[test]
    fn term_owned_implements_termview() {
        let t = Term::new(3u32, Monomial::from_slice(&[0, 2]));

        assert_eq!(*t.coeff(), 3);
        assert_eq!(t.mono().exponents(), &[0, 2]);
    }
}
