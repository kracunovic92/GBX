use crate::monomial::{MonomialError, MonomialLike};
use core::fmt;
use core::ops::Mul;

/// A monomial in `N` variables, represented by its exponents.
///
/// Mathematically, this corresponds to
/// `x₁^{e[0]} * x₂^{e[1]} * … * x_N^{e[N-1]}`
/// with each exponent `e[i] ∈ ℕ`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Monomial<const N: usize> {
    exponents: [u32; N],
}

impl<const N: usize> Monomial<N> {
    /// The multiplicative identity monomial: all exponents are zero.
    pub const fn one() -> Self {
        Self { exponents: [0; N] }
    }

    /// Constructs a monomial from a full exponent vector.
    ///
    /// No invariants except that each exponent is interpreted as `ℕ`.
    pub const fn from_exponents(exponents: [u32; N]) -> Self {
        Self { exponents }
    }

    /// Returns a reference to the underlying exponent vector.
    #[inline]
    pub fn exponents(&self) -> &[u32; N] {
        &self.exponents
    }

    /// Number of variables (compile-time constant).
    #[inline]
    pub const fn n_vars(&self) -> usize {
        N
    }
    /// Returns `true` if this is the multiplicative identity `1`,
    /// i.e. all exponents are zero.
    #[inline]
    pub fn is_one(&self) -> bool {
        self.exponents
            .iter()
            .all(|&e| e == 0)
    }
    /// Total degree = sum of all exponents.
    #[inline]
    pub fn degree(&self) -> u64 {
        self.exponents
            .iter()
            .map(|&e| e as u64)
            .sum()
    }

    /// Total degree with checked addition.
    ///
    /// Returns an error if the sum of exponents would overflow `u64`.
    #[inline]
    pub fn degree_checked(&self) -> Result<u64, MonomialError> {
        let mut deg: u64 = 0;
        let mut i = 0;
        while i < N {
            let term = self.exponents[i] as u64;
            deg = deg
                .checked_add(term)
                .ok_or(MonomialError::DegreeOverflow)?;
            i += 1;
        }
        Ok(deg)
    }

    /// Multiplies two monomials: add exponents componentwise.
    ///
    /// `(x^a) * (x^b) = x^{a+b}`.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if exponent addition overflows `u32`.
    /// For a non-panicking, checked version, use [`checked_mul`].
    #[inline]
    pub fn mul(&self, other: &Self) -> Self {
        // Fast path: use `checked_mul` and panic with a clear message
        // if overflow occurs. In release, `checked_add` still returns
        // `None` on overflow, so we get a panic instead of silent wrap.
        self.checked_mul(other)
            .expect("Monomial exponent overflow in Monomial::mul")
    }

    /// Checked monomial multiplication.
    ///
    /// Returns an error if any exponent addition would overflow `u32`.
    #[inline]
    pub fn checked_mul(&self, other: &Self) -> Result<Self, MonomialError> {
        let mut out = [0u32; N];
        let mut i = 0;
        while i < N {
            let lhs = self.exponents[i];
            let rhs = other.exponents[i];
            out[i] = lhs
                .checked_add(rhs)
                .ok_or(MonomialError::ExponentOverflow { index: i, lhs, rhs })?;
            i += 1;
        }
        Ok(Self { exponents: out })
    }

    /// Returns `true` if `self` divides `other` as monomials,
    /// i.e. all exponents of `self` are <= corresponding exponents of `other`.
    #[inline]
    pub fn divides(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < N {
            if self.exponents[i] > other.exponents[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Returns the quotient `other / self` if `self` divides `other`,
    /// otherwise returns `None`.
    #[inline]
    pub fn quotient(&self, other: &Self) -> Option<Self> {
        if !self.divides(other) {
            return None;
        }
        let mut out = [0u32; N];
        let mut i = 0;
        while i < N {
            out[i] = other.exponents[i] - self.exponents[i];
            i += 1;
        }
        Some(Self { exponents: out })
    }

    /// Componentwise least common multiple: `lcm(self, other)`.
    ///
    /// Defined by `lcm(e, f)[i] = max(e[i], f[i])`.
    #[inline]
    pub fn lcm(&self, other: &Self) -> Self {
        let mut out = [0u32; N];
        let mut i = 0;
        while i < N {
            let a = self.exponents[i];
            let b = other.exponents[i];
            out[i] = if a > b { a } else { b };
            i += 1;
        }
        Self { exponents: out }
    }

    /// Componentwise greatest common divisor: `gcd(self, other)`.
    ///
    /// Defined by `gcd(e, f)[i] = min(e[i], f[i])`.
    #[inline]
    pub fn gcd(&self, other: &Self) -> Self {
        let mut out = [0u32; N];
        let mut i = 0;
        while i < N {
            let a = self.exponents[i];
            let b = other.exponents[i];
            out[i] = if a < b { a } else { b };
            i += 1;
        }
        Self { exponents: out }
    }
}

impl<const N: usize> fmt::Debug for Monomial<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Monomial")
            .field(&self.exponents)
            .finish()
    }
}

/// Owned × owned multiplication: `a * b`.
impl<const N: usize> Mul for Monomial<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Monomial::mul(&self, &rhs)
    }
}

