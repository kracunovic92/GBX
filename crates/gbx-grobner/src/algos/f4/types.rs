//! Useful types for F4
use gbx_poly::monomial::Monomial;
use gbx_poly::polynomial::PolynomialView;

/// Canonical monomial type used by F4.
pub type PolyMono = Monomial;

/// Coefficient type of F4 polynomial.
pub type PolyCoeff<P> = <P as PolynomialView>::Coeff;

/// Follow if we need to continue or end process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationOutcome {
    /// Doing more
    Progress,
    /// Finally done
    Done,
}
