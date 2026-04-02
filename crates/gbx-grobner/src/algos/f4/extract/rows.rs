use std::collections::BTreeSet;

use crate::algos::f4::error::Result;
use crate::algos::f4::symbolic::ordered::OrderedMono;
use crate::algos::f4::types::PolyMono;

use gbx_poly::monomial::MonomialView;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Extract reduced rows whose leading monomials are new relative to the
/// original symbolic rows.
///
/// In F4 terms, this corresponds to keeping only rows that contribute new
/// leading monomials after batch reduction.
pub fn extract_new_rows<P, O>(symbolic_rows: &[P], reduced_rows: &[P], order: &O) -> Result<Vec<P>>
where
    O: MonomialOrder,
    P: PolynomialView + Clone,
    P::Term: TermView,
    PolyMono<P>: Clone + Eq + MonomialView<Word = u32>,
{
    let existing_heads: BTreeSet<_> = symbolic_rows
        .iter()
        .filter_map(|row| row.leading_mono().cloned())
        .map(|mono| OrderedMono::new(mono, order))
        .collect();

    let mut out = Vec::new();

    for row in reduced_rows {
        let Some(lead_mono) = row.leading_mono().cloned() else {
            continue;
        };

        let key = OrderedMono::new(lead_mono, order);
        if !existing_heads.contains(&key) {
            out.push(row.clone());
        }
    }

    Ok(out)
}
