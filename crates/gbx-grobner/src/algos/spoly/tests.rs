use super::{s_polynomial_in, SPolyError};

use gbx_field::fp::{FpDyn, FpDynElem};
use gbx_poly::monomial::DynamicMonomial;
use gbx_poly::order::Lex;
use gbx_poly::polynomial::{Polynomial, PolynomialMut, PolynomialView};
use gbx_poly::ring::RingCtx;
use gbx_poly::term::Term;
use gbx_poly::write_pretty;
use gbx_storage::polynomial::VecTerms;

type Mono = DynamicMonomial;
type Coeff = FpDynElem;
type TestTerm = Term<Coeff, Mono>;
type Poly = Polynomial<TestTerm, VecTerms<TestTerm>>;

fn ring_lex_2() -> RingCtx<FpDyn, Lex> {
    RingCtx::new(FpDyn::prime(32003).unwrap(), Lex, 2).unwrap()
}

fn ring_lex_3() -> RingCtx<FpDyn, Lex> {
    RingCtx::new(FpDyn::prime(32003).unwrap(), Lex, 3).unwrap()
}

fn t(ctx: &RingCtx<FpDyn, Lex>, c: u32, exps: &[u32]) -> Term<Coeff, Mono> {
    Term::new(ctx.field.new(c), DynamicMonomial::from_slice(exps))
}

fn p(ctx: &RingCtx<FpDyn, Lex>, terms: &[(u32, &[u32])]) -> Poly {
    Poly::from_terms_in(
        ctx,
        terms.iter().map(|(c, exps)| t(ctx, *c, exps)).collect(),
    )
    .unwrap()
}

fn pretty(ctx: &RingCtx<FpDyn, Lex>, poly: &Poly) -> String {
    let vars: &[String] = &[];
    let mut out = String::new();
    write_pretty!(&mut out, ctx, poly, vars).unwrap();
    out
}

#[test]
fn spoly_monic_example_matches_expected_after_normalization() {
    let ctx = ring_lex_2();

    let f = p(&ctx, &[(1, &[2, 0]), (1, &[0, 1])]);
    let g = p(&ctx, &[(1, &[1, 1]), (1, &[0, 0])]);

    let mut s = s_polynomial_in(&ctx, &f, &g).unwrap();
    s.normalize_in_place(&ctx).unwrap();

    let expected = p(&ctx, &[(32002, &[1, 0]), (1, &[0, 2])]);

    assert_eq!(pretty(&ctx, &s), pretty(&ctx, &expected));
}

#[test]
fn spoly_non_monic_example_matches_expected_after_normalization() {
    let ctx = ring_lex_2();

    // f = 2x^2 + y
    // g = 3xy + 1
    //
    // S(f,g) = (1/2)*y*f - (1/3)*x*g
    //        = (1/2)y^2 - (1/3)x
    //
    // mod 32003:
    // 1/2 = 16002
    // 1/3 = 10668
    // -(1/3) = 32003 - 10668 = 21335
    let f = p(&ctx, &[(2, &[2, 0]), (1, &[0, 1])]);
    let g = p(&ctx, &[(3, &[1, 1]), (1, &[0, 0])]);

    let mut s = s_polynomial_in(&ctx, &f, &g).unwrap();
    s.normalize_in_place(&ctx).unwrap();

    let expected = p(&ctx, &[(21335, &[1, 0]), (16002, &[0, 2])]);

    assert_eq!(pretty(&ctx, &s), pretty(&ctx, &expected));
}

#[test]
fn spoly_zero_left_input_returns_error() {
    let ctx = ring_lex_2();

    let zero = p(&ctx, &[]);
    let g = p(&ctx, &[(1, &[1, 0])]);

    let err = s_polynomial_in(&ctx, &zero, &g).unwrap_err();
    assert_eq!(err, SPolyError::ZeroInput);
}

#[test]
fn spoly_zero_right_input_returns_error() {
    let ctx = ring_lex_2();

    let f = p(&ctx, &[(1, &[1, 0])]);
    let zero = p(&ctx, &[]);

    let err = s_polynomial_in(&ctx, &f, &zero).unwrap_err();
    assert_eq!(err, SPolyError::ZeroInput);
}

#[test]
fn spoly_ring_mismatch_returns_error() {
    let ctx1 = ring_lex_2();
    let ctx2 = ring_lex_3();

    let f = p(&ctx1, &[(1, &[2, 0]), (1, &[0, 1])]);
    let g = p(&ctx2, &[(1, &[1, 1, 0]), (1, &[0, 0, 0])]);

    let err = s_polynomial_in(&ctx1, &f, &g).unwrap_err();

    match err {
        SPolyError::Poly(_) => {}
        other => panic!("expected propagated polynomial/ring error, got {other:?}"),
    }
}

#[test]
fn spoly_normalized_result_cancels_lcm_leading_term() {
    let ctx = ring_lex_2();

    let f = p(&ctx, &[(1, &[2, 0]), (1, &[0, 1])]);
    let g = p(&ctx, &[(1, &[1, 1]), (1, &[0, 0])]);

    let mut s = s_polynomial_in(&ctx, &f, &g).unwrap();
    s.normalize_in_place(&ctx).unwrap();

    let expected = p(&ctx, &[(32002, &[1, 0]), (1, &[0, 2])]);

    assert_eq!(pretty(&ctx, &s), pretty(&ctx, &expected));
}
