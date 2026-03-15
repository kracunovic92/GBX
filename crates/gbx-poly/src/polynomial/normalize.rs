//! Term list normalization (context-driven).
//!
//! This normalizes a `Vec<Term>` in-place:
//! - drops zero coefficients (`ctx.field.is_zero`)
//! - sorts by `ctx.order` descending (leading term first)
//! - merges identical monomials using `ctx.field.add`
//!
//! # Errors
//! This function itself does not perform monomial arithmetic; it only compares
//! monomials, combines coefficients, and rebuilds terms.
//!
//! # Panics
//! None.

use crate::monomial::{Monomial, MonomialView};
use crate::order::MonomialOrder;
use crate::polynomial::error::Result;
use crate::ring::{FieldCtx, RingCtx};
use crate::term::{TermOwned, TermView};

/// Normalize a term list in-place inside a ring context.
pub fn normalize_terms_in<F, O, T>(ctx: &RingCtx<F, O>, terms: &mut Vec<T>) -> Result<()>
where
    F: FieldCtx<Elem = T::Coeff>,
    O: MonomialOrder,
    T: TermOwned + TermView,
    T::Coeff: Copy + Eq,
    T::Mono: Monomial + MonomialView<Word = u32> + Clone + Eq,
{
    terms.retain(|t| !ctx.field.is_zero(*t.coeff()));
    if terms.len() <= 1 {
        return Ok(());
    }

    terms.sort_unstable_by(|a, b| ctx.order.cmp(b.mono(), a.mono()));

    let mut write = 0usize;
    let mut acc_coeff = *terms[0].coeff();
    let mut acc_mono = terms[0].mono().clone();

    for read in 1..terms.len() {
        if terms[read].mono() == &acc_mono {
            acc_coeff = ctx.field.add(acc_coeff, *terms[read].coeff());
        } else {
            if !ctx.field.is_zero(acc_coeff) {
                terms[write] = T::from_parts(acc_coeff, acc_mono);
                write += 1;
            }
            acc_coeff = *terms[read].coeff();
            acc_mono = terms[read].mono().clone();
        }
    }

    if !ctx.field.is_zero(acc_coeff) {
        terms[write] = T::from_parts(acc_coeff, acc_mono);
        write += 1;
    }

    terms.truncate(write);
    Ok(())
}
