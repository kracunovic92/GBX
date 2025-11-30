//! Additive operation traits and structure markers.
//!
//! The *operation* (`+`) is separated from its **algebraic laws**:
//!
//! - [`Additive`]        – raw operation, thin wrapper over [`core::ops::Add`]
//! - [`AdditiveAssign`]  – in-place addition, wrapper over [`core::ops::AddAssign`]
//! - [`AddSemigroup`]    – associative addition
//! - [`AddMonoid`]       – associative + identity (`Zero`)
//! - [`AddGroup`]        – additive group (identity + inverse)
//! - [`AddAbelianGroup`] – additive *commutative* (Abelian) group
//!
//! The laws themselves are not enforced at the type level; they are intended
//! to be checked via *law tests* in this crate’s test suite and in consumer
//! crates.

use crate::Zero;
use core::ops::{Add, AddAssign, Neg};

/// Raw additive operation: `self + rhs`.
///
/// In practice, this is a thin wrapper around [`core::ops::Add`] with
/// `Output = Self`. The blanket impl below means that any type implementing
/// `Add<Output = Self>` is automatically `Additive`.
pub trait Additive: Sized + Add<Output = Self> {
    /// Adds `rhs` to `self`, returning the result.
    ///
    /// The default implementation simply delegates to the `+` operator.
    #[must_use]
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

/// Blanket impl: any type with `Add<Output = Self>` is additive.
///
/// This includes primitive integer and floating-point types, as well as
/// user-defined types that implement `Add` appropriately.
impl<T> Additive for T where T: Add<Output = T> {}

/// In-place additive update: `x += y`.
///
/// This is a thin wrapper over [`core::ops::AddAssign`]. It is useful for
/// algorithms that want to work with in-place updates without committing to
/// a specific representation.
pub trait AdditiveAssign: Additive + AddAssign<Self> {}

/// Marker trait: additive semigroup.
///
/// This indicates that:
/// - The `add` operation is **closed** on `Self`.
/// - Addition is **associative**:
///   `(a + b) + c == a + (b + c)` for all `a, b, c`.
///
/// These laws are *not* enforced in the type system; they are expected to be
/// validated by law tests.
pub trait AddSemigroup: Additive {}

/// Marker trait: additive monoid.
///
/// This extends [`AddSemigroup`] by requiring an additive identity element,
/// provided by [`Zero`]:
///
/// - There exists `0` such that:
///   - `0 + a == a`
///   - `a + 0 == a`
///
/// Implementors must also implement [`Zero`] consistently.
pub trait AddMonoid: AddSemigroup + Zero {}

/// Full additive group: every element has an additive inverse.
///
/// Extends [`AddMonoid`] with:
///
/// - For every `a` there exists `-a` such that:
///   - `a + (-a) == 0`
///   - `(-a) + a == 0`
///
/// In Rust, this is expressed via [`core::ops::Neg`].
pub trait AddGroup: AddMonoid + Neg<Output = Self> {
    /// Convenience method: additive inverse (`-self`).
    #[must_use]
    #[inline]
    fn neg(self) -> Self {
        -self
    }
}

/// Marker trait: additive *commutative* (Abelian) group.
///
/// In a ring or field, the underlying additive group is always Abelian.
/// This trait makes that explicit in the type system, even though the
/// commutativity law itself is not enforced.
pub trait AddAbelianGroup: AddGroup {}

/// Blanket impls for the marker traits.
///
/// Any type that satisfies the bounds is considered to have the corresponding
/// algebraic structure.
impl<T> AddSemigroup for T where T: Additive {}
impl<T> AddMonoid for T where T: AddSemigroup + Zero {}
impl<T> AddGroup for T where T: AddMonoid + Neg<Output = T> {}
impl<T> AddAbelianGroup for T where T: AddGroup {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct MyAdditive(i32);

    impl Add for MyAdditive {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            MyAdditive(self.0 + rhs.0)
        }
    }

    impl AddAssign for MyAdditive {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }

    impl Neg for MyAdditive {
        type Output = Self;

        fn neg(self) -> Self::Output {
            MyAdditive(-self.0)
        }
    }

    impl Zero for MyAdditive {
        const ZERO: Self = MyAdditive(0);
    }

    fn assert_add_abelian_group<T: AddAbelianGroup>(_x: T) {}

    #[test]
    fn additive_and_group_methods_work_for_custom_type() {
        let a = MyAdditive(3);
        let b = MyAdditive(4);

        assert_eq!(
            a.add(b)
                .0,
            7
        );

        let neg_a = a.neg();
        assert_eq!(neg_a.0, -3);

        assert_add_abelian_group(a);
    }

    #[test]
    fn additive_assign_updates_in_place() {
        let mut x = MyAdditive(5);
        x += MyAdditive(7);
        assert_eq!(x.0, 12);
    }
}
