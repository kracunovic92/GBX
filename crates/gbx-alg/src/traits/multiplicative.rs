//! Multiplicative operation traits and structure markers.
//!
//! Like the additive side, we separate the raw operation (`*`) from the algebraic laws.
//! These traits are primarily **capability markers**; associativity/commutativity are
//! expected and should be validated by tests (and optional `laws` helpers).
//!
//! Layering:
//! - [`Multiplicative`]        – operation `a * b`
//! - [`MultiplicativeAssign`]  – operation `a *= b`
//! - [`MulSemigroup`]          – associative multiplication (law)
//! - [`MulMonoid`]             – semigroup + identity (`One`)
//! - [`MulAbelianMonoid`]      – monoid + commutativity (law)

use crate::One;

/// Raw multiplicative operation: `self * rhs`.
///
/// Thin wrapper around [`core::ops::Mul`] with `Output = Self`.
pub trait Multiplicative: Sized + core::ops::Mul<Output = Self> {
    /// Multiplies `self` by `rhs`, returning the product.
    #[must_use]
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        core::ops::Mul::mul(self, rhs)
    }
}

/// Any type implementing `Mul<Output = Self>` is automatically `Multiplicative`.
impl<T> Multiplicative for T where T: core::ops::Mul<Output = T> {}

/// In-place multiplication: `x *= y`.
pub trait MultiplicativeAssign: Multiplicative + core::ops::MulAssign<Self> {}

impl<T> MultiplicativeAssign for T where T: Multiplicative + core::ops::MulAssign<Self> {}

/// Marker: multiplicative semigroup (associativity is expected).
pub trait MulSemigroup: Multiplicative {}

impl<T> MulSemigroup for T where T: Multiplicative {}

/// Marker: multiplicative monoid (has identity `1`).
pub trait MulMonoid: MulSemigroup + One {}

impl<T> MulMonoid for T where T: MulSemigroup + One {}

/// Marker: commutative multiplicative monoid (commutativity is expected).
pub trait MulAbelianMonoid: MulMonoid {}

impl<T> MulAbelianMonoid for T where T: MulMonoid {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct MyMul(i32);

    impl core::ops::Mul for MyMul {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }

    impl core::ops::MulAssign for MyMul {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 *= rhs.0;
        }
    }

    impl One for MyMul {
        const ONE: Self = Self(1);
    }

    #[test]
    fn multiplicative_wrapper_works() {
        let a = MyMul(3);
        let b = MyMul(4);
        assert_eq!(a.mul(b), MyMul(12));
    }

    #[test]
    fn multiplicative_assign_works() {
        let mut x = MyMul(5);
        x *= MyMul(7);
        assert_eq!(x, MyMul(35));
    }

    #[test]
    fn i64_is_mul_monoid() {
        fn requires_monoid<T: MulMonoid>(_x: T) {}
        requires_monoid(2i64);
    }
}
