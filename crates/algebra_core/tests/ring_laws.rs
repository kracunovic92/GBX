#![allow(missing_docs)]

use algebra_core::Ring;

/// a * (b + c) == a * b + a * c
fn left_distrib<T>(a: T, b: T, c: T)
where
    T: Ring + Clone + PartialEq + core::fmt::Debug,
{
    let left = a.clone() * (b.clone() + c.clone());
    let right = a.clone() * b.clone() + a * c;
    assert_eq!(left, right);
}

/// (a + b) * c == a * c + b * c
fn right_distrib<T>(a: T, b: T, c: T)
where
    T: Ring + Clone + PartialEq + core::fmt::Debug,
{
    let left = (a.clone() + b.clone()) * c.clone();
    let right = a.clone() * c.clone() + b * c;
    assert_eq!(left, right);
}

/// 0 * a == 0 && a * 0 == 0
fn annihilation<T>(a: T)
where
    T: Ring + Clone + PartialEq + core::fmt::Debug,
{
    let z = T::zero();
    assert_eq!(z.clone() * a.clone(), z);
    assert_eq!(a * z.clone(), z);
}

#[test]
fn ring_laws_i32() {
    left_distrib::<i32>(2, 3, 5);
    right_distrib::<i32>(2, 3, 5);
    annihilation::<i32>(7);
}
