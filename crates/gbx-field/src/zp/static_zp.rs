use core::fmt;
use core::ops::{Add, Mul, Neg, Sub};

use gbx_alg::{One, Zero};

use crate::macros::{impl_assign_ops, impl_sum_product};

/// Integers modulo a type-level modulus `P`.
///
/// Stored in canonical reduced form: `0 <= value < P`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Zp<const P: u32>(pub(crate) u32);

impl<const P: u32> Zp<P> {
    /// The modulus `P`.
    #[must_use]
    #[inline]
    pub const fn modulus() -> u32 {
        P
    }

    /// Construct `x (mod P)` reduced to `[0, P)`.
    ///
    /// # Debug
    /// Asserts `P >= 2`.
    #[must_use]
    #[inline]
    pub const fn new(x: u32) -> Self {
        debug_assert!(P >= 2, "Zp<P>: modulus must be >= 2");
        Self(x % P)
    }

    /// Non-const constructor (API convenience).
    #[must_use]
    #[inline]
    pub fn new_checked(x: u32) -> Self {
        Self::new(x)
    }

    /// Construct from `u64`, reduced mod `P`.
    #[must_use]
    #[inline]
    pub fn from_u64(x: u64) -> Self {
        debug_assert!(P >= 2, "Zp<P>: modulus must be >= 2");
        Self((x % (P as u64)) as u32)
    }

    /// Construct from already-reduced representative.
    ///
    /// # Debug
    /// Asserts `P >= 2` and `x < P`.
    #[must_use]
    #[inline]
    pub const fn from_reduced_unchecked(x: u32) -> Self {
        debug_assert!(P >= 2, "Zp<P>: modulus must be >= 2");
        debug_assert!(x < P, "Zp::from_reduced_unchecked: x must be < P");
        Self(x)
    }

    /// Canonical representative in `[0, P)`.
    #[must_use]
    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Exponentiation mod `P` using binary exponentiation.
    ///
    /// Defined for all exponents, including `0^0 == 1` by convention.
    #[must_use]
    #[inline]
    pub fn pow(self, mut e: u128) -> Self {
        let mut acc = <Self as One>::ONE;
        if e == 0 {
            return acc;
        }

        let mut base = self;
        while e != 0 {
            if (e & 1) != 0 {
                acc *= base;
            }
            e >>= 1;
            if e != 0 {
                base *= base;
            }
        }
        acc
    }

    /// Iterate all residues `0..P` (useful for small-modulus tests).
    #[inline]
    pub fn iter_all() -> impl Iterator<Item = Self> {
        (0..P).map(Self::from_reduced_unchecked)
    }
}

/* ===== fmt ===== */

impl<const P: u32> fmt::Debug for Zp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Zp<{}>({})", P, self.0)
    }
}

impl<const P: u32> fmt::Display for Zp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/* ===== identities ===== */

impl<const P: u32> Zero for Zp<P> {
    const ZERO: Self = Self(0);
}

impl<const P: u32> One for Zp<P> {
    const ONE: Self = Self(1);
}

impl<const P: u32> Default for Zp<P> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

/* ===== ring ops ===== */

impl<const P: u32> Add for Zp<P> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(P >= 2);
        debug_assert!(self.0 < P && rhs.0 < P);

        let (s, carry) = self.0.overflowing_add(rhs.0);
        let s = if carry || s >= P { s.wrapping_sub(P) } else { s };
        Self(s)
    }
}

impl<const P: u32> Sub for Zp<P> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(P >= 2);
        debug_assert!(self.0 < P && rhs.0 < P);

        if self.0 >= rhs.0 { Self(self.0 - rhs.0) } else { Self(self.0 + (P - rhs.0)) }
    }
}

impl<const P: u32> Neg for Zp<P> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        debug_assert!(P >= 2);
        debug_assert!(self.0 < P);

        if self.0 == 0 { self } else { Self(P - self.0) }
    }
}

impl<const P: u32> Mul for Zp<P> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert!(P >= 2);
        debug_assert!(self.0 < P && rhs.0 < P);

        let prod = (self.0 as u64) * (rhs.0 as u64);
        Self((prod % (P as u64)) as u32)
    }
}

/* ===== shared boilerplate ===== */

impl_assign_ops!(<const P: u32> Zp<P>);
impl_sum_product!(<const P: u32> Zp<P>);

/* ===== conversions ===== */

impl<const P: u32> From<u32> for Zp<P> {
    #[inline]
    fn from(x: u32) -> Self {
        Self::new_checked(x)
    }
}

impl<const P: u32> From<u64> for Zp<P> {
    #[inline]
    fn from(x: u64) -> Self {
        Self::from_u64(x)
    }
}

impl<const P: u32> From<i64> for Zp<P> {
    #[inline]
    fn from(x: i64) -> Self {
        debug_assert!(P >= 2);
        let r = x.rem_euclid(P as i64) as u32;
        Self::from_reduced_unchecked(r)
    }
}

impl<const P: u32> From<Zp<P>> for u32 {
    #[inline]
    fn from(z: Zp<P>) -> Self {
        z.value()
    }
}

#[cfg(test)]
mod tests {
    use super::Zp;

    #[test]
    fn new_reduces() {
        type R = Zp<7>;
        assert_eq!(R::new(0).value(), 0);
        assert_eq!(R::new(6).value(), 6);
        assert_eq!(R::new(7).value(), 0);
        assert_eq!(R::new(8).value(), 1);
    }

    #[test]
    fn add_wraps() {
        type R = Zp<7>;
        assert_eq!((R::new(6) + R::new(6)).value(), 5);
    }

    #[test]
    fn sub_wraps() {
        type R = Zp<7>;
        assert_eq!((R::new(1) - R::new(3)).value(), 5);
    }

    #[test]
    fn neg_works() {
        type R = Zp<7>;
        assert_eq!((-R::new(0)).value(), 0);
        assert_eq!((-R::new(2)).value(), 5);
    }

    #[test]
    fn mul_mods() {
        type R = Zp<7>;
        assert_eq!((R::new(5) * R::new(6)).value(), 2);
    }

    #[test]
    fn pow_basic() {
        type R = Zp<7>;
        assert_eq!(R::new(3).pow(0).value(), 1);
        assert_eq!(R::new(3).pow(1).value(), 3);
        assert_eq!(R::new(3).pow(2).value(), 2); // 9 mod 7
    }

    #[test]
    fn sum_and_product_work() {
        type R = Zp<7>;
        let xs = [R::new(1), R::new(2), R::new(6)];
        let s: R = xs.into_iter().sum();
        let p: R = xs.into_iter().product();
        assert_eq!(s.value(), 2); // 1+2+6=9 mod 7
        assert_eq!(p.value(), 5); // 1*2*6=12 mod 7
    }
}
