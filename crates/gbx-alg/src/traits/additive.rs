//! Additive operation traits and structure markers.
//!
//! We separate the raw operation (`+`) from the algebraic *laws*.
//! The type system cannot enforce associativity/commutativity, so these traits
//! are **markers** intended to be validated via tests (and optional `laws` helpers).
//!
//! Layering:
//! - [`Additive`]        – operation `a + b`
//! - [`AdditiveAssign`]  – operation `a += b`
//! - [`AddSemigroup`]    – associative addition (law)
//! - [`AddMonoid`]       – semigroup + identity (`Zero`)
//! - [`AddGroup`]        – monoid + inverse (`-a`)
//! - [`AddAbelianGroup`] – group + commutativity (law)

use crate::Zero;

/// Raw additive operation: `self + rhs`.
///
/// This is a thin wrapper around [`core::ops::Add`] with `Output = Self`.
///
/// The default method is `#[inline]` and should optimize to the same code as `+`.
pub trait Additive: Sized + core::ops::Add<Output = Self> {
    /// Adds `rhs` to `self`, returning the result.
    #[must_use]
    #[inline]
    fn add(self, rhs: Self) -> Self {
        core::ops::Add::add(self, rhs)
    }
}

/// Any type implementing `Add<Output = Self>` is automatically `Additive`.
impl<T> Additive for T where T: core::ops::Add<Output = T> {}

/// In-place additive update: `x += y`.
pub trait AdditiveAssign: Additive + core::ops::AddAssign<Self> {}

impl<T> AdditiveAssign for T where T: Additive + core::ops::AddAssign<Self> {}

/// Marker: additive semigroup (associativity is expected).
pub trait AddSemigroup: Additive {}

impl<T> AddSemigroup for T where T: Additive {}

/// Marker: additive monoid (has identity `0`).
pub trait AddMonoid: AddSemigroup + Zero {}

impl<T> AddMonoid for T where T: AddSemigroup + Zero {}

/// Marker: additive group (has inverses).
pub trait AddGroup: AddMonoid + core::ops::Neg<Output = Self> {
    /// Convenience: additive inverse `-self`.
    #[must_use]
    #[inline]
    fn neg(self) -> Self {
        core::ops::Neg::neg(self)
    }
}

impl<T> AddGroup for T where T: AddMonoid + core::ops::Neg<Output = T> {}

/// Marker: additive abelian group (commutativity is expected).
pub trait AddAbelianGroup: AddGroup {}

impl<T> AddAbelianGroup for T where T: AddGroup {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct MyAdd(i32);

    impl core::ops::Add for MyAdd {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }

    impl core::ops::AddAssign for MyAdd {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }

    impl core::ops::Neg for MyAdd {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self(-self.0)
        }
    }

    impl Zero for MyAdd {
        const ZERO: Self = Self(0);
    }

    #[test]
    fn additive_wrapper_works() {
        let a = MyAdd(3);
        let b = MyAdd(4);
        assert_eq!(a.add(b), MyAdd(7));
    }

    #[test]
    fn additive_assign_works() {
        let mut x = MyAdd(5);
        x += MyAdd(7);
        assert_eq!(x, MyAdd(12));
    }

    #[test]
    fn group_neg_works() {
        let a = MyAdd(9);
        assert_eq!(a.neg(), MyAdd(-9));
    }

    #[test]
    fn i32_is_add_group() {
        fn requires_group<T: AddGroup>(_x: T) {}
        requires_group(1i32);
    }

    #[test]
    fn u32_is_add_monoid_but_not_group() {
        fn requires_monoid<T: AddMonoid>(_x: T) {}
        requires_monoid(1u32);
    }
}
