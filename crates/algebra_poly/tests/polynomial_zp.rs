#![allow(missing_docs)]

use algebra_core::{Additive, One};
use algebra_field::Zp;

use algebra_poly::monomial::{DynamicMonomial, Grevlex, Lex, Monomial, MonomialOrder};
use algebra_poly::polynomial::{DynamicPolynomial, Polynomial, PolynomialLike, PolynomialMut};
use algebra_poly::term::{DynamicTerm, Term};

type F7 = Zp<7>;
type P2Lex = Polynomial<F7, 2, Lex>;

#[test]
fn fixed_polynomial_zero_and_degree() {
    let p: P2Lex = P2Lex::zero();

    assert!(p.is_zero());

    // Use the trait API (your refactor point):
    assert!(
        p.leading_term()
            .is_none()
    );

    // `degree()` is a default method on PolynomialLike → trait must be in scope (it is).
    assert_eq!(p.degree(), None);
}

#[test]
fn fixed_polynomial_addition_matches_expectation() {
    let m1 = Monomial::<2>::from_exponents([1, 0]);
    let m2 = Monomial::<2>::from_exponents([0, 1]);

    let p1: P2Lex = P2Lex::from_terms(vec![Term::new(F7::one(), m1), Term::new(F7::one(), m2)]);
    let p2: P2Lex = P2Lex::from_terms(vec![Term::new(F7::one(), m1), Term::new(-F7::one(), m2)]);

    // Use the new generic helper (no `add_ref` assumption):
    let sum = P2Lex::add_poly(&p1, &p2);

    assert!(!sum.is_zero());
    assert_eq!(
        sum.terms()
            .len(),
        1
    );

    let t = &sum.terms()[0];
    assert_eq!(t.mono, m1);

    // No `+` operator on fields; use Additive:
    assert_eq!(t.coeff, F7::one().add(F7::one()));
}

#[test]
fn dynamic_polynomial_basic_addition() {
    let m1 = DynamicMonomial::from_slice(&[1, 0]);
    let m2 = DynamicMonomial::from_slice(&[0, 1]);

    type DPlex = DynamicPolynomial<F7, Lex>;

    let p1: DPlex = DPlex::from_terms(vec![
        DynamicTerm::new(F7::one(), m1.clone()),
        DynamicTerm::new(F7::one(), m2.clone()),
    ]);

    let p2: DPlex = DPlex::from_terms(vec![
        DynamicTerm::new(F7::one(), m1.clone()),
        DynamicTerm::new(-F7::one(), m2.clone()),
    ]);

    let sum = DPlex::add_poly(&p1, &p2);

    assert!(!sum.is_zero());
    assert_eq!(
        sum.terms()
            .len(),
        1
    );

    let t = &sum.terms()[0];
    assert_eq!(t.mono, m1);
    assert_eq!(t.coeff, F7::one().add(F7::one()));
}

#[test]
fn lex_and_grevlex_can_disagree_on_ordering() {
    // Degree differs, so Grevlex compares by degree first; Lex compares from first exponent.
    let a = Monomial::<2>::from_exponents([1, 0]); // x0
    let b = Monomial::<2>::from_exponents([0, 2]); // x1^2

    assert_eq!(Lex::cmp(&a, &b), core::cmp::Ordering::Greater);
    assert_eq!(Grevlex::cmp(&a, &b), core::cmp::Ordering::Less);
}
