use gbx_poly::monomial::Monomial;
use gbx_poly::polynomial::PolynomialView;

/// Canonical monomial type used by F4.
pub type PolyMono = Monomial;

/// Coefficient type of an F4 polynomial.
pub type PolyCoeff<P> = <P as PolynomialView>::Coeff;
