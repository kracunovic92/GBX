use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::term::{TermOwned, TermView};

/// Internal bound alias for term types used by Buchberger's algorithm.
pub trait BuchbergerTerm: TermOwned + TermView + Clone
where
    Self::Coeff: Copy + Eq,
    Self::Mono: Monomial + MonomialAlgos + MonomialView + Clone + Eq,
{
}

impl<T> BuchbergerTerm for T
where
    T: TermOwned + TermView + Clone,
    T::Coeff: Copy + Eq,
    T::Mono: Monomial + MonomialAlgos + MonomialView + Clone + Eq,
{
}
