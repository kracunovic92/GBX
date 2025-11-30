//! Generic tests for multiplicative algebraic laws.
//!
//! These tests exercise the multiplicative traits from `algebra_core`
//! on some concrete types. The helpers are generic, so they can be reused
//! in other crates (fields, rings, etc.).

use algebra_core::{MulAbelianMonoid, MulMonoid, MulSemigroup, Multiplicative, One, TryInverse};
use std::fmt::Debug;

/// Check associativity: (a * b) * c == a * (b * c)
fn check_mul_associative<T>(values: &[T])
where
    T: MulSemigroup + Copy + Eq + Debug,
{
    for &a in values {
        for &b in values {
            for &c in values {
                let lhs = <T as Multiplicative>::mul(<T as Multiplicative>::mul(a, b), c);
                let rhs = <T as Multiplicative>::mul(a, <T as Multiplicative>::mul(b, c));
                assert_eq!(
                    lhs, rhs,
                    "mul associativity failed for ({a:?}, {b:?}, {c:?})"
                );
            }
        }
    }
}

/// Check multiplicative identity: 1 * a == a and a * 1 == a
fn check_mul_identity<T>(values: &[T])
where
    T: MulMonoid + Copy + Eq + Debug,
{
    let one = T::one();
    for &a in values {
        let left = <T as Multiplicative>::mul(one, a);
        let right = <T as Multiplicative>::mul(a, one);

        assert_eq!(left, a, "left mul identity failed for {a:?}");
        assert_eq!(right, a, "right mul identity failed for {a:?}");
    }
}

/// Check multiplicative inverses: a * a⁻¹ == 1 and a⁻¹ * a == 1
///
/// Caller is responsible for ensuring `values` does not contain
/// non-invertible elements (e.g. 0 in a field).
fn check_mul_inverse<T>(values: &[T])
where
    T: MulAbelianMonoid + One + TryInverse<Output = T> + Copy + Eq + Debug,
{
    let one = T::one();
    for &a in values {
        let inv_a = a
            .try_inv()
            .unwrap_or_else(|| panic!("value {a:?} should be invertible"));

        let left = <T as Multiplicative>::mul(a, inv_a);
        let right = <T as Multiplicative>::mul(inv_a, a);

        assert_eq!(left, one, "a * inv(a) != 1 for {a:?}");
        assert_eq!(right, one, "inv(a) * a != 1 for {a:?}");
    }
}

/// Check commutativity: a * b == b * a
fn check_mul_commutative<T>(values: &[T])
where
    T: MulAbelianMonoid + Copy + Eq + Debug,
{
    for &a in values {
        for &b in values {
            let lhs = <T as Multiplicative>::mul(a, b);
            let rhs = <T as Multiplicative>::mul(b, a);
            assert_eq!(lhs, rhs, "mul commutativity failed for ({a:?}, {b:?})");
        }
    }
}

// -------------------------------------------------------------------------
// Concrete tests
// -------------------------------------------------------------------------

#[test]
fn mul_monoid_laws_for_u32() {
    // u32 with standard multiplication is a monoid:
    // - associative
    // - identity = 1
    // but not a group (no inverses for all elements).
    let values: Vec<u32> = (0u32..=20u32).collect();

    check_mul_associative(&values);
    check_mul_identity(&values);

    // We do *not* call `check_mul_inverse` or `check_mul_commutative` with
    // `MulAbelianGroup` bounds here, because u32 under multiplication is NOT
    // a group (0 has no inverse, non-zero elements don't form a group either).
}
