#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::*;

use crate::algos::post::reduction::validate::{assert_fully_reduced_basis, assert_minimal_leading_monomials, find_reduction_violations};

use gbx_field::fp::{Fp, FpElem};
use gbx_poly::order::Lex;
use gbx_poly::poly;
use gbx_poly::polynomial::Polynomial;
use gbx_poly::ring::Ring;

type P = Polynomial<FpElem>;

fn test_ctx() -> RingCtx<Fp, Lex> {
    let field = Fp::prime(32003).unwrap();

    Ring::builder()
        .field(field)
        .order(Lex)
        .nvars(2)
        .build()
        .unwrap()
}

#[test]
fn minimality_check_rejects_divisible_leading_monomials() {
    let ctx = test_ctx();

    // x^2 + y
    let x2_plus_y: P = poly!(&ctx; (1, [2, 0]), (1, [0, 1])).unwrap();

    // x + 1
    let x_plus_one: P = poly!(&ctx; (1, [1, 0]), (1, [0, 0])).unwrap();

    let basis = vec![x2_plus_y, x_plus_one];

    let err = assert_minimal_leading_monomials(&basis).unwrap_err();

    assert!(matches!(err, PostError::InvariantViolation));
}

#[test]
fn reduced_basis_check_detects_tail_violation() {
    let ctx = test_ctx();

    // x + y
    let x_plus_y: P = poly!(&ctx; (1, [1, 0]), (1, [0, 1])).unwrap();

    // y + 1
    let y_plus_one: P = poly!(&ctx; (1, [0, 1]), (1, [0, 0])).unwrap();

    let basis = vec![x_plus_y, y_plus_one];

    let violations = find_reduction_violations(&basis);

    assert_eq!(violations.len(), 1);
    assert!(assert_fully_reduced_basis(&basis).is_err());
}

#[test]
fn reduced_basis_check_accepts_reduced_basis() {
    let ctx = test_ctx();

    // x + 1
    let x_plus_one: P = poly!(&ctx; (1, [1, 0]), (1, [0, 0])).unwrap();

    // y + 1
    let y_plus_one: P = poly!(&ctx; (1, [0, 1]), (1, [0, 0])).unwrap();

    let basis = vec![x_plus_one, y_plus_one];

    assert_minimal_leading_monomials(&basis).unwrap();
    assert_fully_reduced_basis(&basis).unwrap();
}

#[test]
fn reduce_in_place_removes_zero_polynomials() {
    let ctx = test_ctx();

    let zero: P = Polynomial::zero_in(&ctx);

    // x + 1
    let x_plus_one: P = poly!(&ctx; (1, [1, 0]), (1, [0, 0])).unwrap();

    let mut gb = GrobnerBasis::new(ctx.id(), vec![zero, x_plus_one]);

    reduce_in_place(&ctx, &mut gb).unwrap();

    assert_eq!(gb.len(), 1);
    assert_fully_reduced_basis(gb.as_slice()).unwrap();
}

#[test]
fn reduce_in_place_tail_reduces_basis() {
    let ctx = test_ctx();

    // x + y
    let x_plus_y: P = poly!(&ctx; (1, [1, 0]), (1, [0, 1])).unwrap();

    // y + 1
    let y_plus_one: P = poly!(&ctx; (1, [0, 1]), (1, [0, 0])).unwrap();

    let mut gb = GrobnerBasis::new(ctx.id(), vec![x_plus_y, y_plus_one]);

    reduce_in_place(&ctx, &mut gb).unwrap();

    assert_minimal_leading_monomials(gb.as_slice()).unwrap();
    assert_fully_reduced_basis(gb.as_slice()).unwrap();
}
