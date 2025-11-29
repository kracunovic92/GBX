//! Identity elements for algebraic structures.
//!
//! This module defines three tiny, orthogonal traits:
//! - [`Zero`]: additive identity
//! - [`One`]:  multiplicative identity
//! - [`Scalar`]: convenient alias for types that have both identities and
//!   behave like small “field-like” scalars (copyable, comparable).
//!
//! These traits are intentionally minimal and live at the very bottom of the
//! algebra hierarchy. Higher-level traits (groups, rings, fields) build on
//! top of them.

use core::cmp::PartialEq;

/// Additive identity: an element `0` such that `0 + a = a = a + 0`.
///
/// Only construction (`zero()`) plus a convenience predicate (`is_zero()`).
/// We do not define addition here; composition happens in higher-level traits.
///
/// # Examples
/// ```
/// use algebra_core::Zero;
/// assert_eq!(i32::zero(), 0);
/// assert!(0i64.is_zero());
/// assert!(!5i64.is_zero());
/// ```
pub trait Zero: Sized + PartialEq {
    /// The additive identity `0` as an associated constant.
    const ZERO: Self;

    /// Return `0`.
    #[must_use]
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    /// Returns `true` if `self` is equal to the additive identity `0`.
    #[must_use]
    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

/// Multiplicative identity: an element `1` such that `1 * a = a = a * 1`.
///
/// Only construction (`one()`) plus a convenience predicate (`is_one()`)
/// is provided.
///
/// # Examples
/// ```
/// use algebra_core::One;
/// assert_eq!(u128::one(), 1);
/// assert!(1u32.is_one());
/// assert!(!2u32.is_one());
/// ```
pub trait One: Sized + PartialEq {
    /// The multiplicative identity `1` as an associated constant.
    const ONE: Self;

    /// Return `1`.
    #[must_use]
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    /// Returns `true` if `self` is equal to the multiplicative identity `1`.
    #[must_use]
    #[inline]
    fn is_one(&self) -> bool {
        *self == Self::ONE
    }
}

/// A “scalar-like” type: it has both additive and multiplicative identities,
/// is bit-copyable and comparable for equality.
///
/// This is just a convenience alias used throughout the library to shorten
/// generic bounds. It does **not** assume any additive or multiplicative
/// operations beyond the identities themselves.
///
/// # Examples
///
/// Implementing `Zero` and `One` for a custom type and using it as a `Scalar`:
///
/// ```
/// use algebra_core::{Zero, One, Scalar};
///
/// #[derive(Clone, Copy, PartialEq, Debug)]
/// struct MyField(u32);
///
/// impl Zero for MyField {
///     const ZERO: Self = MyField(0);
/// }
///
/// impl One for MyField {
///     const ONE: Self = MyField(1);
/// }
///
/// // Any type that implements Zero + One + Copy + PartialEq is a Scalar.
/// fn takes_scalar<S: Scalar>(_x: S) {}
///
/// takes_scalar(MyField::one());
/// ```
pub trait Scalar: Zero + One + Copy + PartialEq {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct MyField(u32);

    impl Zero for MyField {
        const ZERO: Self = MyField(0);
    }

    impl One for MyField {
        const ONE: Self = MyField(1);
    }

    #[test]
    fn zero_and_one_behave_as_expected_for_custom_type() {
        let z = MyField::zero();
        let o = MyField::one();

        assert!(z.is_zero());
        assert!(o.is_one());
        assert!(!z.is_one());
        assert!(!o.is_zero());
    }

    #[test]
    fn scalar_alias_trait_is_satisfied() {
        fn requires_scalar<S: Scalar>(_x: S) {}

        requires_scalar(MyField::one());
    }
}
