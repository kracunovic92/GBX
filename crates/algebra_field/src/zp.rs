use algebra_core::{One, TryInverse, Zero};
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[cfg(feature = "validate-modulus")]
use std::sync::OnceLock;

/// Finite field-like type `Zp<P>` representing integers modulo a type-level modulus `P`.
///
/// - For **prime** `P`, `Zp<P>` is a field: every non-zero element has a multiplicative inverse
///   and the type satisfies the usual field axioms (associativity, commutativity, distributivity).
/// - For **non-prime** `P`, `Zp<P>` is only a (commutative) ring; in that case
///   [`TryInverse::try_inv`] returns `None` for non-invertible elements.
///
/// All values are stored in reduced form in the interval `[0, P)`. Arithmetic is performed
/// modulo `P`.
///
/// When the `validate-modulus` feature is enabled, the first use of a concrete `Zp<P>`
/// will validate at runtime that `P` is prime and `P >= 2`, panicking otherwise.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Zp<const P: u64>(u64);

impl<const P: u64> Zp<P> {
    /// Returns the modulus `P`.
    #[must_use]
    #[inline]
    pub const fn modulus() -> u64 {
        P
    }

    /// Constructs an element `x (mod P)` reduced into `[0, P)`.
    ///
    /// In debug builds, asserts `P >= 2`.
    /// With `validate-modulus` enabled, performs a one-time primality check.
    #[cfg(not(feature = "validate-modulus"))]
    #[must_use]
    #[inline]
    pub const fn new(x: u64) -> Self {
        debug_assert!(
            P >= 2,
            "Zp<P>: modulus P must be greater than or equal to 2 (a prime)."
        );
        Self(x % P)
    }

    /// `new` with runtime modulus validation when the feature is enabled.
    #[cfg(feature = "validate-modulus")]
    #[must_use]
    #[inline]
    pub fn new(x: u64) -> Self {
        validate_modulus_once::<P>();
        Self(x % P)
    }

    /// Non-const constructor that always takes the “checked” path.
    ///
    /// In debug builds it asserts `P >= 2`. With `validate-modulus` it always
    /// reuses the cached primality check.
    #[must_use]
    #[inline]
    pub fn new_checked(x: u64) -> Self {
        debug_assert!(
            P >= 2,
            "Zp<P>: modulus P must be greater than or equal to 2 (a prime)."
        );
        #[cfg(feature = "validate-modulus")]
        validate_modulus_once::<P>();
        Self(x % P)
    }

    /// Constructs from an already-reduced representative.
    ///
    /// # Panics (debug only)
    ///
    /// Panics in debug builds if `x >= P`. Use only on trusted inputs / fast paths.
    #[must_use]
    #[inline]
    pub const fn from_reduced_unchecked(x: u64) -> Self {
        debug_assert!(x < P, "from_reduced_unchecked: x must be < P");
        Self(x)
    }

    /// Returns the canonical representative in `[0, P)`.
    #[must_use]
    #[inline]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Computes `self^e (mod P)` using binary exponentiation.
    #[must_use]
    #[inline]
    pub fn pow(self, mut e: u128) -> Self {
        let (mut base, mut acc) = (self, Self::one());
        while e > 0 {
            if e & 1 == 1 {
                acc *= base;
            }
            base *= base;
            e >>= 1;
        }
        acc
    }

    /// Computes `self^e` for signed exponents.
    ///
    /// - If `e >= 0`, this is just `self.pow(e as u128)`.
    /// - If `e < 0`, we first try to invert `self`; if it is invertible,
    ///   we then exponentiate the inverse.
    #[must_use]
    #[inline]
    pub fn pow_signed(self, e: i128) -> Option<Self> {
        let abs = e.unsigned_abs();
        if e >= 0 {
            Some(self.pow(abs))
        } else {
            self.try_inv()
                .map(|inv| inv.pow(abs))
        }
    }

    /// Convenience: multiplicative inverse assuming a valid field modulus (prime `P`).
    ///
    /// Returns `None` for `0` or for non-invertible elements when `P` is composite.
    #[must_use]
    #[inline]
    pub fn inv(self) -> Option<Self> {
        <Self as TryInverse>::try_inv(self)
    }

