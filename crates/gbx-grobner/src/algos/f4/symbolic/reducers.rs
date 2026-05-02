use crate::algos::f4::error::F4Error;
use crate::algos::f4::symbolic::types::{SymbolicProduct, SymbolicSource};

use gbx_poly::monomial::{checked_div_exact, divides, Monomial};
use gbx_poly::polynomial::PolynomialView;

/// If some basis polynomial `g_i` has `LM(g_i)` dividing `monomial`,
/// return the symbolic product `(m0, g_i)` where:
///
/// `monomial = m0 * LM(g_i)`.
///
/// First-pass implementation:
/// - scans the basis linearly,
/// - returns the first matching reducer.
pub fn find_top_reducer_product<P>(monomial: &Monomial, basis: &[P]) -> Result<Option<SymbolicProduct<Monomial>>, F4Error>
where
    P: PolynomialView,
{
    for (basis_index, poly) in basis.iter().enumerate() {
        let Some(lead_mono) = poly.leading_mono() else {
            continue;
        };

        if divides(lead_mono, monomial) {
            let multiplier = checked_div_exact(lead_mono, monomial)?;

            return Ok(Some(SymbolicProduct {
                source: SymbolicSource::Basis(basis_index),
                multiplier,
            }));
        }
    }

    Ok(None)
}

/// Find the first basis index whose leading monomial divides `monomial`.
pub fn find_top_reducer_index<P>(monomial: &Monomial, basis: &[P]) -> Result<Option<(usize, Monomial)>, F4Error>
where
    P: PolynomialView,
{
    for (basis_index, poly) in basis.iter().enumerate() {
        let Some(lead_mono) = poly.leading_mono() else {
            continue;
        };

        if divides(lead_mono, monomial) {
            let multiplier = checked_div_exact(lead_mono, monomial)?;

            return Ok(Some((basis_index, multiplier)));
        }
    }

    Ok(None)
}
