use crate::criteria::{leading_mono_at, PairKey};
use crate::GrobnerBasis;
use gbx_poly::monomial::{checked_lcm_degree, Monomial, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Key = total degree of `lcm(LM_i, LM_j)`.
///
/// This is a standard Buchberger pair-ordering heuristic:
/// pairs with smaller LCM degree are typically considered earlier
/// by min-priority queues.
///
/// # Behavior
///
/// Returns:
///
/// - `Some(deg)` if both leading monomials exist and the LCM degree
///   was computed successfully
/// - `None` if:
///   - either polynomial is zero
///   - the LCM degree computation fails, for example due to overflow
///     or arity mismatch
#[derive(Debug, Default, Clone, Copy)]
pub struct LcmDegreeKey;

impl<P> PairKey<P> for LcmDegreeKey
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: MonomialView<Word = u32> + Monomial,
{
    #[inline]
    fn key_for_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> Option<u32> {
        let lm_i = leading_mono_at(gb, i)?;
        let lm_j = leading_mono_at(gb, j)?;
        checked_lcm_degree(lm_i, lm_j).ok()
    }
}
