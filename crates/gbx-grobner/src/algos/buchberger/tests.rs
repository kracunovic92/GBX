#![allow(clippy::unwrap_used)]

use super::*;
use crate::algos::post::BasisPostOptionsKind;
use gbx_field::fp::{Fp, FpDyn, FpDynElem};
use gbx_poly::monomial::{DynamicMonomial, FixedMonomial};
use gbx_poly::order::Lex;
use gbx_poly::polynomial::Polynomial;
use gbx_poly::ring::{Ring, RingCtx, StaticFpCtx};
use gbx_poly::term::Term;
use gbx_storage::polynomial::VecTerms;

// ---------------- static (Fp<7>, FixedMonomial<2>) ----------------
type F7 = Fp<7>;
type T2 = Term<F7, FixedMonomial<2>>;
type P2 = Polynomial<T2, VecTerms<T2>>;

fn ring_static() -> RingCtx<StaticFpCtx<7>, Lex> {
    Ring::builder()
        .field(StaticFpCtx::<7>::new())
        .order(Lex)
        .nvars(2)
        .build()
        .unwrap()
}

fn p2(r: &RingCtx<StaticFpCtx<7>, Lex>, terms: &[(u32, u32, u32)]) -> P2 {
    let ts = terms
        .iter()
        .map(|&(c, a, b)| Term::new(F7::new(c), FixedMonomial::<2>::from_exponents([a, b])))
        .collect();
    P2::from_terms_in(r, ts).unwrap()
}

// ---------------- dynamic (FpDyn, DynamicMonomial) ----------------
type TD = Term<FpDynElem, DynamicMonomial>;
type PD = Polynomial<TD, VecTerms<TD>>;

fn ring_dyn() -> RingCtx<FpDyn, Lex> {
    Ring::builder()
        .field(FpDyn::prime(7).unwrap())
        .order(Lex)
        .nvars(2)
        .build()
        .unwrap()
}

fn pd(r: &RingCtx<FpDyn, Lex>, terms: &[(u32, &[u32])]) -> PD {
    let ts = terms
        .iter()
        .map(|&(c, e)| Term::new(r.field.new(c), DynamicMonomial::from_slice(e)))
        .collect();
    PD::from_terms_in(r, ts).unwrap()
}

#[test]
fn buchberger_keeps_ctx_tag_and_trivial_basis() {
    let r = ring_static();
    let opts = BuchbergerOptions::default();

    let f0 = p2(&r, &[(1, 1, 0)]);
    let f1 = p2(&r, &[(1, 0, 1)]);

    let gb = buchberger(&r, vec![f0, f1], opts).unwrap();
    assert_eq!(gb.ring_id(), r.id());
    assert_eq!(gb.len(), 2);
}

#[test]
fn buchberger_dynamic_trivial_basis() {
    let r = ring_dyn();
    let opts = BuchbergerOptions::default();

    let f0 = pd(&r, &[(1, &[1, 0])]);
    let f1 = pd(&r, &[(1, &[0, 1])]);

    let gb = buchberger(&r, vec![f0, f1], opts).unwrap();
    assert_eq!(gb.ring_id(), r.id());
    assert_eq!(gb.len(), 2);
}

#[test]
fn buchberger_empty_input_returns_empty_basis() {
    let r = ring_static();
    let gb: crate::GrobnerBasis<P2> = buchberger(&r, Vec::<P2>::new(), BuchbergerOptions::default()).unwrap();
    assert!(gb.is_empty());
}

#[test]
fn buchberger_post_none_keeps_generators() {
    let r = ring_static();
    let opts = BuchbergerOptions { post: BasisPostOptionsKind::None, ..Default::default() };

    let f0 = p2(&r, &[(1, 2, 0), (1, 0, 2), (6, 0, 0)]);
    let f1 = p2(&r, &[(1, 3, 0), (6, 0, 1)]);

    let gb = buchberger(&r, vec![f0, f1], opts).unwrap();
    assert!(gb.len() >= 2);
}

#[test]
fn buchberger_post_reduced_runs() {
    let r = ring_static();
    let opts = BuchbergerOptions { post: BasisPostOptionsKind::Reduced, ..Default::default() };

    let f0 = p2(&r, &[(1, 1, 0)]);
    let f1 = p2(&r, &[(1, 0, 1)]);

    let gb = buchberger(&r, vec![f0, f1], opts).unwrap();
    assert_eq!(gb.len(), 2);
}
