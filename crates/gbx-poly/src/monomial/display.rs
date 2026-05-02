use core::fmt;

use crate::monomial::MonomialView;

/// Monomial formatting style.
#[derive(Debug, Clone, Copy)]
pub enum MonomialStyle {
    /// Product form, for example `x0^2*x3`.
    Product,

    /// Raw exponent vector, for example `[0,1,2,0]`.
    Exponents,
}

/// Display helper for monomials.
#[derive(Debug)]
pub struct MonomialDisplay<'a, M> {
    mono: &'a M,
    vars: Option<&'a [String]>,
    style: MonomialStyle,
}

impl<'a, M> MonomialDisplay<'a, M>
where
    M: MonomialView,
{
    /// Creates a display adapter that renders the monomial in product form.
    ///
    /// Variables are printed as `x0`, `x1`, ...
    ///
    /// # Example
    ///
    /// ```ignore
    /// let s = MonomialDisplay::product(&m).to_string();
    /// ```
    #[inline]
    pub fn product(mono: &'a M) -> Self {
        Self { mono, vars: None, style: MonomialStyle::Product }
    }

    /// Creates a display adapter that renders the monomial in product form
    /// using custom variable names.
    ///
    /// If `vars` does not contain a name for some variable index, that variable
    /// falls back to `x{i}`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let vars = vec!["x".to_string(), "y".to_string()];
    /// let s = MonomialDisplay::product_with_vars(&m, &vars).to_string();
    /// ```
    #[inline]
    pub fn product_with_vars(mono: &'a M, vars: &'a [String]) -> Self {
        Self { mono, vars: Some(vars), style: MonomialStyle::Product }
    }

    /// Creates a display adapter that renders the raw exponent vector.
    ///
    /// For example, a monomial with exponents `[1, 0, 2]` is displayed as
    /// `[1,0,2]`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let s = MonomialDisplay::exponents(&m).to_string();
    /// ```
    #[inline]
    pub fn exponents(mono: &'a M) -> Self {
        Self { mono, vars: None, style: MonomialStyle::Exponents }
    }
}

impl<M> fmt::Display for MonomialDisplay<'_, M>
where
    M: MonomialView,
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

                    let var_name = self.vars.and_then(|vars| vars.get(i));

                    match (var_name, e) {
                        (Some(v), 1) => write!(f, "{v}")?,
                        (Some(v), _) => write!(f, "{v}^{e}")?,
                        (None, 1) => write!(f, "x{i}")?,
                        (None, _) => write!(f, "x{i}^{e}")?,
                    }
                }

                if first {
                    write!(f, "1")?;
                }

                Ok(())
            }
        }
    }
}
