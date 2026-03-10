//! Exponent vector traits.
//!
//! This module defines the minimal interface `gbx-poly` needs from exponent
//! storage backends.
//!
//!
//! **Note:** The meaning of the exponents (e.g. overflow rules, degree caching,
//! arity checks) belongs in `gbx-poly`.

/// Read-only exponent vector container.
///
/// This is intentionally tiny: it only provides `len()` and an immutable slice view.
/// Anything beyond that belongs to higher layers (`gbx-poly`).
pub trait Exps: Clone + Eq {
    /// Exponent "word" type (most commonly `u32`).
    ///
    /// Implementations may choose smaller types (e.g. `u16`) for packed storage,
    /// as long as they can expose a slice of words.
    type Word: Copy + Eq;

    /// Number of variables (length of the exponent vector).
    ///
    /// Must equal `self.as_slice().len()`.
    fn len(&self) -> usize;

    /// Borrow the exponent vector as a slice.
    fn as_slice(&self) -> &[Self::Word];

    /// Convenience: whether the exponent vector is empty.
    ///
    /// Empty exponent vectors are valid and represent a constant monomial
    /// in a `0`-variable ring, or may be used as a special-case representation
    /// depending on higher layers.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Mutable exponent vector container.
///
/// This trait is separate from [`Exps`] so read-only backends can remain minimal.
/// `gbx-poly` can use this for in-place monomial operations when supported.
pub trait ExpsMut: Exps {
    /// The returned slice must have the same length as `self.as_slice()`.
    fn as_mut_slice(&mut self) -> &mut [Self::Word];
}
