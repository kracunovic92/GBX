#![allow(missing_docs)]

use algebra_core::{Additive, Field, Multiplicative, One, Ring, Semiring, TryInverse, Zero};
use algebra_field::Zp;
use core::fmt::Debug;

fn check_add_laws<F, I>(vals: I)
where
    F: Field + Additive + Zero + Copy + Eq + Debug,
    I: Clone + IntoIterator<Item = F>,
{
    let xs: Vec<F> = vals.into_iter().collect();
    let zero = F::zero();

    // associativity & identity
    for &a in &xs {
        for &b in &xs {
            for &c in &xs {
                let lhs = a + b + c;
                let rhs = a + (b + c);
                assert_eq!(lhs, rhs);
            }
        }
        assert_eq!(zero + a, a);
        assert_eq!(a + zero, a);
    }

    // inverse
    for &a in &xs {
        let inv = -a;
        assert_eq!(a + inv, zero);
        assert_eq!(inv + a, zero);
    }
}

fn check_mul_laws<F, I>(vals: I)
where
    F: Field + Multiplicative + One + Zero + Copy + Eq + Debug,
    I: Clone + IntoIterator<Item = F>,
{
    let xs: Vec<F> = vals.into_iter().collect();
    let one = F::one();
    let zero = F::zero();

    // associativity & identity
    for &a in &xs {
        for &b in &xs {
            for &c in &xs {
                let lhs = a * b * c;
                let rhs = a * (b * c);
                assert_eq!(lhs, rhs);
            }
        }
        assert_eq!(one * a, a);
        assert_eq!(a * one, a);
    }

    // zero annihilator
    for &a in &xs {
        assert_eq!(zero * a, zero);
        assert_eq!(a * zero, zero);
    }
}

fn check_distributivity<F, I>(vals: I)
where
    F: Semiring + Additive + Multiplicative + Copy + Eq + Debug,
    I: Clone + IntoIterator<Item = F>,
{
    let xs: Vec<F> = vals.into_iter().collect();

    for &a in &xs {
        for &b in &xs {
            for &c in &xs {
                let left = a * (b + c);
                let right = a * b + a * c;
                assert_eq!(left, right);

                let left = (a + b) * c;
                let right = a * c + b * c;
                assert_eq!(left, right);
            }
        }
    }
}

fn check_field_axioms<const P: u64>() {
    // Small sample of representatives [0..P)
    let vals: Vec<Zp<P>> = (0..P).map(Zp::<P>::new).collect();

    // trait membership sanity
    fn assert_field<T: Field + Ring + Semiring>(_x: T) {}
    assert_field(Zp::<P>::new(1));

    // Skip 0 for additive group/inverse tests
    let nonzero: Vec<Zp<P>> = vals
        .iter()
        .copied()
        .filter(|x: &Zp<P>| !x.is_zero())
        .collect::<Vec<Zp<P>>>();

    check_add_laws::<Zp<P>, _>(nonzero.iter().copied());
    check_mul_laws::<Zp<P>, _>(nonzero.iter().copied());
    check_distributivity::<Zp<P>, _>(vals.iter().copied());

    // every non-zero element has an inverse
    for &x in &nonzero {
        let inv = <Zp<P> as TryInverse>::try_inv(x).expect("nonzero must be invertible in a field");
        assert_eq!(x * inv, Zp::<P>::one());
    }
}

#[test]
fn field_axioms_for_small_primes() {
    check_field_axioms::<2>();
    check_field_axioms::<3>();
    check_field_axioms::<5>();
    check_field_axioms::<7>();
    check_field_axioms::<11>();
}
