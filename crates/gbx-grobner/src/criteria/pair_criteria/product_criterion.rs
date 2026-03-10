use crate::criteria::{leading_mono_at, PairCriterion};
use crate::GrobnerBasis;
use gbx_poly::monomial::{gcd_is_one, Monomial, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Buchberger's product criterion.
///
/// This criterion eliminates a pair `(i, j)` when the leading monomials
/// `LM(g_i)` and `LM(g_j)` are relatively prime.
///
/// Equivalently, if
///
/// `gcd(LM(g_i), LM(g_j)) = 1`,
///
/// then the pair is discarded.
///
/// # Purpose
///
/// This is a standard and sound Buchberger criterion.
/// It is often the first optimization added to a baseline implementation.
///
/// # Behavior
///
/// - returns `true` if the pair should be kept
/// - returns `false` if the product criterion eliminates the pair
///
/// If either polynomial is zero or missing, this implementation conservatively
/// returns `false`, meaning the pair is not kept.
///
/// # Notes
///
/// This criterion is purely local:
/// it depends only on the two leading monomials.
/// That makes it a natural fit for [`PairCriterion`].
#[derive(Debug, Default, Clone, Copy)]
pub struct ProductCriterion;

impl<P> PairCriterion<P> for ProductCriterion
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: MonomialView<Word = u32> + Monomial,
{
    #[inline]
    fn keep_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> bool {
        let lm_i = match leading_mono_at(gb, i) {
            Some(m) => m,
            None => return false,
        };

        let lm_j = match leading_mono_at(gb, j) {
            Some(m) => m,
            None => return false,
        };

        !gcd_is_one(lm_i, lm_j)
    }
}
