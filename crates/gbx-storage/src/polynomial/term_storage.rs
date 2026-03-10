//! Storage abstraction for polynomial term collections.
//!
//! `gbx-poly` algorithms operate on a “bag of terms” but should not care whether
//! terms live in a `Vec`, a `BTreeMap`, an arena, etc.
//!
//! The key capability is: sometimes we need to run normalization on a `Vec<T>`
//! (sort + merge), so storages provide `with_vec` to temporarily materialize a
//! `Vec<T>` and persist changes back.

use std::vec::Vec;

/// Generic storage interface for a collection of polynomial terms.
pub trait TermStorage<T> {
    /// Borrow the stored terms as a slice.
    ///
    /// Note: some storages may return terms in a canonical order (e.g. `BTreeTerms`),
    /// while others return insertion order (e.g. `VecTerms`).
    fn as_slice(&self) -> &[T];

    /// Push a single raw term.
    fn push(&mut self, term: T);

    /// Replace contents from a vector (already materialized by caller).
    fn set_from_vec(&mut self, v: Vec<T>);

    /// Temporarily expose terms as a mutable `Vec<T>`.
    ///
    /// ## Contract
    /// Implementations must persist any modifications back into `Self`
    /// when the closure returns.
    fn with_vec<R>(&mut self, f: impl FnOnce(&mut Vec<T>) -> R) -> R;

    /// Remove all stored terms.
    ///
    /// Default implementation clears via [`with_vec`].
    ///
    /// Implementations may override this for more efficient behavior.
    #[inline]
    fn clear(&mut self) {
        self.with_vec(|v| v.clear());
    }

    /// Reserve capacity for at least `additional` more terms.
    ///
    /// Default implementation forwards to the underlying `Vec`.
    ///
    /// Implementations may override for storage-specific optimization.
    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.with_vec(|v| v.reserve(additional));
    }

    /// Returns `true` if storage contains no terms.
    ///
    /// Equivalent to `self.len() == 0`.
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    /// Returns the number of stored terms.
    ///
    /// This reflects the current internal representation and does not
    /// imply that terms are normalized or merged.
    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}
