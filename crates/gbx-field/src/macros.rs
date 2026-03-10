//! Internal macros for reducing boilerplate.
//
// Not public API. Intentionally crate-private.

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

#[cfg(test)]
mod tests {
    use gbx_alg::{One, Zero};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct M(u32);

    impl Zero for M {
        const ZERO: Self = Self(0);
    }
    impl One for M {
        const ONE: Self = Self(1);
    }

    impl core::ops::Add for M {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }
    impl core::ops::Sub for M {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.0 - rhs.0)
        }
    }
    impl core::ops::Mul for M {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }

    // Use the non-generic fallback in tests.
    impl_assign_ops!(M);
    impl_sum_product!(M);

    #[test]
    fn assign_ops_work() {
        let mut x = M(3);
        x += M(4);
        x *= M(2);
        x -= M(1);
        assert_eq!(x, M(13)); // ((3+4)*2)-1
    }

    #[test]
    fn sum_and_product_work() {
        let xs = [M(2), M(3), M(4)];
        let s: M = xs.into_iter().sum();
        let p: M = xs.into_iter().product();
        assert_eq!(s, M(9));
        assert_eq!(p, M(24));
    }
}
