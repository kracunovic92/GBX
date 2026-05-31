//! Fields: commutative rings where every nonzero element has a multiplicative inverse.
//!
//! Inverses are partial (undefined at `0`), so we expose a fallible API
//! via [`TryInverse`] and [`CheckedDiv`].

use crate::{MulAbelianMonoid, Multiplicative, Ring};
use core::fmt;

/// Error returned by checked division when dividing by zero (or non-invertible element).
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

/// Try to get the multiplicative inverse. Returns `None` for non-invertible elements.
///
/// For a true field, the only non-invertible element is `0`.
pub trait TryInverse: Sized {
    /// The inversion result type. Usually `Self`.
    type Output;

    /// Returns the multiplicative inverse of `self`, or `None` if it doesn’t exist.
    #[must_use]
    fn try_inv(self) -> Option<Self::Output>;
}

/// Helper for checked division built atop [`TryInverse`].
pub trait CheckedDiv: Sized {
    /// Division that checks for non-invertible divisor.
    ///
    /// # Errors
    /// Returns `Err(DivByZero)` if `rhs` has no inverse (e.g. is zero in a field).
    fn checked_div(self, rhs: Self) -> Result<Self, DivByZero>;
}

/// Blanket impl: if you can invert and multiply, you can checked-divide.
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

/// A field is a commutative ring where every nonzero element is invertible.
///
/// This trait is marker-only; algorithms typically use bounds like `T: Field`.
pub trait Field: Ring + MulAbelianMonoid + TryInverse<Output = Self> + CheckedDiv {}

impl<T> Field for T where T: Ring + MulAbelianMonoid + TryInverse<Output = T> + CheckedDiv {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{One, Zero};

    /// Tiny toy field Z/7Z just for testing `TryInverse` / `CheckedDiv`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Mod7(u8);

    impl Mod7 {
        fn new(n: u8) -> Self {
            Self(n % 7)
        }
    }

    impl core::ops::Add for Mod7 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.0 + rhs.0)
        }
    }

    impl core::ops::Neg for Mod7 {
        type Output = Self;
        fn neg(self) -> Self::Output {
            // in mod 7: -a == 7-a (except 0)
            if self.0 == 0 { self } else { Self::new(7 - self.0) }
        }
    }

    impl core::ops::Mul for Mod7 {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(self.0 * rhs.0)
        }
    }

    impl Zero for Mod7 {
        const ZERO: Self = Self(0);
    }

    impl One for Mod7 {
        const ONE: Self = Self(1);
    }

    impl TryInverse for Mod7 {
        type Output = Self;

        fn try_inv(self) -> Option<Self::Output> {
            if self.0 == 0 {
                return None;
            }
            for x in 1u8..7 {
                if (self * Self::new(x)).0 == 1 {
                    return Some(Self::new(x));
                }
            }
            None
        }
    }

    #[test]
    fn div_by_zero_display() {
        assert_eq!(format!("{DivByZero}"), "division by zero");
    }

    #[test]
    fn try_inverse_none_for_zero_some_for_nonzero() {
        assert!(Mod7::new(0).try_inv().is_none());
        assert!(Mod7::new(3).try_inv().is_some());
    }

    #[test]
    fn checked_div_nonzero_denominator_is_ok() {
        let a = Mod7::new(5);
        let b = Mod7::new(3);

        let q = match a.checked_div(b) {
            Ok(q) => q,
            Err(err) => panic!("division by nonzero should succeed: {err}"),
        };
        assert_eq!(q * b, a);
    }

    #[test]
    fn checked_div_by_zero_is_error() {
        let a = Mod7::new(5);
        let z = Mod7::new(0);

        assert!(matches!(a.checked_div(z), Err(DivByZero)));
    }

    #[test]
    fn field_marker_compiles() {
        fn requires_field<F: Field>(_x: F) {}
        requires_field(Mod7::new(1));
    }
}
