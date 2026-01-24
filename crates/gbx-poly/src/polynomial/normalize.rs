//! Shared term normalization.
//!
//! Normalization enforces the invariant expected by polynomial algorithms:
//! - remove zero terms
//! - sort by monomial order (descending)
//! - merge equal monomials by summing coefficients
//!
//! This function is generic over the *term type* and *monomial order*.
//! It operates on a `Vec<Term>`; storage backends supply a `Vec` via
//! `gbx_storage::polynomial::TermStorage::with_vec`.

use crate::monomial::Monomial;
use crate::monomial::MonomialOrder;
use crate::term::traits::Term;
use gbx_alg::{Field, Zero};

pub fn normalize_terms<T, O>(terms: &mut Vec<T>)
where
    T: Term,
    O: MonomialOrder,
    T::Field: Field + Clone,
    T::Mono: Monomial + Clone + Eq,
{
    terms.retain(|t| !t.coeff().is_zero());
    if terms.is_empty() {
        return;
    }

    terms.sort_by(|a, b| O::cmp(a.mono(), b.mono()).reverse());

    let mut out: Vec<T> = Vec::with_capacity(terms.len());

    for t in terms.drain(..) {
        if let Some(last) = out.last_mut() {
            if last.mono() == t.mono() {
                let sum = last.coeff().clone() + t.coeff().clone();
                if sum.is_zero() {
                    out.pop();
                } else {
                    let mono = last.mono().clone();
                    *last = T::from_parts(sum, mono);
                }
                continue;
            }
        }
        out.push(t);
    }

    *terms = out;
}
