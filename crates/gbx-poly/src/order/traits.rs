//! Order trait shared by compile-time and runtime-selected orders.

use core::cmp::Ordering;

use crate::monomial::MonomialView;

/// A monomial order (term order) on exponent vectors.
///
/// Gröbner basis algorithms assume a fixed monomial order (lex, grevlex, ...).
/// We define orders as comparisons on exponent slices, which makes them usable
/// for any monomial representation.
///
/// Implementations must define a **total order**.
///
/// # Panics
/// `cmp_exps`/`cmp` may panic if exponent vectors have different lengths.
/// Comparing monomials with differing variable counts is considered a
/// programmer error.
pub trait MonomialOrder {
    /// Compare exponent vectors `a` and `b` according to this order.
    fn cmp_exps(&self, a: &[u32], b: &[u32]) -> Ordering;

    /// Convenience helper: compare monomials by calling `exponents()`.
    #[inline]
    fn cmp<M: MonomialView<Word = u32>>(&self, a: &M, b: &M) -> Ordering {
        self.cmp_exps(a.exponents(), b.exponents())
    }
}

#[cfg(test)]
mod tests {
    use super::MonomialOrder;
    use crate::monomial::{FixedMonomial, MonomialView};
    use crate::order::{GREVLEX, LEX};
    use core::cmp::Ordering;

    #[test]
    fn cmp_exps_matches_cmp_for_fixed() {
        let a = FixedMonomial::<3>::from_exponents([2, 1, 0]);
        let b = FixedMonomial::<3>::from_exponents([2, 0, 1]);

        assert_eq!(LEX.cmp(&a, &b), LEX.cmp_exps(a.exponents(), b.exponents()));
        assert_eq!(
            GREVLEX.cmp(&a, &b),
            GREVLEX.cmp_exps(a.exponents(), b.exponents())
        );
    }

    #[test]
    fn cmp_exps_equal_is_equal() {
        let a = [1u32, 2, 3];
        let b = [1u32, 2, 3];

        assert_eq!(LEX.cmp_exps(&a, &b), Ordering::Equal);
        assert_eq!(GREVLEX.cmp_exps(&a, &b), Ordering::Equal);
    }
}
