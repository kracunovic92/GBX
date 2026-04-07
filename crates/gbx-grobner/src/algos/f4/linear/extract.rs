use std::collections::HashSet;
use std::hash::Hash;

use crate::algos::f4::error::Result;
use crate::algos::f4::linear::dense::first_nonzero_col;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Collect leading monomials of the original symbolic rows.
/// These are the heads already present before matrix reduction.
pub fn collect_symbolic_heads<P>(rows: &[P]) -> HashSet<<P::Term as TermView>::Mono>
where
    P: PolynomialMut + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq + Default,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Hash,
{
    rows.iter()
        .filter_map(|r| r.leading_mono().cloned())
        .collect()
}

/// Decode one reduced dense row back into a polynomial.
pub fn decode_dense_row<P, F, O>(ctx: &RingCtx<F, O>, coeffs: &[<P::Term as TermView>::Coeff], columns: &[<P::Term as TermView>::Mono]) -> Result<P>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialView,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq + Default,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let mut out = P::zero_in(ctx);

    for (coeff, mono) in coeffs.iter().zip(columns.iter()) {
        if *coeff == <P::Term as TermView>::Coeff::default() {
            continue;
        }

        let term = <P::Term as TermOwned>::from_parts(*coeff, mono.clone());
        out.push_term_raw(term);
    }

    out.normalize_in_place(ctx)?;
    Ok(out)
}

/// Extract only rows whose leading monomial is new relative to symbolic input.
pub fn extract_new_rows_from_dense<P, F, O>(ctx: &RingCtx<F, O>, symbolic_rows: &[P], reduced_rows: &[Vec<<P::Term as TermView>::Coeff>], columns: &[<P::Term as TermView>::Mono]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq + Default,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Hash,
{
    let symbolic_heads = collect_symbolic_heads(symbolic_rows);
    let mut out = Vec::new();

    for row in reduced_rows {
        let Some(head_col) = first_nonzero_col(row) else {
            continue;
        };

        let head_mono = &columns[head_col];
        if symbolic_heads.contains(head_mono) {
            continue;
        }

        let poly = decode_dense_row::<P, F, O>(ctx, row, columns)?;
        if !poly.is_zero() {
            out.push(poly);
        }
    }

    Ok(out)
}
