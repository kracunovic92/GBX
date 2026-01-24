#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use gbx_alg::One;

use super::Zp;

#[test]
fn basic_arithmetic_mod_7() {
    type Z = Zp<7>;
    let a = Z::new(5);
    let b = Z::new(3);

    assert_eq!((a + b).value(), 1);
    assert_eq!((a * b).value(), 1);
    assert_eq!((-a).value(), 2);
}

#[test]
fn pow_works() {
    type Z = Zp<7>;
    let a = Z::new(3);
    assert_eq!(a.pow(0).value(), Z::one().value());
    assert_eq!(a.pow(1).value(), 3);
    assert_eq!(a.pow(2).value(), 2);
}

#[cfg(feature = "laws")]
#[test]
fn zp_is_ring_by_laws_small_moduli() {
    let elems7: Vec<Zp<7>> = Zp::<7>::iter_all().collect();
    gbx_alg::laws::check_ring(&elems7);

    let elems8: Vec<Zp<8>> = Zp::<8>::iter_all().collect();
    gbx_alg::laws::check_ring(&elems8);
}
