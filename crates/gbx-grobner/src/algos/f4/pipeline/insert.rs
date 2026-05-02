use crate::algos::f4::error::Result;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::update::update_with_polynomial;
use crate::algos::f4::state::F4State;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

pub fn insert_new_rows<P, F, O>(ctx: &RingCtx<F, O>, state: &mut F4State<P>, rows: Vec<P>, criterion: &ProductCriterion) -> Result<()>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    for mut row in rows {
        if row.is_zero() {
            continue;
        }

        row.normalize_in_place(ctx)?;

        if row.is_zero() {
            continue;
        }

        update_with_polynomial(&mut state.basis, &mut state.pending, row, criterion)?;
    }

    Ok(())
}
