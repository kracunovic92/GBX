//! Display adapter for Gröbner bases.

use crate::display::style::GbStyle;
use core::fmt;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolyDisplay, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Display adapter for a Gröbner basis.
///
/// A Gröbner basis is represented as a slice of polynomials.
/// This type renders the entire basis using a chosen [`GbStyle`].
#[derive(Debug)]
pub struct GbDisplay<'a, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView<Coeff = F::Elem>,
{
    ring: &'a RingCtx<F, O>,
    polys: &'a [P],
    vars: &'a [String],
    style: GbStyle,
}

impl<'a, F, O, P> GbDisplay<'a, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView<Coeff = F::Elem>,
{
    /// Creates a human-readable Gröbner basis formatter.
    ///
    /// Each polynomial is rendered on its own line using [`PolyDisplay::pretty`].
    #[inline]
    pub fn pretty_lines(ring: &'a RingCtx<F, O>, polys: &'a [P], vars: &'a [String]) -> Self {
        Self { ring, polys, vars, style: GbStyle::PrettyLines }
    }

    /// Creates a stable tuple-based Gröbner basis formatter.
    ///
    /// Each polynomial is rendered on its own line using [`PolyDisplay::tuple_dump`].
    #[inline]
    pub fn tuple_lines(ring: &'a RingCtx<F, O>, polys: &'a [P], vars: &'a [String]) -> Self {
        Self { ring, polys, vars, style: GbStyle::TupleLines }
    }
}

impl<F, O, P> fmt::Display for GbDisplay<'_, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView<Coeff = F::Elem>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, p) in self.polys.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }

            match self.style {
                GbStyle::PrettyLines => {
                    write!(f, "{}", PolyDisplay::pretty(self.ring, p, self.vars))?;
                }
                GbStyle::TupleLines => {
                    write!(f, "{}", PolyDisplay::tuple_dump(self.ring, p, self.vars))?;
                }
            }
        }

        Ok(())
    }
}
