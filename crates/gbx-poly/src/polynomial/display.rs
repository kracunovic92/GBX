use crate::monomial::MonomialView;
use crate::order::MonomialOrder;
use crate::polynomial::PolynomialView;
use crate::ring::{FieldCtx, RingCtx};
use crate::term::{TermDisplay, TermView};
use core::fmt;

/// Formatting style for polynomial display.
///
/// This controls how [`PolyDisplay`] renders a polynomial.
///
/// - [`Pretty`](PolyStyle::Pretty): human-friendly mathematical form
/// - [`TupleDump`](PolyStyle::TupleDump): stable machine-friendly form for testing/diffing

#[derive(Debug, Clone, Copy)]
pub enum PolyStyle {
    /// Human-readable form, e.g. `x^2 + 3*y - 1`.
    Pretty,
    /// Stable tuple dump, e.g. `[(3,[1,2]),(1,[0,0])]`.
    TupleDump,
}
/// Display adapter for a polynomial.
///
/// This type renders a polynomial without requiring the polynomial type itself
/// to implement formatting policies. It borrows:
///
/// - the ring context (for coefficient representation/modulus)
/// - the polynomial
/// - a variable name list (used by pretty printing)
///
/// # Styles
///
/// - [`PolyStyle::Pretty`] prints a human-friendly sum of terms.
/// - [`PolyStyle::TupleDump`] prints a stable representation suitable for snapshot tests.
///
/// # Variable Names
///
/// `vars` is used only for pretty printing. It must match the ring’s variable
/// ordering (index `0` corresponds to the first variable, etc.).
#[derive(Debug)]
pub struct PolyDisplay<'a, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView<Coeff = F::Elem>,
{
    ring: &'a RingCtx<F, O>,
    poly: &'a P,
    vars: &'a [String],
    style: PolyStyle,
}

impl<'a, F, O, P> PolyDisplay<'a, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView<Coeff = F::Elem>,
{
    /// This function does not normalize the polynomial. Terms are rendered in the
    /// order returned by `poly.terms()`.
    pub fn pretty(ring: &'a RingCtx<F, O>, poly: &'a P, vars: &'a [String]) -> Self {
        Self { ring, poly, vars, style: PolyStyle::Pretty }
    }
    /// Creates a stable “tuple dump” polynomial formatter.
    ///
    /// Output format:
    ///
    /// ```text
    /// [(c,[e0,e1,...]),(c,[e0,e1,...]),...]
    /// ```
    ///
    /// where each term is formatted using [`TermDisplay::tuple`].
    ///
    /// This style is intended for:
    ///
    /// - snapshot testing
    /// - diffing Gröbner bases
    /// - external cross-checks (e.g. Singular output normalization)
    pub fn tuple_dump(ring: &'a RingCtx<F, O>, poly: &'a P, vars: &'a [String]) -> Self {
        Self { ring, poly, vars, style: PolyStyle::TupleDump }
    }
}

impl<'a, F, O, P> fmt::Display for PolyDisplay<'a, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView<Coeff = F::Elem>,
    <P::Term as TermView>::Mono: MonomialView<Word = u32>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.style {
            PolyStyle::TupleDump => fmt_poly_tuple(self.ring, self.poly, f),
            PolyStyle::Pretty => fmt_poly_pretty(self.ring, self.poly, self.vars, f),
        }
    }
}

fn fmt_poly_tuple<F, O, P>(ring: &RingCtx<F, O>, poly: &P, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView<Coeff = F::Elem>,
    <P::Term as TermView>::Mono: MonomialView<Word = u32>,
{
    // Format: [(c,[...]),(c,[...])]
    // delegated term formatting:
    let mut first = true;
    write!(f, "[")?;
    for t in poly.terms() {
        if !first {
            write!(f, ",")?;
        }
        first = false;
        write!(f, "{}", TermDisplay::tuple(ring, t))?;
    }
    write!(f, "]")
}

fn fmt_poly_pretty<F, O, P>(ring: &RingCtx<F, O>, poly: &P, vars: &[String], f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView<Coeff = F::Elem>,
    <P::Term as TermView>::Mono: MonomialView<Word = u32>,
{
    let p_mod = ring.field.modulus_u32();
    let mut wrote_any = false;

    for t in poly.terms() {
        // skip zero terms just in case
        let cu = ring.field.repr_u32(*t.coeff());
        if cu == 0 {
            continue;
        }

        let signed = to_signed_rep(cu, p_mod);
        if signed == 0 {
            continue;
        }

        if !wrote_any {
            // first term: TermDisplay will include '-' if needed
            write!(f, "{}", TermDisplay::pretty_with_vars(ring, t, vars))?;
            wrote_any = true;
            continue;
        }

        write!(f, " + ")?;
        write!(f, "{}", TermDisplay::pretty_with_vars(ring, t, vars))?;
    }

    if !wrote_any {
        write!(f, "0")?;
    }

    Ok(())
}
/// Converts a non-negative modular representative into a signed integer.
///
/// If a modulus `p` is provided, values greater than `p/2` are mapped
/// into the negative range using:
///
/// `c -> c - p`
///
/// This produces symmetric representatives in the interval:
///
/// `[-⌊p/2⌋, ⌊p/2⌋]`
///
/// If no modulus is provided, the value is interpreted directly.
///
/// # Example (mod 7)
///
/// ```text
/// 6  -> -1
/// 5  -> -2
/// 3  ->  3
/// ```
fn to_signed_rep(c: u32, p: Option<u32>) -> i64 {
    match p {
        Some(p) if p != 0 => {
            let c = c as i64;
            let p = p as i64;
            if c > p / 2 { c - p } else { c }
        }
        _ => c as i64,
    }
}
