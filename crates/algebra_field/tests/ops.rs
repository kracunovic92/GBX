#![allow(missing_docs)]

use algebra_core::{CheckedDiv, One, TryInverse, Zero};
use algebra_field::Zp;

type F7 = Zp<7>;
type F11 = Zp<11>;

#[test]
fn construction_and_reduction() {
    let a = F7::new(5);
    assert_eq!(a.value(), 5);
    assert_eq!(F7::new(7).value(), 0);
    assert_eq!(F7::new(8).value(), 1);
}

#[test]
fn identities() {
    assert_eq!(F7::zero().value(), 0);
    assert_eq!(F7::one().value(), 1);
}

#[test]
fn add_sub_neg() {
    let a = F7::new(5);
    let b = F7::new(6);

    // 5 + 6 = 11 ≡ 4 (mod 7)
    assert_eq!((a + b).value(), 4);

    // -5 ≡ 2 (mod 7)
    assert_eq!((-a).value(), 2);

    // 5 - 6 ≡ -1 ≡ 6 (mod 7)
    assert_eq!((a - b).value(), 6);
}

#[test]
fn mul_div_inv() {
    let a = F7::new(5);
    let b = F7::new(3);

    // 5 * 3 = 15 ≡ 1 (mod 7)
    assert_eq!((a * b).value(), 1);

    let inv = a
        .try_inv()
        .unwrap();
    assert_eq!(inv.value(), 3);

    assert_eq!((a * inv).value(), 1);

    let q = a
        .checked_div(b)
        .unwrap();
    assert_eq!((q * b).value(), a.value());

    assert!(
        F7::zero()
            .try_inv()
            .is_none()
    );

    let q0 = F7::zero()
        .checked_div(F7::new(3))
        .unwrap();
    assert_eq!(q0.value(), 0);

    assert!(
        F7::one()
            .checked_div(F7::zero())
            .is_err()
    );
}

#[test]
fn sum_and_product() {
    let s = (1..5)
        .map(F11::new)
        .sum::<F11>();
    assert_eq!(s.value(), (1 + 2 + 3 + 4) % 11);

    let p = (1..5)
        .map(F11::new)
        .product::<F11>();
    let expected = (1u64 * 2 * 3 * 4) % 11;
    assert_eq!(p.value(), expected);
}
