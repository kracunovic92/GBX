use core::fmt;
#[cfg(feature = "panic-div")]
use core::ops::Div;
use core::ops::{Add, Mul, Neg, Sub};

use gbx_alg::{One, Zero};

use super::Fp;
use crate::macros::{impl_assign_ops, impl_sum_product};
use crate::zp::Zp;

impl<const P: u32> Zero for Fp<P> {
    const ZERO: Self = Self(<Zp<P> as Zero>::ZERO);
}

impl<const P: u32> One for Fp<P> {
    const ONE: Self = Self(<Zp<P> as One>::ONE);
}

impl<const P: u32> Default for Fp<P> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

/* ===== ops (delegate to inner Zp) ===== */

impl<const P: u32> Add for Fp<P> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<const P: u32> Sub for Fp<P> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<const P: u32> Neg for Fp<P> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<const P: u32> Mul for Fp<P> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

/* ===== shared boilerplate ===== */

impl_assign_ops!(<const P: u32> Zp<P>);
impl_sum_product!(<const P: u32> Zp<P>);

/* ===== optional panic division ===== */

#[cfg(feature = "panic-div")]
impl<const P: u32> Div for Fp<P> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        use gbx_alg::CheckedDiv;
        self.checked_div(rhs)
            .unwrap_or_else(|_| panic!("division by zero in Fp<{P}>"))
    }
}

/* ===== formatting ===== */

impl<const P: u32> fmt::Debug for Fp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fp<{}>({})", P, self.value())
    }
}

impl<const P: u32> fmt::Display for Fp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
