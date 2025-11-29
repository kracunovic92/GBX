//! Dynamic (runtime-sized) monomial representation.
//!
//! This module defines [`DynamicMonomial`], a monomial type where the number
//! of variables is chosen at runtime. It complements the fixed-size
//! [`Monomial<N>`] type and implements the common [`MonomialLike`] trait.

use super::{MonomialError, MonomialLike};
use core::fmt;

/// A monomial in a runtime-chosen number of variables.
///
/// Mathematically, this represents
/// `x₁^{e[0]} * x₂^{e[1]} * … * x_n^{e[n-1]}`
/// with each exponent `e[i] ∈ ℕ`.
///
/// Unlike [`crate::monomial::Monomial<N>`], the number of variables is not
/// known at compile time. It is stored as the length of the exponent slice.
///
/// This type is intended for use in scenarios where the polynomial ring is
/// specified at runtime (e.g. CLI tools, backends that receive input files
/// describing the ring), while [`Monomial<N>`] remains the choice for
/// compile-time–fixed rings and performance-sensitive kernels.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DynamicMonomial {
    exponents: Box<[u32]>,
}

impl DynamicMonomial {
    /// Creates the multiplicative identity monomial `1` in `n_vars` variables,
    /// i.e. all exponents are zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use algebra_poly::monomial::DynamicMonomial;
    ///
    /// let m = DynamicMonomial::one(3);
    /// assert_eq!(m.exponents(), &[0, 0, 0]);
    /// ```
    pub fn one(n_vars: usize) -> Self {
        Self { exponents: vec![0; n_vars].into_boxed_slice() }
    }

    /// Constructs a monomial from a slice of exponents.
    ///
    /// The number of variables equals `exponents.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algebra_poly::monomial::DynamicMonomial;
    ///
    /// let m = DynamicMonomial::from_slice(&[2, 5]);
    /// assert_eq!(m.exponents(), &[2, 5]);
    /// ```
    pub fn from_slice(exponents: &[u32]) -> Self {
        Self {
            exponents: exponents
                .to_vec()
                .into_boxed_slice(),
        }
    }

    /// Constructs a monomial from a `Vec<u32>` of exponents.
    ///
    /// This is a convenience wrapper around [`from_boxed`](Self::from_boxed).
    pub fn from_vec(exponents: Vec<u32>) -> Self {
        Self { exponents: exponents.into_boxed_slice() }
    }

    /// Constructs a monomial from a boxed slice of exponents.
    ///
    /// This is the most general constructor. No invariants except:
    /// - each exponent is interpreted as a non-negative integer.
    pub fn from_boxed(exponents: Box<[u32]>) -> Self {
        Self { exponents }
    }

    /// Number of variables in this monomial.
    ///
    /// This is equal to `self.exponents().len()`.
    #[inline]
    pub fn n_vars(&self) -> usize {
        self.exponents
            .len()
    }

