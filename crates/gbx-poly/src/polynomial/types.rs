//! Public polynomial type aliases.
//!
//! These aliases are convenience wrappers around the generic
//! [`Polynomial`](crate::polynomial::Polynomial) container.
//!
//! Monomial order is *not* part of the polynomial type; it is supplied by the ring context.

use gbx_storage::polynomial::VecTerms;

use crate::monomial::{DynamicMonomial, FixedMonomial};
use crate::polynomial::poly::Polynomial;
use crate::term::Term;

/// Fully dynamic: runtime coefficient element + runtime arity monomial.
pub type PolyDyn<C, S = VecTerms<Term<C, DynamicMonomial>>> = Polynomial<Term<C, DynamicMonomial>, S>;

/// Dynamic coeff element + fixed arity.
pub type PolyDynFieldFixedVars<C, const N: usize, S = VecTerms<Term<C, FixedMonomial<N>>>> = Polynomial<Term<C, FixedMonomial<N>>, S>;

/// Static coeff element (or any `C`) + runtime arity monomial.
pub type PolyFixedFieldDynVars<C, S = VecTerms<Term<C, DynamicMonomial>>> = Polynomial<Term<C, DynamicMonomial>, S>;

/// Fully fixed: coeff element + fixed arity.
pub type PolyFixed<C, const N: usize, S = VecTerms<Term<C, FixedMonomial<N>>>> = Polynomial<Term<C, FixedMonomial<N>>, S>;

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::monomial::FixedMonomial;
    use crate::order::Lex;
    use crate::polynomial::traits::PolynomialMut;
    use crate::ring::{Ring, StaticFpCtx};
    use crate::term::Term;
    use gbx_field::fp::Fp;

    #[test]
    fn aliases_compile_and_construct() {
        type F7 = Fp<7>;

        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        type P = PolyFixed<F7, 2>;

        let _z = P::zero_in(&ring);

        let _p = P::from_terms_in(
            &ring,
            vec![Term::new(F7::new(1), FixedMonomial::<2>::from_exponents([1, 0]))],
        )
        .unwrap();
    }
}
