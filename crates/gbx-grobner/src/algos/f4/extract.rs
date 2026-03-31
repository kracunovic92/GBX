use crate::algos::f4::error::Result;
use crate::basis::GrobnerBasis;

use crate::matrix::types::ReducedMatrixData;
use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Rebuild candidate basis polynomials from reduced F4 matrix rows.
///
/// This is a correctness-first extraction step:
/// - each nonzero reduced row is converted back into a polynomial,
/// - rows may be normalized,
/// - zero rows are discarded,
/// - rows whose leading monomial is already present in the basis are discarded.
pub fn extract_new_polynomials<F, O, P>(
    ctx: &RingCtx<F, O>,
    gb: &GrobnerBasis<P>,
    reduced: ReducedMatrixData<F::Elem, <P::Term as TermView>::Mono>,
    normalize_extracted: bool,
    _safety_reduce_extracted: bool,
) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Clone,
    <P::Term as TermView>::Mono: Monomial + Clone + PartialEq,
{
    let mut out = Vec::new();

    for row in reduced.rows {
        let mut terms = Vec::new();

        for (coeff, mono) in row.into_iter().zip(reduced.columns.monomials.iter()) {
            if ctx.field.is_zero(coeff.clone()) {
                continue;
            }

            terms.push(P::Term::from_parts(coeff, mono.clone()));
        }

        if terms.is_empty() {
            continue;
        }

        let mut p = P::from_terms_in(ctx, terms)?;

        if normalize_extracted {
            p.normalize_in_place(ctx)?;
        }

        if p.is_zero() {
            continue;
        }

        let Some(lm) = p.leading_term().map(|t| t.mono().clone()) else {
            continue;
        };

        let already_present = gb
            .iter()
            .filter_map(|g| g.leading_term().map(|t| t.mono()))
            .any(|m| m == &lm);

        if !already_present {
            out.push(p);
        }
    }

    Ok(out)
}
