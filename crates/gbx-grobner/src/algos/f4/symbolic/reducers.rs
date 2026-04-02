use crate::algos::f4::error::F4Error;
use crate::algos::f4::types::PolyMono;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

pub fn find_top_reducer<P>(monomial: &PolyMono<P>, basis: &[P]) -> Result<Option<(usize, PolyMono<P>)>, F4Error>
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
