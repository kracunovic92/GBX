//! Additive operation traits and structure markers.
//!
//! The *operation* (`+`) is separated from its **algebraic laws**:
//!
//! - [`Additive`] – raw operation, thin wrapper over [`core::ops::Add`].
//! - [`AddSemigroup`] – associative addition.
//! - [`AddMonoid`] – associative + identity (`Zero`).
//! - [`AddGroup`] – additive group (identity + inverse).
//!
//! The laws themselves are not enforced at the type level; they are intended
//! to be checked via *law tests* in this crate’s test suite and in consumer
//! crates.

use core::ops::{Add, Neg};

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
/// provided by [`crate::Zero`]:
///
/// - There exists `0` such that:
///   - `0 + a == a`
///   - `a + 0 == a`
///
/// Implementors must also implement [`crate::Zero`] consistently.
pub trait AddMonoid: AddSemigroup + crate::Zero {}

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

/// Blanket impl: any type that satisfies the bounds is an `AddGroup`.
impl<T> AddGroup for T where T: AddMonoid + Neg<Output = T> {}

/// Implement additive markers for signed primitive integers.
///
/// **Note:** With Rust’s overflow semantics, group laws are only strictly valid
/// when staying within non-overflowing ranges. Law tests should account for
/// this (e.g. by restricting the domain).
macro_rules! impl_add_markers_for_signed {
    ($($t:ty),* $(,)?) => { $(
        impl AddSemigroup for $t {}
        impl AddMonoid    for $t {}
        // `AddGroup` comes from the blanket impl once `AddMonoid + Neg` hold.
    )* };
}

impl_add_markers_for_signed!(i8, i16, i32, i64, i128, isize);

/// Implement additive markers for unsigned primitive integers.
///
/// These form a monoid under wrapping addition (`+`), but not a group in the
/// usual sense (there is no inverse in the standard integer model).
macro_rules! impl_add_markers_for_unsigned {
    ($($t:ty),* $(,)?) => { $(
        impl AddSemigroup for $t {}
        impl AddMonoid    for $t {}
        // No `AddGroup` impl: additive inverses are not available.
    )* };
}

impl_add_markers_for_unsigned!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::Additive;
    use crate::AddGroup;

    #[test]
    fn add_i32() {
        assert_eq!(3i32.add(4), 7);
        // via `AddGroup::neg` default method
        assert_eq!(5i32.neg(), -5);
    }

    #[test]
    fn add_u32() {
        assert_eq!(3u32.add(4), 7);
        // `AddGroup` is *not* implemented for `u32`; that’s intentional.
    }
}
