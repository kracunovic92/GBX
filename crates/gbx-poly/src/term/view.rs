//! Read-only view of term
//!
//! This trait allows algorithms to accept both the owned [`crate::term::Term`]
//! and any adapter/wrapper type that exposes coefficient + monomial by reference.

use crate::monomial::MonomialView;

/// Read-only view of a term `c * x^α`.
///
/// This trait allows algorithms to accept both the owned [`crate::term::Term`]
/// and any adapter/wrapper type that exposes coefficient + monomial by reference.
///
/// # Contract
/// - `mono()` must return a monomial view whose `exponents().len()` is its arity.
/// - `coeff()` and `mono()` must remain consistent for the lifetime of `&self`.
pub trait TermView {
    /// Coefficient type of the term.
    type Coeff;
    /// Monomial type of the term.
    type Mono: MonomialView;

    /// Returns a reference to the coefficient.
    ///
    /// This must not allocate or clone.
    fn coeff(&self) -> &Self::Coeff;

    /// Returns a reference to the monomial.
    ///
    /// The returned monomial must remain valid
    /// for the lifetime of `&self`.
    fn mono(&self) -> &Self::Mono;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::FixedMonomial;
    use crate::term::Term;

    #[derive(Clone)]
    struct Wrapped<C, M>(C, M);

    impl<C, M> TermView for Wrapped<C, M>
    where
        M: MonomialView,
    {
        type Coeff = C;
        type Mono = M;

        fn coeff(&self) -> &C {
            &self.0
        }
        fn mono(&self) -> &M {
            &self.1
        }
    }

    #[test]
    fn termview_works_for_wrapper() {
        let w = Wrapped(7u32, FixedMonomial::<2>::from_exponents([1, 0]));
        assert_eq!(*w.coeff(), 7);
        assert_eq!(w.mono().exponents(), &[1, 0]);
    }

    #[test]
    fn term_owned_implements_termview() {
        let t = Term::new(3u32, FixedMonomial::<2>::from_exponents([0, 2]));
        assert_eq!(*t.coeff(), 3);
        assert_eq!(t.mono().exponents(), &[0, 2]);
    }
}
