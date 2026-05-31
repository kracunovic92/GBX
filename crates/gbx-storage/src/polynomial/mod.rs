//! Polynomial-related storage primitives.
//!
//! Storage-only (no algebra). Higher crates (`gbx-poly`) plug in different
//! internal representations while keeping polynomial algorithms the same.

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

        s.with_vec(|v| v.sort_unstable());
        assert_eq!(s.as_slice(), &[1, 2]);

        s.set_from_vec(vec![9]);
        assert_eq!(s.as_slice(), &[9]);
    }
}
