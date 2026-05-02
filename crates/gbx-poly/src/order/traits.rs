//! Order trait shared by compile-time and runtime-selected orders.

use core::cmp::Ordering;

use crate::monomial::MonomialView;

/// A monomial order on exponent vectors.
///
/// Gröbner basis algorithms assume one fixed monomial order for a ring.
/// Implementations must define a total order on exponent vectors of equal
/// length.
///
/// # Panics
///
/// Implementations may panic if exponent vectors have different lengths.
pub trait MonomialOrder {
    /// Compares two exponent vectors according to this order.
    fn cmp_exps(&self, a: &[u32], b: &[u32]) -> Ordering;

    /// Compares two monomials by comparing their exponent vectors.
    #[inline]
    fn cmp<M: MonomialView>(&self, a: &M, b: &M) -> Ordering {
        self.cmp_exps(a.exponents(), b.exponents())
    }
}

#[cfg(test)]
mod tests {
    use super::MonomialOrder;
    use crate::monomial::{Monomial, MonomialView};
    use crate::order::{GREVLEX, LEX};
    use core::cmp::Ordering;

    #[test]
    fn cmp_exps_matches_cmp() {
        let a = Monomial::from_slice(&[2, 1, 0]);
        let b = Monomial::from_slice(&[2, 0, 1]);

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