/// Owned × ref multiplication: `a * &b`.
impl<const N: usize> Mul<&Monomial<N>> for Monomial<N> {
    type Output = Monomial<N>;

    #[inline]
    fn mul(self, rhs: &Monomial<N>) -> Self::Output {
        Monomial::mul(&self, rhs)
    }
}

/// Ref × owned multiplication: `&a * b`.
impl<const N: usize> Mul<Monomial<N>> for &Monomial<N> {
    type Output = Monomial<N>;

    #[inline]
    fn mul(self, rhs: Monomial<N>) -> Self::Output {
        Monomial::mul(self, &rhs)
    }
}

/// Ref × ref multiplication: `&a * &b`.
impl<const N: usize> Mul<&Monomial<N>> for &Monomial<N> {
    type Output = Monomial<N>;

    #[inline]
    fn mul(self, rhs: &Monomial<N>) -> Self::Output {
        Monomial::mul(self, rhs)
    }
}

/// MonomialLike implementation for fixed-size monomials.
impl<const N: usize> MonomialLike for Monomial<N> {
    type Error = MonomialError;

    #[inline]
    fn n_vars(&self) -> usize {
        self.n_vars()
    }

    #[inline]
    fn exponents(&self) -> &[u32] {
        self.exponents()
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.is_one()
    }

    fn degree_checked(&self) -> Result<u64, Self::Error> {
        self.degree_checked()
    }

    fn checked_mul(&self, other: &Self) -> Result<Self, Self::Error> {
        self.checked_mul(other)
    }

    fn divides(&self, other: &Self) -> bool {
        self.divides(other)
    }

    fn quotient(&self, other: &Self) -> Option<Self> {
        self.quotient(other)
    }

    fn lcm(&self, other: &Self) -> Self {
        self.lcm(other)
    }
}

#[cfg(test)]
mod tests {
    use super::{Monomial, MonomialError};

    type M2 = Monomial<2>;
    type M3 = Monomial<3>;

    #[test]
    fn one_is_all_zero_exponents() {
        let m: M3 = Monomial::one();
        assert_eq!(m.exponents(), &[0, 0, 0]);
        assert_eq!(m.degree(), 0);
    }

    #[test]
    fn from_exponents_sets_components_correctly() {
        let m: M2 = Monomial::from_exponents([2, 5]);
        assert_eq!(m.exponents(), &[2, 5]);
        assert_eq!(m.degree(), 7);
    }

    #[test]
    fn degree_is_sum_of_exponents() {
        let m: M3 = Monomial::from_exponents([1, 3, 4]);
        assert_eq!(m.degree(), 8);
        assert_eq!(
            m.degree_checked()
                .unwrap(),
            8
        );
    }

    #[test]
    fn mul_adds_exponents_componentwise() {
        let a: M2 = Monomial::from_exponents([1, 2]);
        let b: M2 = Monomial::from_exponents([3, 4]);

        let c = a.mul(&b);
        assert_eq!(c.exponents(), &[4, 6]);
        assert_eq!(c.degree(), 10);

        let d = a * b;
        assert_eq!(d.exponents(), &[4, 6]);

        let e = &a * &b;
        assert_eq!(e.exponents(), &[4, 6]);
    }

    #[test]
    fn checked_mul_reports_overflow() {
        let max: u32 = u32::MAX;
        let a: M2 = Monomial::from_exponents([max, 0]);
        let b: M2 = Monomial::from_exponents([1, 0]);

        let err = a
            .checked_mul(&b)
            .expect_err("should overflow");
        match err {
            MonomialError::ExponentOverflow { index, lhs, rhs } => {
                assert_eq!(index, 0);
                assert_eq!(lhs, max);
                assert_eq!(rhs, 1);
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn divides_is_true_if_all_exponents_are_leq() {
        let a: M3 = Monomial::from_exponents([1, 2, 0]);
        let b: M3 = Monomial::from_exponents([3, 2, 5]);

        assert!(a.divides(&b));
        assert!(!b.divides(&a));

        let c: M3 = Monomial::from_exponents([2, 2, 2]);
        let d: M3 = Monomial::from_exponents([2, 2, 2]);
        assert!(c.divides(&d));
        assert!(d.divides(&c));
    }

    #[test]
    fn quotient_is_none_if_not_divisible() {
        let a: M2 = Monomial::from_exponents([2, 1]);
        let b: M2 = Monomial::from_exponents([1, 0]);
        assert!(
            a.quotient(&b)
                .is_none()
        );
    }

    #[test]
    fn quotient_subtracts_exponents_when_divides() {
        let a: M3 = Monomial::from_exponents([1, 2, 0]);
        let b: M3 = Monomial::from_exponents([3, 5, 0]);

        let q = a
            .quotient(&b)
            .expect("a should divide b");
        assert_eq!(q.exponents(), &[2, 3, 0]);
    }
}
