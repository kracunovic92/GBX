use crate::order::MonomialOrder;
use crate::polynomial::PolynomialView;
use crate::ring::{FieldCtx, RingCtx};
use crate::term::TermDisplay;
use core::fmt;

/// Formatting style for polynomial display.
#[derive(Debug, Clone, Copy)]
pub enum PolyStyle {
    /// Human-readable form.
    Pretty,

    /// Stable tuple dump.
    TupleDump,
}

/// Display adapter for a polynomial.
#[derive(Debug)]
pub struct PolyDisplay<'a, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView<Coeff = F::Elem>,
{
    ring: &'a RingCtx<F, O>,
    poly: &'a P,
    vars: &'a [String],
    style: PolyStyle,
}

#[allow(missing_docs)]
impl<'a, F, O, P> PolyDisplay<'a, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView<Coeff = F::Elem>,
{
    pub fn pretty(ring: &'a RingCtx<F, O>, poly: &'a P, vars: &'a [String]) -> Self {
        Self { ring, poly, vars, style: PolyStyle::Pretty }
    }

    pub fn tuple_dump(ring: &'a RingCtx<F, O>, poly: &'a P, vars: &'a [String]) -> Self {
        Self { ring, poly, vars, style: PolyStyle::TupleDump }
    }
}

impl<F, O, P> fmt::Display for PolyDisplay<'_, F, O, P>
where
    F: FieldCtx,
    O: MonomialOrder,
    P: PolynomialView<Coeff = F::Elem>,
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
    P: PolynomialView<Coeff = F::Elem>,
{
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
    P: PolynomialView<Coeff = F::Elem>,
{
    let p_mod = ring.field.modulus_u32();
    let mut wrote_any = false;

    for t in poly.terms() {
        let cu = ring.field.repr_u32(*t.coeff());

        if cu == 0 {
            continue;
        }

        let signed = to_signed_rep(cu, p_mod);

        if signed == 0 {
            continue;
        }

        if !wrote_any {
            write!(f, "{}", TermDisplay::pretty_with_vars(ring, t, vars))?;
            wrote_any = true;
            continue;
        }

        if signed < 0 {
            write!(f, " - ")?;
        } else {
            write!(f, " + ")?;
        }

        let abs_coeff = signed.abs() as u32;
        let abs_term = crate::term::Term::new(ring.field.new(abs_coeff), t.mono().clone());

        write!(
            f,
            "{}",
            TermDisplay::pretty_with_vars(ring, &abs_term, vars)
        )?;
    }

    if !wrote_any {
        write!(f, "0")?;
    }

    Ok(())
}

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
