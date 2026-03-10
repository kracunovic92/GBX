use crate::monomial::MonomialView;
use std::fmt;

/// Monomial formatting style.
#[derive(Debug, Clone, Copy)]
pub enum MonomialStyle {
    /// `x0^2*x3` (or with provided var names).
    Product,
    /// `[0,1,2,0]`
    Exponents,
}

#[derive(Debug)]
/// Display helper for any `MonomialView`.
pub struct MonomialDisplay<'a, M> {
    mono: &'a M,
    vars: Option<&'a [String]>,
    style: MonomialStyle,
}

#[allow(missing_docs)]
impl<'a, M> MonomialDisplay<'a, M>
where
    M: MonomialView,
{
    pub fn product(mono: &'a M) -> Self {
        Self { mono, vars: None, style: MonomialStyle::Product }
    }

    pub fn product_with_vars(mono: &'a M, vars: &'a [String]) -> Self {
        Self { mono, vars: Some(vars), style: MonomialStyle::Product }
    }

    pub fn exponents(mono: &'a M) -> Self {
        Self { mono, vars: None, style: MonomialStyle::Exponents }
    }
}
impl<'a, M> fmt::Display for MonomialDisplay<'a, M>
where
    M: MonomialView<Word = u32>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.style {
            MonomialStyle::Exponents => {
                write!(f, "[")?;
                for (i, &e) in self.mono.exponents().iter().enumerate() {
                    if i != 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{e}")?;
                }
                write!(f, "]")
            }
            MonomialStyle::Product => {
                let mut first = true;
                for (i, &e) in self.mono.exponents().iter().enumerate() {
                    if e == 0 {
                        continue;
                    }
                    if !first {
                        write!(f, "*")?;
                    }
                    first = false;

                    let v = self
                        .vars
                        .and_then(|vs| vs.get(i))
                        .map(|s| s.as_str())
                        .unwrap_or_else(|| {
                            // fallback x0, x1, ...
                            // (no allocation)
                            // We'll print "x{i}" directly below.
                            ""
                        });

                    if v.is_empty() {
                        if e == 1 {
                            write!(f, "x{i}")?;
                        } else {
                            write!(f, "x{i}^{e}")?;
                        }
                    } else {
                        if e == 1 {
                            write!(f, "{v}")?;
                        } else {
                            write!(f, "{v}^{e}")?;
                        }
                    }
                }

                if first { write!(f, "1") } else { Ok(()) }
            }
        }
    }
}
