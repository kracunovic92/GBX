use crate::algos::f4::pairs::critical_pair::CriticalPair;
use gbx_poly::monomial::{gcd_is_one, Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Decide whether a newly generated critical pair should be kept.
pub trait PairCriterion<P>
where
    P: PolynomialView,
    P::Term: TermView,
{
    fn allows(&self, basis: &[P], pair: &CriticalPair<<<P as PolynomialView>::Term as TermView>::Mono>) -> bool;
}

/// Keep every pair.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoCriterion;

impl<P> PairCriterion<P> for NoCriterion
where
    P: PolynomialView,
    P::Term: TermView,
{
    fn allows(&self, _basis: &[P], _pair: &CriticalPair<<<P as PolynomialView>::Term as TermView>::Mono>) -> bool {
        true
    }
}

/// Buchberger product criterion:
/// if gcd(LM(f_i), LM(f_j)) = 1, then the pair can be discarded.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProductCriterion;

impl<P> PairCriterion<P> for ProductCriterion
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone,
{
    fn allows(&self, basis: &[P], pair: &CriticalPair<<<P as PolynomialView>::Term as TermView>::Mono>) -> bool {
        let Some(lm_i) = basis[pair.i].leading_mono() else {
            return false;
        };
        let Some(lm_j) = basis[pair.j].leading_mono() else {
            return false;
        };

        !gcd_is_one(lm_i, lm_j)
    }
}
