use crate::algos::f4::error::Result;
use crate::algos::f4::pairs::criterion::PairCriterion;
use crate::algos::f4::pairs::pending::{add_pairs_with_new_basis_element, PendingPairs};

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

pub fn update_with_polynomial<P, C>(basis: &mut Vec<P>, pending: &mut PendingPairs<<<P as PolynomialView>::Term as TermView>::Mono>, polynomial: P, criterion: &C) -> Result<()>
where
    P: PolynomialView + Clone,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    C: PairCriterion<P>,
{
    let new_index = basis.len();
    basis.push(polynomial);
    add_pairs_with_new_basis_element(basis, new_index, pending, criterion)?;
    Ok(())
}
