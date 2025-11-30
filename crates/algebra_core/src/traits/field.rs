#![allow(clippy::needless_path_prefix)]
//! Fields: rings where every nonzero element has a multiplicative inverse.
//!
//! Inverses are partial (undefined at `0`), so we expose a fallible API
//! via [`TryInverse`] and [`CheckedDiv`].

use core::fmt;

use crate::{Multiplicative, Ring};

/// Error returned by checked division when dividing by zero.
///
/// This type is intentionally tiny and `Copy`, so it is cheap to propagate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DivByZero;

impl fmt::Display for DivByZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("division by zero")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DivByZero {}

/// Try to get the multiplicative inverse. Returns `None` for zero.
///
/// This trait models the fact that in a field, every non-zero element has a
/// multiplicative inverse, but `0` does not.
pub trait TryInverse: Sized {
    /// The result type of inversion; typically `Self`.
    ///
    /// Some abstractions expose an associated `Output` to allow
    /// widening/narrowing (e.g., `&T` → owned `T`), but most
    /// implementations set `Output = Self`.
    type Output;

    /// Returns the multiplicative inverse of `self`, or `None` if it doesn’t exist.
    ///
    /// For fields, every non-zero element has an inverse; zero returns `None`.
    #[must_use]
    fn try_inv(self) -> Option<Self::Output>;
}

/// Helper for checked division built atop [`TryInverse`].
///
/// This is provided as a blanket impl for any type where
/// `TryInverse::Output = Self` and which supports multiplication via
/// [`Multiplicative`].
pub trait CheckedDiv: Sized {
    /// Division that checks for zero divisor.
    ///
    /// # Errors
    ///
    /// Returns `Err(DivByZero)` if `rhs` has no inverse (e.g., is zero in a field).
    fn checked_div(self, rhs: Self) -> Result<Self, DivByZero>;
}

impl<T> CheckedDiv for T
where
    T: TryInverse<Output = T> + Multiplicative,
{
    #[inline]
    fn checked_div(self, rhs: Self) -> Result<Self, DivByZero> {
        rhs.try_inv()
            .map(|inv| core::ops::Mul::mul(self, inv))
            .ok_or(DivByZero)
    }
}

/// A field is a ring where every nonzero element is invertible.
///
/// This trait is marker-only; algorithms typically use bounds like `T: Field`
/// (optionally plus additional constraints such as `Copy`, `Eq`, etc.).
pub trait Field: Ring + TryInverse<Output = Self> {}

/// Blanket impl: any type that is a ring and implements [`TryInverse`] with
/// `Output = Self` is considered a field by convention.
impl<T> Field for T where T: Ring + TryInverse<Output = T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_by_zero_error_is_copy_and_display() {
        let e = DivByZero;
        assert_eq!(format!("{e}"), "division by zero");
        let _copy = e;
    }

    /// A tiny toy "field-like" type modulo 7, only for testing TryInverse / CheckedDiv.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Mod7(u8);

    impl Mod7 {
        fn new(n: u8) -> Self {
            Self(n % 7)
        }

        fn value(self) -> u8 {
            self.0
        }
    }

    impl core::ops::Mul for Mod7 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Mod7::new(self.0 * rhs.0)
        }
    }

    impl crate::One for Mod7 {
        const ONE: Self = Mod7(1);
    }

    impl TryInverse for Mod7 {
        type Output = Self;

        fn try_inv(self) -> Option<Self::Output> {
            if self.0 == 0 {
                return None;
            }

            for x in 1u8..7 {
                if Mod7::new(self.0 * x).0 == 1 {
                    return Some(Mod7::new(x));
                }
            }
            None
        }
    }

    #[test]
    fn try_inverse_none_for_zero_some_for_nonzero() {
        let zero = Mod7::new(0);
        let three = Mod7::new(3);

        assert!(
            zero.try_inv()
                .is_none()
        );
        assert!(
            three
                .try_inv()
                .is_some()
        );
    }

    #[test]
    fn checked_div_nonzero_denominator_is_ok() {
        let a = Mod7::new(5);
        let b = Mod7::new(3);

        // 5 * 3 = 15 ≡ 1 (mod 7)
        let prod = a * b;
        assert_eq!(prod.value(), 1);

        // a / b = q, so q * b = a
        let q = a
            .checked_div(b)
            .expect("division by nonzero should succeed");
        assert_eq!((q * b).value(), a.value());
    }

    #[test]
    fn checked_div_zero_numerator_is_zero() {
        let zero = Mod7::new(0);
        let b = Mod7::new(3);

        let q = zero
            .checked_div(b)
            .expect("0 / nonzero should succeed");
        assert_eq!(q.value(), 0);
    }

    #[test]
    fn checked_div_by_zero_is_error() {
        let a = Mod7::new(5);
        let zero = Mod7::new(0);

        let res = a.checked_div(zero);
        assert!(res.is_err(), "division by zero must error");
        assert!(matches!(res, Err(DivByZero)));
    }
}
