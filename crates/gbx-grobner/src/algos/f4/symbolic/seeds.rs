//! Build aligned pair seed rows for F4 symbolic preprocessing.

use crate::algos::f4::batch::PairBatch;
use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::symbolic::types::{SeedRow, SymbolicRowKind};
use crate::basis::GrobnerBasis;
use crate::pairing::leading_mono_at;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Build aligned pair seed rows for a selected F4 batch.
///
/// For each selected critical pair `(i, j)`, this computes the two monomial
/// multiples whose leading monomials agree at
///
/// `lcm(LM(g_i), LM(g_j))`.
///
/// The output therefore contains exactly two seed rows per selected pair:
///
/// - `u_i * g_i`,
/// - `u_j * g_j`,
///
/// where
///
/// - `u_i * LM(g_i) = lcm(LM(g_i), LM(g_j))`,
/// - `u_j * LM(g_j) = lcm(LM(g_i), LM(g_j))`.
///
/// Missing basis indices are treated as internal invariant violations.
pub fn build_pair_seed_rows<P, K>(gb: &GrobnerBasis<P>, batch: &PairBatch<K>) -> Result<Vec<SeedRow<<P::Term as TermView>::Mono>>>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let mut out = Vec::with_capacity(batch.len() * 2);

    for pair in batch.iter() {
        let lm_i = leading_mono_at(gb, pair.i).ok_or(F4Error::MissingBasisPolynomial { index: pair.i })?;
        let lm_j = leading_mono_at(gb, pair.j).ok_or(F4Error::MissingBasisPolynomial { index: pair.j })?;

        let lcm = lm_i
            .checked_lcm(lm_j)
            .map_err(|_| F4Error::SymbolicInvariant)?;

        let u_i = lcm
            .checked_div_by(lm_i)
            .map_err(|_| F4Error::SymbolicInvariant)?
            .ok_or(F4Error::SymbolicInvariant)?;

        let u_j = lcm
            .checked_div_by(lm_j)
            .map_err(|_| F4Error::SymbolicInvariant)?
            .ok_or(F4Error::SymbolicInvariant)?;

        out.push(SeedRow { source_basis_index: pair.i, multiplier: u_i, kind: SymbolicRowKind::PairLeft });

        out.push(SeedRow { source_basis_index: pair.j, multiplier: u_j, kind: SymbolicRowKind::PairRight });
    }

    Ok(out)
}
