//! Default `Vec<T>`-backed term storage.

use super::term_storage::TermStorage;
use std::vec::Vec;

/// `Vec<T>`-backed storage for polynomial terms.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VecTerms<T> {
    terms: Vec<T>,
}

impl<T> VecTerms<T> {
    /// Creates an empty `VecTerms`.
    ///
    /// # Example
    ///
    /// ```
    /// let storage: VecTerms<i32> = VecTerms::new();
    /// assert!(storage.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    /// Creates storage from an existing vector.
    ///
    /// This does **not** normalize or reorder the terms.
    /// The vector is taken as-is.
    ///
    /// # Responsibility
    ///
    /// The caller is responsible for ensuring any required invariants
    /// (e.g., sorted order or merged terms).
    #[inline]
    pub fn from_vec(terms: Vec<T>) -> Self {
        Self { terms }
    }

    /// Consumes the storage and returns the underlying vector.
    ///
    /// This is a zero-cost operation.
    ///
    /// Useful when:
    ///
    /// - Converting back to a concrete representation
    /// - Passing ownership to another component
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.terms
    }
}

impl<T> Default for VecTerms<T> {
    #[inline]
    fn default() -> Self {
        Self { terms: Vec::new() }
    }
}

impl<T> TermStorage<T> for VecTerms<T> {
    #[inline]
    fn as_slice(&self) -> &[T] {
        &self.terms
    }

    #[inline]
    fn push(&mut self, term: T) {
        self.terms.push(term);
    }

    #[inline]
    fn set_from_vec(&mut self, v: Vec<T>) {
        self.terms = v;
    }

    #[inline]
    fn with_vec<R>(&mut self, f: impl FnOnce(&mut Vec<T>) -> R) -> R {
        f(&mut self.terms)
    }

    #[inline]
    fn clear(&mut self) {
        self.terms.clear();
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.terms.reserve(additional);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::TermStorage;

    #[test]
    fn preserves_insertion_order() {
        let mut s = VecTerms::new();
        s.push(3);
        s.push(1);
        s.push(2);
        assert_eq!(s.as_slice(), &[3, 1, 2]);
    }

    #[test]
    fn with_vec_persists_mutations() {
        let mut s = VecTerms::from_vec(vec![3, 2, 1]);
        s.with_vec(|v| v.sort());
        assert_eq!(s.as_slice(), &[1, 2, 3]);
    }
}
