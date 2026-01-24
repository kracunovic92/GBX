use core::ops::{Add, Mul, Neg, Sub};

use gbx_alg::{One, Zero};

use super::Zp;
use crate::fp::Fp;
use crate::macros::{impl_assign_ops, impl_sum_product};

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

impl_assign_ops!(<const P: u32> Fp<P>);
impl_sum_product!(<const P: u32> Fp<P>);

/* ===== conversions (optional) ===== */

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
