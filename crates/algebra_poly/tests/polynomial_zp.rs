#![allow(missing_docs)]

use algebra_core::One;
use algebra_field::Zp;
use algebra_poly::monomial::MonomialOrder;
use algebra_poly::monomial::{DynamicMonomial, Grevlex, Lex, Monomial};
use algebra_poly::polynomial::{DynamicPolynomial, Polynomial};
use algebra_poly::term::{DynamicTerm, Term};

type F7 = Zp<7>;
type P2Lex = Polynomial<F7, 2, Lex>;
type P2Grev = Polynomial<F7, 2, Grevlex>;

#[test]
fn fixed_polynomial_zero_and_degree() {
    let p: P2Lex = Polynomial::zero();
    assert!(p.is_zero());
    assert!(
        p.lt()
            .is_none()
    );
    assert_eq!(p.degree(), None);
}

#[test]
fn fixed_polynomial_addition_matches_expectation() {
    let m1 = Monomial::<2>::from_exponents([1, 0]);
    let m2 = Monomial::<2>::from_exponents([0, 1]);

    let p1: P2Lex = P2Lex::from_terms(vec![Term::new(F7::one(), m1), Term::new(F7::one(), m2)]);

    let p2: P2Lex = P2Lex::from_terms(vec![Term::new(F7::one(), m1), Term::new(-F7::one(), m2)]);

    let sum = p1.add_ref(&p2);

    assert!(!sum.is_zero());
    assert_eq!(
        sum.terms()
            .len(),
        1
    );

    let t = &sum.terms()[0];

    assert_eq!(t.mono, m1);
    assert_eq!(t.coeff, F7::one() + F7::one());
}

#[test]
fn dynamic_polynomial_basic_addition() {
    let m1 = DynamicMonomial::from_slice(&[1, 0]);
    let m2 = DynamicMonomial::from_slice(&[0, 1]);

    type DPlex = DynamicPolynomial<F7, Lex>;

    let p1: DPlex = DynamicPolynomial::from_terms(vec![
        DynamicTerm::new(F7::one(), m1.clone()),
        DynamicTerm::new(F7::one(), m2.clone()),
    ]);

    let p2: DPlex = DynamicPolynomial::from_terms(vec![
        DynamicTerm::new(F7::one(), m1.clone()),
        DynamicTerm::new(-F7::one(), m2.clone()),
    ]);

    let sum = p1.add_ref(&p2);

    assert!(!sum.is_zero());
    assert_eq!(
        sum.terms()
            .len(),
        1
    );
    let t = &sum.terms()[0];
    assert_eq!(t.mono, m1);
    assert_eq!(t.coeff, F7::one() + F7::one());
}

#[test]
fn lex_and_grevlex_disagree_on_ordering() {
    let a = Monomial::<3>::from_exponents([3, 0, 0]);
    let b = Monomial::<3>::from_exponents([2, 1, 0]);

    assert!(matches!(Lex::cmp(&a, &b), core::cmp::Ordering::Greater));
    assert!(matches!(Grevlex::cmp(&a, &b), core::cmp::Ordering::Greater));
}
