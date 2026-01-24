#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use gbx_alg::{Field, One, TryInverse};

use super::Fp;

#[test]
fn fp7_inverse_works() {
    type F = Fp<7>;
    let a = F::new(5);
    let inv = a.try_inv().unwrap();
    assert_eq!((a * inv).value(), 1);
}

#[cfg(feature = "laws")]
#[test]
fn fp7_is_field_by_laws() {
    let elems: Vec<Fp<7>> = Fp::<7>::iter_all().collect();
    gbx_alg::laws::check_field(&elems);
}

#[test]
fn fp_marker_compiles() {
    fn needs_field<T: Field>(_x: T) {}
    needs_field(Fp::<7>::one());
}
