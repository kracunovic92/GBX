//! Polynomial-related storage primitives.
//!
//! This module is **storage-only** (no math). It exists to let higher-level crates
//! (`gbx-poly`) plug in different internal representations while keeping the
//! polynomial *interface* the same.
//!
//! # Why is this in `gbx-storage`?
//!
//! `gbx-poly` owns polynomial algorithms (normalization, division, Gröbner, ...)
//! and only needs a small set of operations on “a bag of terms”:
//!
//! - read-only access as `&[T]`
//! - `push(T)` while building intermediate polynomials
//! - occasionally: sort/merge/normalize (best done on a `Vec<T>`)
//!
//! The [`TermStorage`] trait provides exactly that.
//!
//! # Provided implementations
//!
//! - [`VecTerms<T>`]: a minimal `Vec<T>`-backed storage (default).

mod term_storage;
mod vec_terms;

pub use term_storage::TermStorage;
pub use vec_terms::VecTerms;

#[cfg(test)]
mod tests {
    use super::{TermStorage, VecTerms};

    #[test]
    fn vec_terms_basics() {
        let mut s = VecTerms::<i32>::new();
        assert!(s.is_empty());

        s.push(2);
        s.push(1);
        assert_eq!(s.as_slice(), &[2, 1]);

        s.with_vec(|v| v.sort());
        assert_eq!(s.as_slice(), &[1, 2]);

        s.set_from_vec(vec![9]);
        assert_eq!(s.as_slice(), &[9]);
    }
}