    /// Returns a reference to the underlying exponent vector.
    ///
    /// The length of this slice is the number of variables.
    #[inline]
    pub fn exponents(&self) -> &[u32] {
        &self.exponents
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
    ///
    /// This is a convenience wrapper around [`Self::degree_checked`].
    ///
    /// # Panics
    ///
    /// Panics if the degree would overflow `u64`.
    /// For a non-panicking version, use [`Self::degree_checked`].
    #[inline]
    pub fn degree(&self) -> u64 {
        self.degree_checked()
            .expect("DynamicMonomial::degree overflowed u64")
    }

    /// Total degree with checked addition.
    ///
    /// Returns an error if the sum of exponents would overflow `u64`.
    pub fn degree_checked(&self) -> Result<u64, MonomialError> {
        let mut deg: u64 = 0;
        for &e in self
            .exponents
            .iter()
        {
            deg = deg
                .checked_add(e as u64)
                .ok_or(MonomialError::DegreeOverflow)?;
        }
        Ok(deg)
    }

    /// Checked monomial multiplication.
    ///
    /// Returns an error if:
    /// - exponent addition would overflow `u32`, or
    /// - the monomials have different numbers of variables.
    ///
    /// This is the checked counterpart of [`Self::mul`].
    pub fn checked_mul(&self, other: &Self) -> Result<Self, MonomialError> {
        let lhs_n = self.n_vars();
        let rhs_n = other.n_vars();

        if lhs_n != rhs_n {
            return Err(MonomialError::MismatchedVariableCount { lhs: lhs_n, rhs: rhs_n });
        }

        let mut out = Vec::with_capacity(lhs_n);
        for (i, (&lhs, &rhs)) in self
            .exponents
            .iter()
            .zip(
                other
                    .exponents
                    .iter(),
            )
            .enumerate()
        {
            let sum = lhs
                .checked_add(rhs)
                .ok_or(MonomialError::ExponentOverflow { index: i, lhs, rhs })?;
            out.push(sum);
        }

        Ok(Self { exponents: out.into_boxed_slice() })
    }

    /// Multiplies two monomials: add exponents componentwise.
    ///
    /// `(x^a) * (x^b) = x^{a+b}`.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - exponent addition overflows `u32`, or
    /// - the monomials have different numbers of variables.
    ///
    /// For a non-panicking version, use [`Self::checked_mul`].
    #[inline]
    pub fn mul(&self, other: &Self) -> Self {
        self.checked_mul(other)
            .expect("DynamicMonomial::mul exponent overflow or mismatched variable counts")
    }

    /// Returns `true` if `self` divides `other` as monomials,
    /// i.e. all exponents of `self` are <= corresponding exponents of `other`.
    ///
    /// If the variable counts differ, returns `false`.
    pub fn divides(&self, other: &Self) -> bool {
        if self.n_vars() != other.n_vars() {
            return false;
        }

        self.exponents
            .iter()
            .zip(
                other
                    .exponents
                    .iter(),
            )
            .all(|(&a, &b)| a <= b)
    }

    /// Returns the quotient `other / self` if `self` divides `other`,
    /// otherwise returns `None`.
    ///
    /// When it returns `Some(q)`, one has `q * self = other`.
    pub fn quotient(&self, other: &Self) -> Option<Self> {
        if !self.divides(other) {
            return None;
        }

        let mut out = Vec::with_capacity(self.n_vars());
        for (&a, &b) in self
            .exponents
            .iter()
            .zip(
                other
                    .exponents
                    .iter(),
            )
        {
            // Safe because `a <= b` ensured by `divides`.
            out.push(b - a);
        }

        Some(Self { exponents: out.into_boxed_slice() })
    }

    /// Componentwise least common multiple: `lcm(self, other)`.
    ///
    /// Defined by `lcm(e, f)[i] = max(e[i], f[i])`.
    ///
    /// # Panics
    ///
    /// Panics if the monomials have different numbers of variables.
    pub fn lcm(&self, other: &Self) -> Self {
        let lhs_n = self.n_vars();
        let rhs_n = other.n_vars();

        if lhs_n != rhs_n {
            panic!("DynamicMonomial::lcm called with mismatched variable counts");
        }

        let mut out = Vec::with_capacity(lhs_n);
        for (a, b) in self.zip_exponents(other) {
            out.push(a.max(b)); // <-- FIXED: was min()
        }

        Self { exponents: out.into_boxed_slice() }
    }

    /// Componentwise greatest common divisor: `gcd(self, other)`.
    ///
    /// Defined by `gcd(e, f)[i] = min(e[i], f[i])`.
    ///
    /// # Panics
    ///
    /// Panics if the monomials have different numbers of variables.
    pub fn gcd(&self, other: &Self) -> Self {
        let lhs_n = self.n_vars();
        let rhs_n = other.n_vars();

        if lhs_n != rhs_n {
            panic!("DynamicMonomial::gcd called with mismatched variable counts");
        }

        let mut out = Vec::with_capacity(lhs_n);

        for (a, b) in self.zip_exponents(other) {
            out.push(a.min(b));
        }

        Self { exponents: out.into_boxed_slice() }
    }

    #[inline]
    fn zip_exponents<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = (u32, u32)> + 'a {
        self.exponents
            .iter()
            .copied()
            .zip(
                other
                    .exponents
                    .iter()
                    .copied(),
            )
    }
}

