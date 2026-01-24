#![allow(unused_macros)]

macro_rules! impl_assign_ops {
    // const-generic case
    (<const $P:ident : $Ty:ty> $T:ty) => {
        impl<const $P: $Ty> core::ops::AddAssign for $T {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl<const $P: $Ty> core::ops::SubAssign for $T {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl<const $P: $Ty> core::ops::MulAssign for $T {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
    };

    // non-generic fallback
    ($T:ty) => {
        impl core::ops::AddAssign for $T {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl core::ops::SubAssign for $T {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl core::ops::MulAssign for $T {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
    };
}

macro_rules! impl_sum_product {
    // const-generic case
    (<const $P:ident : $Ty:ty> $T:ty) => {
        impl<const $P: $Ty> core::iter::Sum for $T {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                let mut acc = <Self as gbx_alg::Zero>::ZERO;
                for x in iter {
                    acc += x;
                }
                acc
            }
        }

        impl<'a, const $P: $Ty> core::iter::Sum<&'a Self> for $T {
            #[inline]
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                let mut acc = <Self as gbx_alg::Zero>::ZERO;
                for &x in iter {
                    acc += x;
                }
                acc
            }
        }

        impl<const $P: $Ty> core::iter::Product for $T {
            #[inline]
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                let mut acc = <Self as gbx_alg::One>::ONE;
                for x in iter {
                    acc *= x;
                }
                acc
            }
        }

        impl<'a, const $P: $Ty> core::iter::Product<&'a Self> for $T {
            #[inline]
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                let mut acc = <Self as gbx_alg::One>::ONE;
                for &x in iter {
                    acc *= x;
                }
                acc
            }
        }
    };

    // non-generic fallback
    ($T:ty) => {
        impl core::iter::Sum for $T {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                let mut acc = <Self as gbx_alg::Zero>::ZERO;
                for x in iter {
                    acc += x;
                }
                acc
            }
        }

        impl<'a> core::iter::Sum<&'a Self> for $T {
            #[inline]
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                let mut acc = <Self as gbx_alg::Zero>::ZERO;
                for &x in iter {
                    acc += x;
                }
                acc
            }
        }

        impl core::iter::Product for $T {
            #[inline]
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                let mut acc = <Self as gbx_alg::One>::ONE;
                for x in iter {
                    acc *= x;
                }
                acc
            }
        }

        impl<'a> core::iter::Product<&'a Self> for $T {
            #[inline]
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                let mut acc = <Self as gbx_alg::One>::ONE;
                for &x in iter {
                    acc *= x;
                }
                acc
            }
        }
    };
}

pub(crate) use {impl_assign_ops, impl_sum_product};
