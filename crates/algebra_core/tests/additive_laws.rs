#![allow(missing_docs)]

use algebra_core::{AddGroup, AddMonoid, AddSemigroup, Additive, Zero};
use core::fmt::Debug;

/// Convenience helper to call the `Additive` trait method unambiguously.
fn add<T: Additive>(a: T, b: T) -> T {
    <T as Additive>::add(a, b)
}

/// Convenience helper to call the `AddGroup` trait method unambiguously.
fn neg<T: AddGroup>(a: T) -> T {
    <T as AddGroup>::neg(a)
}

/// Check semigroup law: associativity of +.
fn check_add_semigroup<T, I>(values: I)
where
    T: AddSemigroup + Additive + Copy + Eq + Debug,
    I: Clone + IntoIterator<Item = T>,
{
    let vals: Vec<T> = values.into_iter().collect();

    for &a in &vals {
        for &b in &vals {
            for &c in &vals {
                let lhs = add(add(a, b), c);
                let rhs = add(a, add(b, c));
                assert_eq!(lhs, rhs, "associativity failed for {a:?}, {b:?}, {c:?}");
            }
        }
    }
}

/// Check monoid law: identity 0.
fn check_add_monoid<T, I>(values: I)
where
    T: AddMonoid + Additive + Zero + Copy + Eq + Debug,
    I: IntoIterator<Item = T>,
{
    let zero = T::zero();
    for a in values {
        assert_eq!(add(zero, a), a, "left identity failed for {a:?}");
        assert_eq!(add(a, zero), a, "right identity failed for {a:?}");
    }
}

/// Check group law: inverse.
fn check_add_group<T, I>(values: I)
where
    T: AddGroup + Additive + Zero + Copy + Eq + Debug,
    I: IntoIterator<Item = T>,
{
    let zero = T::zero();
    for a in values {
        let n = neg(a);
        assert_eq!(add(a, n), zero, "left inverse failed for {a:?}");
        assert_eq!(add(n, a), zero, "right inverse failed for {a:?}");
    }
}

/// Helper to build a small safe range for signed integers.
fn small_signed_range() -> impl Iterator<Item = i32> {
    (-10..=10).into_iter()
}

#[test]
fn i32_is_add_group() {
    let vals: Vec<i32> = small_signed_range().collect();
    check_add_semigroup::<i32, _>(vals.iter().copied());
    check_add_monoid::<i32, _>(vals.iter().copied());
    check_add_group::<i32, _>(vals.iter().copied());
}

#[test]
fn u32_is_add_monoid_but_not_group() {
    let vals: Vec<u32> = (0..=20).collect();
    check_add_semigroup::<u32, _>(vals.iter().copied());
    check_add_monoid::<u32, _>(vals.iter().copied());

    // We can't express "not AddGroup" as a type-level constraint in Rust,
    // so we just rely on the fact that there is no `impl AddGroup for u32`
    // and don't try to use `AddGroup` here.
}
