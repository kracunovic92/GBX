//! Owned term construction hooks.
//!
//! Normalization needs to rebuild terms (new coeff + same mono) but does not
//! require term arithmetic. This trait is intentionally tiny.

use crate::term::{Term, TermView};

/// Construction-only term trait used by normalization and container glue.
///
/// Keep this separate from `Term` (which adds arithmetic like checked mul).
pub trait TermOwned: TermView + Sized {
    /// Rebuild a term from owned parts.
    fn from_parts(coeff: Self::Coeff, mono: Self::Mono) -> Self;
}
impl<C, M> TermOwned for Term<C, M>
where
    M: crate::monomial::MonomialView,
{
    #[inline]
    fn from_parts(coeff: Self::Coeff, mono: Self::Mono) -> Self {
        Term::new(coeff, mono)
    }
}
