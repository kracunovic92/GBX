//! Identity elements for algebraic structures.
//!
//! This module defines two tiny, orthogonal traits:
//! - [`Zero`]: additive identity
//! - [`One`]:  multiplicative identity

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

macro_rules! impl_zero_one_for_ints {
    ($($t:ty),* $(,)?) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0;
            }

            impl One for $t {
                const ONE: Self = 1;
            }
        )*
    };
}

impl_zero_one_for_ints!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,
);

#[cfg(test)]
mod unit_tests {
    use super::*;
    use core::fmt::Debug;

    fn check_identities<T>(z: T, o: T)
    where
        T: Zero + One + Copy + PartialEq + Debug,
    {
        assert_eq!(T::zero(), z);
        assert_eq!(T::one(), o);
        assert!(z.is_zero());
        assert!(!o.is_zero());
        assert!(o.is_one());
        assert!(!z.is_one());
    }

    #[test]
    fn integers_have_zero_and_one() {
        check_identities::<u32>(0, 1);
        check_identities::<i64>(0, 1);
        check_identities::<usize>(0, 1);
    }
}