impl fmt::Debug for DynamicMonomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynamicMonomial")
            .field(&self.exponents)
            .finish()
    }
}

/// Implementation of the common monomial interface for dynamic monomials.
///
/// This allows algorithms generic over [`MonomialLike`] to work with
/// [`DynamicMonomial`] just as they do with the fixed-size [`Monomial<N>`].
impl MonomialLike for DynamicMonomial {
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

    fn degree(&self) -> u64
    where
        Self::Error: fmt::Debug,
    {
        self.degree()
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
    use super::{DynamicMonomial as DM, MonomialError, MonomialLike};
    use std::fmt::Debug;

    #[test]
    fn one_is_all_zero_exponents() {
        let m = DM::one(3);
        assert_eq!(m.exponents(), &[0, 0, 0]);
        assert_eq!(m.degree(), 0);
        assert!(m.is_one());
        assert_eq!(m.n_vars(), 3);
    }

    #[test]
    fn from_slice_sets_components_correctly() {
        let m = DM::from_slice(&[2, 5]);
        assert_eq!(m.exponents(), &[2, 5]);
        assert_eq!(m.degree(), 7);
    }

    #[test]
    fn degree_checked_matches_degree() {
        let m = DM::from_slice(&[1, 3, 4]);
        assert_eq!(m.degree(), 8);
        assert_eq!(
            m.degree_checked()
                .unwrap(),
            8
        );
    }

    #[test]
    fn checked_mul_reports_overflow() {
        let max: u32 = u32::MAX;
        let a = DM::from_slice(&[max, 0]);
        let b = DM::from_slice(&[1, 0]);

        let _err = a
            .checked_mul(&b)
            .expect_err("should overflow");
    }

    #[test]
    fn divides_and_quotient() {
        let a = DM::from_slice(&[1, 2, 0]);
        let b = DM::from_slice(&[3, 5, 0]);

        assert!(a.divides(&b));
        assert!(!b.divides(&a));

        let q = a
            .quotient(&b)
            .expect("a should divide b");
        assert_eq!(q.exponents(), &[2, 3, 0]);
    }

    #[test]
    fn lcm_and_gcd() {
        let a = DM::from_slice(&[1, 4]);
        let b = DM::from_slice(&[3, 2]);

        let l = a.lcm(&b);
        let g = a.gcd(&b);

        assert_eq!(l.exponents(), &[3, 4]); // max
        assert_eq!(g.exponents(), &[1, 2]); // min
    }

    #[test]
    fn mismatched_variable_count_checked_mul_errors() {
        let a = DM::from_slice(&[1, 2]);
        let b = DM::from_slice(&[1, 2, 3]);

        let err = a
            .checked_mul(&b)
            .expect_err("mismatch should error");
        match err {
            MonomialError::MismatchedVariableCount { lhs, rhs } => {
                assert_eq!(lhs, 2);
                assert_eq!(rhs, 3);
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn monomial_like_trait_works() {
        let a = DM::from_slice(&[1, 2]);
        let b = DM::from_slice(&[3, 4]);

        // Use via the trait bound

        fn lcm_degree<M>(x: &M, y: &M) -> u64
        where
            M: MonomialLike,
            M::Error: Debug,
        {
            let l = x.lcm(y);
            l.degree()
        }

        let deg = lcm_degree(&a, &b);
        // lcm exponents = [3, 4], degree = 7
        assert_eq!(deg, 7);
    }
}
