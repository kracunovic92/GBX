//! Public concrete polynomial types.
//!
//! The crate exposes two main polynomial flavors:
//!
//! - [`FixedPolynomial`]: compile-time arity via `const N: usize`
//! - [`DynamicPolynomial`]: runtime arity (stored in each monomial/term)
//!
//! Both are thin type aliases over the generic engine [`Polynomial`]. The engine is
//! parameterized by:
//! - term type (`FixedTerm` / `DynamicTerm`)
//! - monomial order (`Lex`, `Grevlex`, or custom implementing [`MonomialOrder`])
//! - storage backend (defaults to [`VecTerms`])
//!
//! # Examples
//!
//! ```
//! use gbx_poly::monomial::Lex;
//! use gbx_poly::polynomial::{FixedPolynomial, PolynomialMut, PolynomialView};
//! use gbx_poly::term::FixedTerm;
//! use gbx_field::fp::Fp;
//!
//! type F = Fp<7>;
//! type P = FixedPolynomial<F, Lex, 2>;
//!
//! // Construct from raw terms (normalizes internally):
//! let t1 = FixedTerm::<F, 2>::from_coeff_and_exponents(F::new(3), [1, 0]);
//! let t2 = FixedTerm::<F, 2>::from_coeff_and_exponents(F::new(5), [1, 0]);
//! let p = <P as PolynomialMut>::from_terms(vec![t1, t2]);
//!
//! assert_eq!(p.terms().len(), 1); // merged
//! assert_eq!(*p.leading_coefficient().unwrap(), F::new(1)); // 3+5 = 8 ≡ 1 mod 7
//! ```
//!
//! Note: `PolynomialMut` constructors are trait associated functions, so you either
//! call them via UFCS as above or use inherent wrappers you add on the engine.

use gbx_storage::polynomial::VecTerms;

use crate::polynomial::poly::Polynomial;
use crate::term::{DynamicTerm, FixedTerm};

/// Runtime-arity sparse polynomial over a field `F` with monomial order `O`.
///
/// Storage backend defaults to [`VecTerms`].
pub type DynamicPolynomial<F, O, S = VecTerms<DynamicTerm<F>>> = Polynomial<DynamicTerm<F>, O, S>;

/// Compile-time-arity sparse polynomial over a field `F` with monomial order `O`
/// and arity `N`.
///
/// Storage backend defaults to [`VecTerms`].
pub type FixedPolynomial<F, O, const N: usize, S = VecTerms<FixedTerm<F, N>>> = Polynomial<FixedTerm<F, N>, O, S>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::Lex;
    use crate::polynomial::traits::{PolynomialMut, PolynomialView};
    use gbx_field::fp::Fp;

    type F7 = Fp<7>;

    #[test]
    fn fixed_alias_compiles_and_constructs_zero() {
        type P = FixedPolynomial<F7, Lex, 2>;
        let p = <P as PolynomialMut>::zero();
        assert!(p.is_zero());
    }

    #[test]
    fn dynamic_alias_compiles_and_constructs_zero() {
        type P = DynamicPolynomial<F7, Lex>;
        let p = <P as PolynomialMut>::zero();
        assert!(p.is_zero());
    }
}
