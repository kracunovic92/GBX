//! Basic extraction
use std::collections::BTreeSet;

use crate::algos::f4::error::Result;
use crate::algos::f4::symbolic::ordered::OrderedMono;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;

/// Extract reduced rows whose leading monomials are new relative to the
/// original symbolic rows.
///
/// # Errors
///
/// Currently this operation does not fail, but it returns the shared F4 result
/// type to match the extraction pipeline.
pub fn extract_new_rows<P, O>(symbolic_rows: &[P], reduced_rows: &[P], order: &O) -> Result<Vec<P>>
where
    O: MonomialOrder,
    P: PolynomialView + Clone,
{
    let existing_heads: BTreeSet<_> = symbolic_rows
        .iter()
        .filter_map(|row| row.leading_mono().cloned())
        .map(|mono| OrderedMono::new(mono, order))
        .collect();

    let mut out = Vec::with_capacity(reduced_rows.len());

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
