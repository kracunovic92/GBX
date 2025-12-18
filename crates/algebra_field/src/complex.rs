//! Complex “field” implementations based on `num_complex::Complex`.
//!
//! These types satisfy the [`Field`] trait interface, but the
//! algebraic laws (associativity, distributivity, etc.) hold only up to
//! floating-point rounding errors. They are suitable for numerical
//! experimentation, plotting, etc., but **not** for exact Gröbner basis
//! computations.

use algebra_core::{One, TryInverse, Zero};
use core::ops::{Add, Div, Mul, Neg, Sub};
use num_complex::Complex;

/// Single-precision complex numbers, modeled as a “numeric field”.
///
/// Internally wraps `num_complex::Complex<f32>`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct C32(pub Complex<f32>);

/// Double-precision complex numbers, modeled as a “numeric field”.
///
/// Internally wraps `num_complex::Complex<f64>`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct C64(pub Complex<f64>);

/* ===== Conversions ===== */

impl From<Complex<f32>> for C32 {
    #[inline]
    fn from(z: Complex<f32>) -> Self {
        Self(z)
    }
}

impl From<C32> for Complex<f32> {
    #[inline]
    fn from(z: C32) -> Self {
        z.0
    }
}

impl From<Complex<f64>> for C64 {
    #[inline]
    fn from(z: Complex<f64>) -> Self {
        Self(z)
    }
}

impl From<C64> for Complex<f64> {
    #[inline]
    fn from(z: C64) -> Self {
        z.0
    }
}

impl Zero for C32 {
    const ZERO: Self = C32(Complex { re: 0.0, im: 0.0 });
}

impl One for C32 {
    const ONE: Self = C32(Complex { re: 1.0, im: 0.0 });
}

impl Zero for C64 {
    const ZERO: Self = C64(Complex { re: 0.0, im: 0.0 });
}

impl One for C64 {
    const ONE: Self = C64(Complex { re: 1.0, im: 0.0 });
}

/* ===== Arithmetic ops delegating to inner Complex<T> ===== */

impl Add for C32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        C32(self.0 + rhs.0)
    }
}

impl Sub for C32 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        C32(self.0 - rhs.0)
    }
}

impl Mul for C32 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        C32(self.0 * rhs.0)
    }
}

impl Div for C32 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        C32(self.0 / rhs.0)
    }
}

impl Neg for C32 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        C32(-self.0)
    }
}

impl Add for C64 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        C64(self.0 + rhs.0)
    }
}

impl Sub for C64 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        C64(self.0 - rhs.0)
    }
}

impl Mul for C64 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        C64(self.0 * rhs.0)
    }
}

impl Div for C64 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        C64(self.0 / rhs.0)
    }
}

impl Neg for C64 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        C64(-self.0)
    }
}

macro_rules! impl_try_inverse_for_complex {
    ($t:ty) => {
        impl TryInverse for $t {
            type Output = Self;

            #[inline]
            fn try_inv(self) -> Option<Self::Output> {
                if self == Self::ZERO { None } else { Some(Self::ONE / self) }
            }
        }
    };
}

impl_try_inverse_for_complex!(C32);
impl_try_inverse_for_complex!(C64);

#[cfg(test)]
mod tests {
    use super::*;
    use algebra_core::{CheckedDiv, Scalar};

    #[test]
    fn identities_c64() {
        let z = C64::zero();
        let o = C64::one();

        assert_eq!(z, C64(Complex::new(0.0, 0.0)));
        assert_eq!(o, C64(Complex::new(1.0, 0.0)));

        assert!(z.is_zero());
        assert!(o.is_one());
        assert!(!z.is_one());
        assert!(!o.is_zero());
    }

    #[test]
    fn inversion_c64() {
        let z = C64::zero();
        assert!(
            z.try_inv()
                .is_none()
        );

        let a = C64(Complex::new(1.0, 2.0));
        let inv = a
            .try_inv()
            .unwrap();

        let prod = a * inv;
        let diff = Complex::<f64>::from(prod) - Complex::new(1.0, 0.0);
        let eps = 1e-10;
        assert!(diff.norm() < eps);
    }

    #[test]
    fn checked_div_c64() {
        let a = C64(Complex::new(2.0, 3.0));
        let b = C64(Complex::new(-1.0, 4.0));

        let q = a
            .checked_div(b)
            .unwrap();
        let back = q * b;

        // a / b * b ≈ a
        let diff = Complex::<f64>::from(back) - Complex::from(a);
        let eps = 1e-10;
        assert!(diff.norm() < eps);

        let z = C64::zero();
        assert!(
            a.checked_div(z)
                .is_err()
        );
    }

    #[test]
    fn scalar_alias_applies() {
        fn takes_scalar<S: Scalar>(_x: S) {}

        takes_scalar(C32::one());
        takes_scalar(C64::one());
    }
}
