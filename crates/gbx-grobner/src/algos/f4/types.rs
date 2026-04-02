use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Canonical monomial type of an F4 polynomial.
pub type PolyMono<P> = <<P as PolynomialView>::Term as TermView>::Mono;

/// Canonical coefficient type of an F4 polynomial.
pub type PolyCoeff<P> = <<P as PolynomialView>::Term as TermView>::Coeff;
