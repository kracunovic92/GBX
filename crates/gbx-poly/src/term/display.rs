use core::fmt;

use crate::monomial::{MonomialDisplay, MonomialView};
use crate::order::MonomialOrder;
use crate::ring::{FieldCtx, RingCtx};
use crate::term::TermView;

/// Formatting style for a single polynomial term.
///
/// This determines how a term is rendered when using [`TermDisplay`].
///
/// # Variants
///
/// - [`Pretty`](TermStyle::Pretty): human-readable mathematical form
///   (e.g. `3*x*y^2`, `-x^2`, `5`)
///
/// - [`Tuple`](TermStyle::Tuple): stable machine-readable representation
///   (e.g. `(3,[1,2,0])`)
///
/// The tuple style is intended for:
///
/// - Snapshot testing
/// - Diffing Gröbner bases
/// - Stable cross-tool comparisons (e.g. Singular integration)
#[derive(Debug, Clone, Copy)]
pub enum TermStyle {
    /// Human-readable mathematical format.
    ///
    /// Examples:
    /// - `3*x*y^2`
    /// - `-x`
    /// - `5`
    Pretty,

    /// Stable tuple representation.
    ///
    /// Format:
    /// `(coeff,[e0,e1,...])`
    ///
    /// Example:
    /// `(3,[1,2,0])`
    Tuple,
}

/// Display adapter for a single polynomial term.
///
/// This type provides formatting logic for terms while keeping
/// term types independent of any specific display policy.
#[derive(Debug)]
pub struct TermDisplay<'a, F, O, T>
where
    F: FieldCtx,
    O: MonomialOrder,
    T: TermView<Coeff = F::Elem>,
{
    ring: &'a RingCtx<F, O>,
    term: &'a T,
    vars: Option<&'a [String]>,
    style: TermStyle,
}

impl<'a, F, O, T> TermDisplay<'a, F, O, T>
where
    F: FieldCtx,
    O: MonomialOrder,
    T: TermView<Coeff = F::Elem>,
{
    /// Creates a human-readable formatter without custom variable names.
    ///
    /// Variables will be rendered using default monomial formatting.
    pub fn pretty(ring: &'a RingCtx<F, O>, term: &'a T) -> Self {
        Self { ring, term, vars: None, style: TermStyle::Pretty }
    }

    /// Creates a human-readable formatter with custom variable names.
    ///
    /// The provided `vars` slice must correspond to the variable ordering
    /// of the ring context.
    ///
    /// # Example
    ///
    /// ```
    /// // vars = ["x", "y", "z"]
    /// // term prints as 3*x*y^2
    /// ```
    pub fn pretty_with_vars(ring: &'a RingCtx<F, O>, term: &'a T, vars: &'a [String]) -> Self {
        Self { ring, term, vars: Some(vars), style: TermStyle::Pretty }
    }

    /// Creates a stable tuple formatter.
    ///
    /// Intended for:
    ///
    /// - Deterministic output
    /// - Testing and diffing
    /// - External CAS comparison
    pub fn tuple(ring: &'a RingCtx<F, O>, term: &'a T) -> Self {
        Self { ring, term, vars: None, style: TermStyle::Tuple }
    }
}

impl<'a, F, O, T> fmt::Display for TermDisplay<'a, F, O, T>
where
    F: FieldCtx,
    O: MonomialOrder,
    T: TermView<Coeff = F::Elem>,
    T::Mono: MonomialView<Word = u32>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c_u = self.ring.field.repr_u32(*self.term.coeff());
        let mono = self.term.mono();

        match self.style {
            TermStyle::Tuple => {
                // (coeff,[e..])
                write!(f, "({},{})", c_u, MonomialDisplay::exponents(mono))
            }
            TermStyle::Pretty => {
                let is_one = mono.exponents().iter().all(|&e| e == 0);
                if is_one {
                    if let Some(p) = self.ring.field.modulus_u32() {
                        let s = to_signed_rep(c_u, Some(p));
                        write!(f, "{s}")
                    } else {
                        write!(f, "{c_u}")
                    }
                } else {
                    let signed = to_signed_rep(c_u, self.ring.field.modulus_u32());
                    let abs = signed.abs();

                    let mono_disp = match self.vars {
                        Some(vs) => MonomialDisplay::product_with_vars(mono, vs),
                        None => MonomialDisplay::product(mono),
                    };

                    if abs == 1 {
                        if signed < 0 { write!(f, "-{mono_disp}") } else { write!(f, "{mono_disp}") }
                    } else {
                        if signed < 0 { write!(f, "-{}*{}", abs, mono_disp) } else { write!(f, "{}*{}", abs, mono_disp) }
                    }
                }
            }
        }
    }
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
