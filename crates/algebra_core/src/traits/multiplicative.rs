#![allow(clippy::needless_path_prefix)]
//! Multiplicative operation traits and structure markers.
//!
//! As with the additive side, we separate the raw *operation* (`*`) from its
//! algebraic *laws*:
//!
//! - [`Multiplicative`]       – raw operation, thin wrapper over [`core::ops::Mul`]
//! - [`MultiplicativeAssign`] – in-place multiplication, wrapper over [`core::ops::MulAssign`]
//! - [`MulSemigroup`]         – associative multiplication
//! - [`MulMonoid`]            – associative + identity (`One`)
//! - [`MulAbelianMonoid`]     – commutative multiplicative monoid
//!
//! Higher structures such as rings and fields build on these traits. The laws
//! are not enforced by the type system; they are intended to be verified via
//! law tests.

use crate::One;

/// Raw multiplicative operation: `self * rhs`.
///
/// This is a thin wrapper around [`core::ops::Mul`] with `Output = Self`.
/// The blanket implementation below means that any type implementing
/// `Mul<Output = Self>` automatically implements [`Multiplicative`].
pub trait Multiplicative: Sized + core::ops::Mul<Output = Self> {
    /// Multiplies `self` by `rhs`, returning the product.
    ///
    /// The default implementation simply delegates to the `*` operator.
    #[must_use]
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self * rhs
    }
}

/// Blanket impl: any type that already implements `Mul<Output = Self>`
/// automatically gets [`Multiplicative`].
impl<T> Multiplicative for T where T: core::ops::Mul<Output = T> {}

/// In-place multiplicative update: `x *= y`.
///
/// This is a thin wrapper over [`core::ops::MulAssign`]. It is useful for
/// algorithms that want to work with in-place updates without committing to
/// a specific representation.
pub trait MultiplicativeAssign: Multiplicative + core::ops::MulAssign<Self> {}

/// Marker trait: multiplicative semigroup.
///
/// Indicates that:
/// - Multiplication is **closed** on `Self`.
/// - Multiplication is **associative**:
///   `(a * b) * c == a * (b * c)` for all `a, b, c`.
///
/// These laws are *not* enforced at the type level and should be checked via
/// law tests.
pub trait MulSemigroup: Multiplicative {}

/// Marker trait: multiplicative monoid.
///
/// Extends [`MulSemigroup`] by requiring a multiplicative identity, provided by
/// [`One`]:
///
/// - There exists `1` such that:
///   - `1 * a == a`
///   - `a * 1 == a`
///
/// Implementors must also implement [`One`] consistently.
pub trait MulMonoid: MulSemigroup + One {}

/// Marker trait: commutative multiplicative monoid.
///
/// In a (commutative) ring or field, multiplication is intended to be
/// associative and commutative with an identity. The commutativity law is
/// documented here but not enforced at the type level.
pub trait MulAbelianMonoid: MulMonoid {}

/// Blanket marker impls: any type satisfying the bounds gets the corresponding
/// multiplicative structure automatically.
impl<T> MultiplicativeAssign for T where T: Multiplicative + core::ops::MulAssign<Self> {}
impl<T> MulSemigroup for T where T: Multiplicative {}
impl<T> MulMonoid for T where T: MulSemigroup + One {}
impl<T> MulAbelianMonoid for T where T: MulMonoid {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct MyMul(i32);

    impl core::ops::Mul for MyMul {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            MyMul(self.0 * rhs.0)
        }
    }

    impl core::ops::MulAssign for MyMul {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 *= rhs.0;
        }
    }

    impl One for MyMul {
        const ONE: Self = MyMul(1);
    }

    // We do NOT manually implement MulSemigroup/MulMonoid/etc:
    // they are supplied by the blanket impls in this module.

    fn _requires_abelian_monoid<T: MulAbelianMonoid>(_x: T) {}

    #[test]
    fn multiplicative_and_markers_work_for_custom_type() {
        let a = MyMul(3);
        let b = MyMul(4);

        assert_eq!(
            a.mul(b)
                .0,
            12
        );

        // This compiles only if MyMul satisfies all marker trait bounds
        // via the blanket implementations.
        _requires_abelian_monoid(a);
    }

    #[test]
    fn multiplicative_assign_updates_in_place() {
        let mut x = MyMul(5);
        x *= MyMul(7);
        assert_eq!(x.0, 35);
    }

    #[test]
    fn primitive_ints_are_multiplicative_monoids() {
        // Thanks to:
        // - Mul<Output = Self>
        // - One (from identity impls)
        // they automatically implement
        // - Multiplicative
        // - MulSemigroup
        // - MulMonoid
        //
        assert_eq!(3i64.mul(4), 12);

        fn _requires_monoid<T: MulMonoid>(_x: T) {}
        _requires_monoid(3i64);
    }
}
