use crate::algos::f4::error::F4Error;
use crate::algos::f4::symbolic::types::{SymbolicProduct, SymbolicSource};
use crate::algos::f4::types::PolyMono;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Find a symbolic top reducer for `monomial` in the current basis.
///
/// If some basis polynomial `g_i` has leading monomial `LM(g_i)` dividing
/// `monomial`, this returns the symbolic product `(m0, g_i)` where
/// `monomial = m0 * LM(g_i)`.
///
/// First-pass implementation:
/// - scans the basis linearly
/// - returns the first matching reducer
///
/// Later this can be replaced with an indexed lookup.
pub fn find_top_reducer_product<P>(monomial: &PolyMono<P>, basis: &[P]) -> Result<Option<SymbolicProduct<PolyMono<P>>>, F4Error>
where
    P: PolynomialView,
    P::Term: TermView,
    PolyMono<P>: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    for (basis_index, poly) in basis.iter().enumerate() {
        let Some(lead_mono) = poly.leading_mono() else {
            continue;
        };

        if lead_mono.divides(monomial) {
            let multiplier = monomial.checked_div_exact_by(lead_mono)?;

            return Ok(Some(SymbolicProduct {
                source: SymbolicSource::Basis(basis_index),
                multiplier,
            }));
        }
    }

    Ok(None)
}
/// Find the first basis index whose leading monomial divides `monomial`.
pub fn find_top_reducer_index<P>(monomial: &PolyMono<P>, basis: &[P]) -> Result<Option<(usize, PolyMono<P>)>, F4Error>
where
    P: PolynomialView,
    P::Term: TermView,
    PolyMono<P>: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    for (basis_index, poly) in basis.iter().enumerate() {
        let Some(lead_mono) = poly.leading_mono() else {
            continue;
        };

        if lead_mono.divides(monomial) {
            let multiplier = monomial.checked_div_exact_by(lead_mono)?;
            return Ok(Some((basis_index, multiplier)));
        }
    }

    Ok(None)
}
