//! Term-list normalization.

use crate::order::MonomialOrder;
use crate::polynomial::error::PolynomialResult;
use crate::ring::{FieldCtx, RingCtx};
use crate::term::Term;

/// Normalizes a term list in-place.
///
/// Normalization:
/// - drops zero coefficients,
/// - sorts descending by monomial order,
/// - merges duplicate monomials.
///
/// # Errors
///
/// Returns a polynomial error if term normalization fails.
pub fn normalize_terms_in<F, O>(ctx: &RingCtx<F, O>, terms: &mut Vec<Term<F::Elem>>) -> PolynomialResult<()>
where
    F: FieldCtx,
    O: MonomialOrder,
    F::Elem: Copy + Eq,
{
    terms.retain(|t| !ctx.field.is_zero(*t.coeff()));

    if terms.len() <= 1 {
        return Ok(());
    }

    terms.sort_unstable_by(|a, b| ctx.order.cmp(b.mono(), a.mono()));

    let mut out: Vec<Term<F::Elem>> = Vec::with_capacity(terms.len());

    let mut iter = terms.drain(..);

    let Some(first) = iter.next() else {
        return Ok(());
    };

    let (mut acc_coeff, mut acc_mono) = first.into_parts();

    for t in iter {
        let (coeff, mono) = t.into_parts();

        if mono == acc_mono {
            acc_coeff = ctx.field.add(acc_coeff, coeff);
        } else {
            if !ctx.field.is_zero(acc_coeff) {
                out.push(Term::new(acc_coeff, acc_mono));
            }

            acc_coeff = coeff;
            acc_mono = mono;
        }
    }

    if !ctx.field.is_zero(acc_coeff) {
        out.push(Term::new(acc_coeff, acc_mono));
    }

    *terms = out;

    Ok(())
}