    /// Iterates all reduced residues `0..P` as `Zp<P>`.
    ///
    /// Mostly useful in tests and exhaustive checks for small primes.
    #[must_use]
    pub fn iter_all() -> impl Iterator<Item = Self> {
        (0..P).map(Self::from_reduced_unchecked)
    }
}

/* ===== Identities (Zero, One, Default) ===== */

impl<const P: u64> Zero for Zp<P> {
    const ZERO: Self = Self(0);
}

impl<const P: u64> One for Zp<P> {
    const ONE: Self = Self(1);
}

impl<const P: u64> Default for Zp<P> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

/* ===== Operator traits (Add, Sub, Mul, Neg, Div) ===== */

impl<const P: u64> Add for Zp<P> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(self.0 < P && rhs.0 < P);
        let (s, carry) = self
            .0
            .overflowing_add(rhs.0);
        let s = if carry || s >= P { s.wrapping_sub(P) } else { s };
        Self(s)
    }
}

impl<const P: u64> Sub for Zp<P> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<const P: u64> Neg for Zp<P> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        debug_assert!(self.0 < P);
        if self.0 == 0 { self } else { Self(P - self.0) }
    }
}

impl<const P: u64> Mul for Zp<P> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert!(self.0 < P && rhs.0 < P);
        let prod = u128::from(self.0) * u128::from(rhs.0);
        let red = prod % u128::from(P);
        #[allow(clippy::cast_possible_truncation)]
        let v = red as u64;
        Self(v)
    }
}

impl<const P: u64> Div for Zp<P> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let inv = rhs
            .try_inv()
            .unwrap_or_else(|| panic!("division by zero in Zp<{P}>"));
        self * inv
    }
}

/* ===== TryInverse (from algebra_core) ===== */

impl<const P: u64> TryInverse for Zp<P> {
    type Output = Self;

    #[inline]
    fn try_inv(self) -> Option<Self::Output> {
        if self.0 == 0 {
            return None;
        }

        // Extended Euclidean algorithm on (self, P).
        let (mut a, mut b) = (i128::from(self.0), i128::from(P));
        let (mut x0, mut x1) = (1i128, 0i128);

        while b != 0 {
            let q = a / b;
            (a, b) = (b, a - q * b);
            (x0, x1) = (x1, x0 - q * x1);
        }

        if a != 1 {
            // Not invertible modulo P (gcd != 1).
            return None;
        }

        let inv = ((x0 % i128::from(P)) + i128::from(P)) % i128::from(P);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Some(Self(inv as u64))
    }
}

/* ===== Ergonomics: Assign ops, iter adapters, conversions ===== */

impl<const P: u64> AddAssign for Zp<P> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const P: u64> MulAssign for Zp<P> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const P: u64> DivAssign for Zp<P> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const P: u64> Sum for Zp<P> {
    fn sum<I: Iterator<Item = Self>>(i: I) -> Self {
        let mut acc = Self::zero();
        for x in i {
            acc += x;
        }
        acc
    }
}

impl<'a, const P: u64> Sum<&'a Self> for Zp<P> {
    fn sum<I: Iterator<Item = &'a Self>>(i: I) -> Self {
        let mut acc = Self::zero();
        for &x in i {
            acc += x;
        }
        acc
    }
}

impl<const P: u64> Product for Zp<P> {
    fn product<I: Iterator<Item = Self>>(i: I) -> Self {
        let mut acc = Self::one();
        for x in i {
            acc *= x;
        }
        acc
    }
}

impl<const P: u64> From<u64> for Zp<P> {
    #[inline]
    fn from(x: u64) -> Self {
        Self::new(x)
    }
}

impl<const P: u64> From<Zp<P>> for u64 {
    #[inline]
    fn from(z: Zp<P>) -> Self {
        z.value()
    }
}

/* ===== Formatting ===== */

impl<const P: u64> fmt::Debug for Zp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Zp<{}>({})", P, self.0)
    }
}

impl<const P: u64> fmt::Display for Zp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/* ===== Modulus validation (feature = "validate-modulus") ===== */

#[cfg(feature = "validate-modulus")]
#[inline]
fn validate_modulus_once<const P: u64>() {
    // One cache per concrete P (monomorphization gives each P its own static).
    static OK: OnceLock<bool> = OnceLock::new();
    let ok = OK.get_or_init(|| P >= 2 && is_prime_u64(P));
    assert!(*ok, "Zp<P>: invalid modulus P={P} (must be prime and >= 2)");
}

