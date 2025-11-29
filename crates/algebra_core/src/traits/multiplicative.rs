//! Multiplicative operation traits and structure markers.
//!
//! As with the additive side, we separate the raw *operation* (`*`) from its
//! algebraic *laws*:
//!
//! - [`Multiplicative`]  – raw operation, thin wrapper over [`core::ops::Mul`].
//! - [`MulSemigroup`]   – associative multiplication.
//! - [`MulMonoid`]      – associative + identity (`One`).
//!
//! Higher structures such as [`crate::Semiring`], [`crate::Ring`], and
//! [`crate::Field`] build on these traits. The laws are not enforced by the
//! type system; they are intended to be verified via law tests.

use core::ops::Mul;

/// Raw multiplicative operation: `self * rhs`.
///
/// This is a thin wrapper around [`core::ops::Mul`] with `Output = Self`.
/// The blanket implementation below means that any type implementing
/// `Mul<Output = Self>` automatically implements [`Multiplicative`].
pub trait Multiplicative: Sized + Mul<Output = Self> {
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
impl<T> Multiplicative for T where T: Mul<Output = T> {}

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
/// [`crate::One`]:
///
/// - There exists `1` such that:
///   - `1 * a == a`
///   - `a * 1 == a`
pub trait MulMonoid: MulSemigroup + crate::One {}

/// Implement multiplicative markers for primitive integers.
///
/// **Note:** With Rust’s overflow semantics, monoid laws strictly hold only
/// when staying within non-overflowing ranges. Law tests should take this
/// into account by restricting the domain or using wrapping semantics
/// explicitly where desired.
macro_rules! impl_mul_markers_for_ints {
    ($($t:ty),* $(,)?) => { $(
        impl MulSemigroup for $t {}
        impl MulMonoid    for $t {}
    )* };
}

impl_mul_markers_for_ints!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,
);

#[cfg(test)]
mod tests {
    use super::Multiplicative;

    #[test]
    fn mul_i64() {
        assert_eq!(3i64.mul(4), 12);
    }
}
