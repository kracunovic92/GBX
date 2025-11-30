//! Generic tests for additive algebraic laws.
//!
//! These tests exercise the traits from `algebra_core::additive`
//! on some concrete types (i32, u32). The helpers are generic, so
//! they can be reused in other crates.

use algebra_core::{AddAbelianGroup, AddGroup, AddMonoid, AddSemigroup, Additive};
use std::fmt::Debug;

/// Check associativity: (a + b) + c == a + (b + c)
fn check_add_associative<T>(values: &[T])
where
    T: AddSemigroup + Copy + Eq + Debug,
{
    for &a in values {
        for &b in values {
            for &c in values {
                let lhs = <T as Additive>::add(<T as Additive>::add(a, b), c);
                let rhs = <T as Additive>::add(a, <T as Additive>::add(b, c));
                assert_eq!(
                    lhs, rhs,
                    "add associativity failed for ({a:?}, {b:?}, {c:?})"
                );
            }
        }
    }
}

/// Check additive identity: 0 + a == a and a + 0 == a
fn check_add_identity<T>(values: &[T])
where
    T: AddMonoid + Copy + Eq + Debug,
{
    let z = T::zero();
    for &a in values {
        let left = <T as Additive>::add(z, a);
        let right = <T as Additive>::add(a, z);

        assert_eq!(left, a, "left add identity failed for {a:?}");
        assert_eq!(right, a, "right add identity failed for {a:?}");
    }
}

/// Check additive inverses: a + (-a) == 0 and (-a) + a == 0
fn check_add_inverse<T>(values: &[T])
where
    T: AddGroup + Copy + Eq + Debug,
{
    let z = T::zero();
    for &a in values {
        let neg_a = <T as AddGroup>::neg(a);

        let left = <T as Additive>::add(a, neg_a);
        let right = <T as Additive>::add(neg_a, a);

        assert_eq!(left, z, "a + (-a) != 0 for {a:?}");
        assert_eq!(right, z, "(-a) + a != 0 for {a:?}");
    }
}

/// Check commutativity: a + b == b + a
fn check_add_commutative<T>(values: &[T])
where
    T: AddAbelianGroup + Copy + Eq + Debug,
{
    for &a in values {
        for &b in values {
            let lhs = <T as Additive>::add(a, b);
            let rhs = <T as Additive>::add(b, a);
            assert_eq!(lhs, rhs, "add commutativity failed for ({a:?}, {b:?})");
        }
    }
}

#[test]
fn add_laws_for_i32() {
    // Small symmetric range to avoid overflow & keep tests fast.
    let values: Vec<i32> = (-10..=10).collect();

    check_add_associative(&values);
    check_add_identity(&values);
    check_add_inverse(&values);
    check_add_commutative(&values);
}

#[test]
fn add_monoid_laws_for_u32() {
    // u32 is a monoid under addition (identity 0),
    // but not a group (no additive inverses for all elements).
    let values: Vec<u32> = (0u32..=20u32).collect();

    check_add_associative(&values);
    check_add_identity(&values);

    // We intentionally do NOT call check_add_inverse / check_add_commutative
    // with AddAbelianGroup bounds here. Even though addition for u32 is
    // commutative, we don't want to pretend it forms an additive group.
}
