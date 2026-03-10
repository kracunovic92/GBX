use core::fmt;
#[cfg(feature = "panic-div")]
use core::ops::Div;
use core::ops::{Add, Mul, Neg, Sub};

use gbx_alg::{One, TryInverse, Zero};

use crate::macros::{impl_assign_ops, impl_sum_product};
use crate::zp::Zp;

#[cfg(feature = "prime-check")]
use super::prime::validate_prime_once;

/// Element of the prime field `𝔽_p` with modulus `P`.
///
/// Internally stored as a reduced `Zp<P>`, but the type conveys the stronger API.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Fp<const P: u32>(pub(crate) Zp<P>);

impl<const P: u32> Fp<P> {
    /// Returns the modulus `P`.
    #[must_use]
    #[inline]
    pub const fn modulus() -> u32 {
        P
    }

    /// Constructs `x (mod P)`.
    ///
    /// If the `prime-check` feature is enabled, validates once (per concrete `P`)
    /// that `P` is prime and at least 2.
    #[must_use]
    #[inline]
    pub fn new(x: u32) -> Self {
        #[cfg(feature = "prime-check")]
        validate_prime_once::<P>();
        Self(Zp::new_checked(x))
    }

    /// Constructs from `u64`, reducing modulo `P`.
    #[must_use]
    #[inline]
    pub fn from_u64(x: u64) -> Self {
        #[cfg(feature = "prime-check")]
        validate_prime_once::<P>();
        Self(Zp::from_u64(x))
    }

    /// Constructs from an already-reduced representative.
    ///
    /// # Debug
    /// Asserts `x < P`.
    #[must_use]
    #[inline]
    pub const fn from_reduced_unchecked(x: u32) -> Self {
        Self(Zp::from_reduced_unchecked(x))
    }

    /// Returns the canonical representative in `[0, P)`.
    #[must_use]
    #[inline]
    pub const fn value(self) -> u32 {
        self.0.value()
    }

    /// Iterates all field elements `0..P`.
    ///
    /// Intended for tests and small primes.
    #[inline]
    pub fn iter_all() -> impl Iterator<Item = Self> {
        (0..P).map(Self::from_reduced_unchecked)
    }
}

/* ===== identities ===== */

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

/* ===== Assign/Sum/Product boilerplate ===== */

// ✅ IMPORTANT: target Fp<P>, not Zp<P>
impl_assign_ops!(<const P: u32> Fp<P>);
impl_sum_product!(<const P: u32> Fp<P>);

/* ===== inversion ===== */

impl<const P: u32> TryInverse for Fp<P> {
    type Output = Self;

    #[inline]
    fn try_inv(self) -> Option<Self::Output> {
        if self.value() == 0 {
            return None;
        }

        // Extended Euclid on (a, P).
        let mut a = self.value() as i64;
        let mut b = P as i64;
        let mut x0: i64 = 1;
        let mut x1: i64 = 0;

        while b != 0 {
            let q = a / b;
            (a, b) = (b, a - q * b);
            (x0, x1) = (x1, x0 - q * x1);
        }

        // In a true prime field gcd must be 1 for all nonzero a.
        if a != 1 {
            return None;
        }

        let inv = x0.rem_euclid(P as i64) as u32;
        Some(Self::from_reduced_unchecked(inv))
    }
}

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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::Fp;
    use gbx_alg::{CheckedDiv, Field, One, TryInverse, Zero};

    #[test]
    fn fp7_inverse_works() {
        type F = Fp<7>;
        let a = F::new(5);
        let inv = a.try_inv().unwrap();
        assert_eq!((a * inv).value(), 1);
    }
    #[test]
    fn fp7_checked_div_nonzero_is_ok() {
        type F = Fp<7>;
        let a = F::new(5);
        let b = F::new(3);

        let q = a
            .checked_div(b)
            .expect("division by nonzero should succeed");
        assert_eq!((q * b).value(), a.value());
    }

    #[test]
    fn fp7_checked_div_by_zero_is_err() {
        type F = Fp<7>;
        let a = F::new(5);
        let z = F::ZERO;

        assert!(matches!(a.checked_div(z), Err(DivByZero)));
    }
    #[test]
    fn fp7_iter_all_has_p_elems() {
        let elems: Vec<Fp<7>> = Fp::<7>::iter_all().collect();
        assert_eq!(elems.len(), 7);
    }

    #[test]
    fn fp_marker_compiles() {
        fn needs_field<T: Field>(_x: T) {}
        needs_field(Fp::<7>::one());
    }

    #[cfg(feature = "panic-div")]
    #[test]
    #[should_panic]
    fn fp7_div_by_zero_panics() {
        type F = Fp<7>;
        let _ = F::new(1) / F::ZERO;
    }
}
