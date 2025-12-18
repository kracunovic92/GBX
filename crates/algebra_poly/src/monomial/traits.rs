use core::fmt::Debug;

/// Common interface for both fixed-size and dynamic monomials.
///
/// Algorithms like Buchberger, S-polynomials, or graph encodings
/// can be written generically over `M: MonomialLike`.
///
/// The built-in implementations are:
/// - [`crate::monomial::Monomial<>`] for fixed-size monomials.
/// - [`crate::monomial::DynamicMonomial`] for runtime-sized monomials.
///
/// For both of these, [`Self::Error`] is [`crate::monomial::MonomialError`].
pub trait MonomialLike: Clone + Eq {
    /// Error type for fallible operations (e.g. overflow, mismatched variable counts).
    ///
    /// For the built-in implementations ([`Monomial<N>`] and [`DynamicMonomial`]),
    /// this is [`crate::monomial::MonomialError`].
    type Error;

    /// Number of variables in this monomial.
    ///
    /// This should match the length of the exponent slice returned by [`exponents`].
    fn n_vars(&self) -> usize;

    /// Access to the underlying exponent vector.
    ///
    /// The length of this slice must be equal to `self.n_vars()`.
    fn exponents(&self) -> &[u64];

    /// Returns `true` if this is the multiplicative identity `1`,
    /// i.e. all exponents are zero.
    ///
    /// Implementors may override this with a more efficient check.
    fn is_one(&self) -> bool {
        self.exponents()
            .iter()
            .all(|&e| e == 0)
    }

    /// Total degree with checked addition.
    ///
    /// Returns an error if the sum of exponents would overflow `u64`.
    fn degree_checked(&self) -> Result<u64, Self::Error>;

    /// Total degree = sum of all exponents.
    ///
    /// This is a convenience wrapper around [`degree_checked`].
    ///
    /// # Panics
    ///
    /// Panics if the degree would overflow `u64`. For a non-panicking
    /// version, use [`degree_checked`].
    fn degree(&self) -> u64
    where
        Self::Error: Debug,
    {
        self.degree_checked()
            .expect("MonomialLike::degree overflowed u64")
    }

    /// Checked monomial multiplication.
    ///
    /// Should return an error if exponent addition would overflow `u32`,
    /// or if the number of variables does not match.
    fn checked_mul(&self, other: &Self) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Returns `true` if `self` divides `other` as monomials,
    /// i.e. all exponents of `self` are <= corresponding exponents of `other`.
    ///
    /// The default implementation:
    /// - Returns `false` if the variable counts differ.
    /// - Compares exponents component-wise otherwise.
    fn divides(&self, other: &Self) -> bool {
        if self.n_vars() != other.n_vars() {
            return false;
        }

        self.exponents()
            .iter()
            .zip(
                other
                    .exponents()
                    .iter(),
            )
            .all(|(&a, &b)| a <= b)
    }

    /// Returns the quotient `other / self` if `self` divides `other`,
    /// otherwise returns `None`.
    ///
    /// When it returns `Some(q)`, one should have `q * self = other`
    /// under the chosen multiplication semantics.
    ///
    /// Implementors decide how to allocate / construct a new monomial.
    fn quotient(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    /// Checked LCM: returns error if variable counts mismatch.
    ///
    /// Default implementation uses `n_vars/exponents` and allocates through
    /// `from_exponents_like` which implementors provide.
    fn checked_lcm(&self, other: &Self) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// LCM (may panic only if implementor chooses to).
    #[inline]
    fn lcm(&self, other: &Self) -> Self
    where
        Self: Sized,
        Self::Error: Debug,
    {
        self.checked_lcm(other)
            .expect("MonomialLike::lcm mismatched variable counts or overflow")
    }

    /// Returns `self / divisor` if `divisor | self`.
    ///
    /// This is the direction you want in polynomial division:
    /// `m_p.div_by(m_g)` gives the monomial multiplier.
    #[inline]
    fn div_by(&self, divisor: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        divisor.quotient(self)
    }

    /// Checked GCD: returns error if variable counts mismatch.
    fn checked_gcd(&self, other: &Self) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Clean GCD calculation
    #[inline]
    fn gcd(&self, other: &Self) -> Self
    where
        Self: Sized,
        Self::Error: Debug,
    {
        self.checked_gcd(other)
            .expect("MonomialLike::gcd mismatched variable counts")
    }
}
