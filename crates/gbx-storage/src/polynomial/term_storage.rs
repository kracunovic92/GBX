//! Storage abstraction for polynomial term collections.
//!
//! `gbx-poly` algorithms operate on a “bag of terms” but should not care whether
//! terms live in a `Vec`, an arena, or a packed representation.
//!
//! The key capability is: sometimes we need to run normalization on a `Vec<T>`
//! (sort + merge), so storages provide `with_vec` to temporarily materialize a
//! `Vec<T>` and persist changes back.

extern crate alloc;
use alloc::vec::Vec;

/// Generic storage interface for a collection of polynomial terms.
pub trait TermStorage<T> {
    /// Borrow the stored terms as a slice.
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

    /// Remove all terms.
    ///
    /// Default implementation uses `with_vec` so it works for any storage.
    #[inline]
    fn clear(&mut self) {
        self.with_vec(|v| v.clear());
    }

    /// Reserve capacity for at least `additional` more terms.
    ///
    /// Default implementation uses `with_vec`. Vec-backed storages should
    /// override for zero overhead.
    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.with_vec(|v| v.reserve(additional));
    }

    /// Convenience: empty check.
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    /// Convenience: length.
    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}
