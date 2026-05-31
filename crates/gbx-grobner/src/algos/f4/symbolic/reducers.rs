//! Top-reducer lookup for symbolic preprocessing.

use crate::algos::f4::error::F4Error;
use crate::algos::f4::symbolic::types::UnevaluatedProduct;

use gbx_poly::monomial::{Monomial, checked_div_exact, divides};
use gbx_poly::polynomial::PolynomialView;

/// Finds the first basis reducer whose leading monomial divides `monomial`.
///
/// Returns the corresponding unevaluated product `m * g_i`, where
/// `monomial = m * LM(g_i)`.
///
/// # Errors
///
/// Returns a monomial error if the multiplier quotient cannot be computed.
pub fn find_top_reducer_product<P>(monomial: &Monomial, basis: &[P]) -> Result<Option<UnevaluatedProduct<Monomial>>, F4Error>
where
    P: PolynomialView,
{
    Ok(find_top_reducer_index(monomial, basis)?.map(|(basis_index, multiplier)| UnevaluatedProduct::from_basis(basis_index, multiplier)))
}

/// Finds the first basis index whose leading monomial divides `monomial`.
///
/// The returned monomial is the multiplier needed to align the reducer's
/// leading monomial with `monomial`.
///
/// # Errors
///
/// Returns a monomial error if the multiplier quotient cannot be computed.
pub fn find_top_reducer_index<P>(monomial: &Monomial, basis: &[P]) -> Result<Option<(usize, Monomial)>, F4Error>
where
    P: PolynomialView,
{
    for (basis_index, poly) in basis.iter().enumerate() {
        let Some(lead_mono) = poly.leading_mono() else {
            continue;
        };

        if !divides(lead_mono, monomial) {
            continue;
        }

        let multiplier = checked_div_exact(lead_mono, monomial)?;

        return Ok(Some((basis_index, multiplier)));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::Polynomial;

    type P = Polynomial<FpElem>;

    #[test]
    fn finds_first_reducer_index_and_multiplier() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        // LM = x
        let g0: P = poly![&ring; (1, [1, 0]), (1, [0, 0])].unwrap();

        // LM = y
        let g1: P = poly![&ring; (1, [0, 1]), (1, [0, 0])].unwrap();

        let basis = vec![g0, g1];

        let target = Monomial::from_slice(&[2, 1]);
        let found = find_top_reducer_index(&target, &basis)
            .expect("lookup should not fail")
            .expect("reducer should exist");

        assert_eq!(found.0, 0);
        assert_eq!(found.1, Monomial::from_slice(&[1, 1]));
    }

    #[test]
    fn returns_none_when_no_reducer_divides_target() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let g0: P = poly![&ring; (1, [2, 0])].unwrap();
        let basis = vec![g0];

        let target = Monomial::from_slice(&[0, 3]);

        assert!(find_top_reducer_index(&target, &basis).unwrap().is_none());
    }

    #[test]
    fn skips_zero_basis_polynomials() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let zero = P::zero_in(&ring);
        let g1: P = poly![&ring; (1, [0, 1])].unwrap();

        let basis = vec![zero, g1];

        let target = Monomial::from_slice(&[1, 2]);
        let found = find_top_reducer_index(&target, &basis)
            .expect("lookup should not fail")
            .expect("reducer should exist");

        assert_eq!(found.0, 1);
        assert_eq!(found.1, Monomial::from_slice(&[1, 1]));
    }

    #[test]
    fn product_lookup_wraps_index_lookup() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let g0: P = poly![&ring; (1, [1, 0])].unwrap();
        let basis = vec![g0];

        let target = Monomial::from_slice(&[3, 2]);

        let product = find_top_reducer_product(&target, &basis)
            .expect("lookup should not fail")
            .expect("product should exist");

        assert_eq!(
            product.source,
            crate::algos::f4::symbolic::ProductSource::Basis(0)
        );
        assert_eq!(product.multiplier, Monomial::from_slice(&[2, 2]));
    }
}
