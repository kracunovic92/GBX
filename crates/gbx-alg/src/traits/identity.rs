//! Identity elements for algebraic structures.
//!
//! This module defines three tiny, orthogonal traits:
//! - [`Zero`]: additive identity
//! - [`One`]:  multiplicative identity
//! - [`Scalar`]: convenience alias for types that have both identities and are cheap scalars.
//!
//! These traits are intentionally minimal. Higher-level traits (rings, fields, polynomials)
//! build on top of them.

use core::cmp::PartialEq;

/// Additive identity: an element `0` such that `0 + a = a = a + 0`.
///
/// Only construction (`zero()`) plus a convenience predicate (`is_zero()`).
///
/// # Examples
/// ```
/// use gbx_alg::Zero;
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
/// Only construction (`one()`) plus a convenience predicate (`is_one()`).
///
/// # Examples
/// ```
/// use gbx_alg::One;
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
/// This is a convenience alias used throughout the ecosystem to shorten
/// generic bounds. It does **not** assume addition/multiplication operations.
///
/// # Examples
/// ```
/// use gbx_alg::{One, Scalar, Zero};
///
/// #[derive(Clone, Copy, PartialEq, Debug)]
/// struct MyScalar(u32);
///
/// impl Zero for MyScalar {
///     const ZERO: Self = MyScalar(0);
/// }
///
/// impl One for MyScalar {
///     const ONE: Self = MyScalar(1);
/// }
///
/// fn takes_scalar<S: Scalar>(_x: S) {}
/// takes_scalar(MyScalar::one());
/// ```
pub trait Scalar: Zero + One + Copy + PartialEq {}

impl<T> Scalar for T where T: Zero + One + Copy + PartialEq {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct MyScalar(u32);

    impl Zero for MyScalar {
        const ZERO: Self = Self(0);
    }

    impl One for MyScalar {
        const ONE: Self = Self(1);
    }

    #[test]
    fn zero_and_one_work() {
        let z = MyScalar::zero();
        let o = MyScalar::one();

        assert!(z.is_zero());
        assert!(o.is_one());
        assert!(!z.is_one());
        assert!(!o.is_zero());
    }

    #[test]
    fn scalar_alias_trait_is_satisfied() {
        fn requires_scalar<S: Scalar>(_x: S) {}
        requires_scalar(MyScalar::one());
    }
}
