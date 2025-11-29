#![allow(missing_docs)]

use algebra_core::Semiring;

fn left_distrib<T>(a: T, b: T, c: T)
where
    T: Semiring + Clone + PartialEq + core::fmt::Debug,
{
    let left = a.clone() * (b.clone() + c.clone());
    let right = a.clone() * b.clone() + a * c;
    assert_eq!(left, right);
}

fn right_distrib<T>(a: T, b: T, c: T)
where
    T: Semiring + Clone + PartialEq + core::fmt::Debug,
{
    let left = (a.clone() + b.clone()) * c.clone();
    let right = a.clone() * c.clone() + b * c;
    assert_eq!(left, right);
}

fn annihilation<T>(a: T)
where
    T: Semiring + Clone + PartialEq + core::fmt::Debug,
{
    let z = T::zero(); // Semiring -> AddMonoid -> Zero
    assert_eq!(z.clone() * a.clone(), z);
    assert_eq!(a * z.clone(), z);
}

#[test]
fn semiring_laws_u32() {
    left_distrib::<u32>(2, 3, 5);
    right_distrib::<u32>(2, 3, 5);
    annihilation::<u32>(7);
}
