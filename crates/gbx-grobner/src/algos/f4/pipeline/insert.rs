use crate::algos::f4::error::Result;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::update::update_with_polynomial;
use crate::algos::f4::state::F4State;
use crate::instrumentation::{f4_debug, f4_span};

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

#[cfg_attr(
    feature = "instrumentation",
    tracing::instrument(level = "debug", skip(ctx, state, rows, criterion), fields(num_rows = rows.len()))
)]
pub fn insert_new_rows<P, F, O>(ctx: &RingCtx<F, O>, state: &mut F4State<P>, rows: Vec<P>, criterion: &ProductCriterion) -> Result<()>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Default,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    let _ = ctx;

    f4_span!("insert_new_rows_loop");

    let mut inserted = 0usize;
    let mut zero_rows = 0usize;

    for row in rows {
        if row.is_zero() {
            zero_rows += 1;
            continue;
        }

        update_with_polynomial(&mut state.basis, &mut state.pending, row, criterion)?;
        inserted += 1;
    }

    f4_debug!(
        inserted,
        zero_rows,
        basis_len = state.basis.len(),
        pending_len = state.pending.len(),
        "finished inserting new rows"
    );

    Ok(())
}
