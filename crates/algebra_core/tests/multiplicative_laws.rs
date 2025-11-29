#![allow(missing_docs)]

use algebra_core::{Multiplicative, One};

/// associativity: (a * b) * c == a * (b * c)
fn assoc_law<T>(a: T, b: T, c: T)
where
    T: Multiplicative + Clone + PartialEq + core::fmt::Debug,
{
    let left = (a.clone() * b.clone()) * c.clone();
    let right = a * (b * c);
    assert_eq!(left, right);
}

/// identity: 1 * a == a && a * 1 == a
fn identity_law<T>(a: T)
where
    T: Multiplicative + One + Clone + PartialEq + core::fmt::Debug,
{
    let e = T::one();
    assert_eq!(e.clone() * a.clone(), a);
    assert_eq!(a.clone() * e, a);
}

#[test]
fn integers() {
    assoc_law::<i64>(2, 3, 4);
    assoc_law::<u32>(2, 3, 4);
    identity_law::<i64>(-7);
    identity_law::<u32>(7);
}
