//! Default `Vec<T>`-backed term storage.

extern crate alloc;
use alloc::vec::Vec;

use super::term_storage::TermStorage;

/// `Vec<T>`-backed storage for polynomial terms.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VecTerms<T> {
    terms: Vec<T>,
}

impl<T> VecTerms<T> {
    #[inline]
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    #[inline]
    pub fn from_vec(terms: Vec<T>) -> Self {
        Self { terms }
    }

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

    // Override defaults for zero overhead
    #[inline]
    fn clear(&mut self) {
        self.terms.clear();
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.terms.reserve(additional);
    }
}