#[cfg(feature = "validate-modulus")]
#[inline]
const fn mul_mod_u128(a: u128, b: u128, m: u128) -> u128 {
    ((a % m) * (b % m)) % m
}

#[cfg(feature = "validate-modulus")]
const fn pow_mod_u128(mut a: u128, mut e: u128, m: u128) -> u128 {
    let mut acc = 1u128 % m;
    while e > 0 {
        if e & 1 == 1 {
            acc = mul_mod_u128(acc, a, m);
        }
        a = mul_mod_u128(a, a, m);
        e >>= 1;
    }
    acc
}

/// Deterministic Miller–Rabin for all 64-bit integers.
/// Uses the well-known base set {2, 3, 5, 7, 11, 13, 17}.
#[cfg(feature = "validate-modulus")]
fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for p in [2u64, 3, 5, 7, 11, 13, 17] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return n == p;
        }
    }

    // Write n-1 = d * 2^s with d odd
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];
    let n128 = u128::from(n);
    let d128 = u128::from(d);

    'outer: for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x: u128 = pow_mod_u128(u128::from(a), d128, n128);
        if x == 1 || x == n128 - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul_mod_u128(x, x, n128);
            if x == n128 - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use algebra_core::{CheckedDiv, One, Zero};

    type F7 = Zp<7>;

    #[test]
    fn identities() {
        let z = F7::zero();
        let o = F7::one();
        assert_eq!(z.value(), 0);
        assert_eq!(o.value(), 1);
    }

    #[test]
    fn basic_arithmetic_mod_7() {
        let a = F7::new(5);
        let b = F7::new(3);

        // 5 + 3 = 8 ≡ 1 (mod 7)
        assert_eq!((a + b).value(), 1);

        // 5 * 3 = 15 ≡ 1 (mod 7)
        assert_eq!((a * b).value(), 1);

        // -5 ≡ 2 (mod 7)
        assert_eq!((-a).value(), 2);
    }

    #[test]
    fn inverse_and_checked_div() {
        let a = F7::new(5);
        let b = F7::new(3);

        // 5 * 3 = 15 ≡ 1 (mod 7)
        assert_eq!((a * b).value(), 1);

        let inv_a = a
            .try_inv()
            .unwrap();
        assert_eq!(inv_a.value(), 3);
        assert_eq!((a * inv_a).value(), 1);

        let q = a
            .checked_div(b)
            .unwrap();
        assert_eq!((q * b).value(), a.value());

        assert!(
            F7::zero()
                .try_inv()
                .is_none()
        );
        let z_over_b = F7::zero()
            .checked_div(F7::new(3))
            .expect("0 / nonzero should succeed");
        assert_eq!(z_over_b.value(), 0);

        assert!(
            F7::new(3)
                .checked_div(F7::zero())
                .is_err()
        );
    }

    #[test]
    fn field_axioms_hold_for_f7() {
        type F = Zp<7>;
        let elems: Vec<F> = F::iter_all().collect();
        let zero = F::zero();
        let one = F::one();

        // Addition: group laws + commutativity
        for &a in &elems {
            for &b in &elems {
                for &c in &elems {
                    assert_eq!((a + b) + c, a + (b + c));
                }
                assert_eq!(a + b, b + a);
            }
            assert_eq!(a + zero, a);
            assert_eq!(zero + a, a);
            assert_eq!(a + (-a), zero);
        }

        // Multiplication: monoid + distributivity
        for &a in &elems {
            for &b in &elems {
                for &c in &elems {
                    assert_eq!((a * b) * c, a * (b * c));
                    assert_eq!(a * (b + c), a * b + a * c);
                    assert_eq!((a + b) * c, a * c + b * c);
                }
                assert_eq!(a * one, a);
                assert_eq!(one * a, a);
            }
        }

        // Inverses for all non-zero elements
        for &a in &elems {
            if a.is_zero() {
                continue;
            }
            let inv = a
                .try_inv()
                .unwrap();
            assert_eq!(a * inv, one);
            assert_eq!(inv * a, one);
        }
    }
}
