//! Builder for [`RingCtx`](RingCtx).

use crate::order::MonomialOrder;
use crate::ring::ctx::RingCtx;
use crate::ring::field::FieldCtx;
use crate::ring::{RingError, RingResult};

/// Builder for constructing a [`RingCtx`].
///
/// The monomial order is type-state checked:
/// `build()` is only available after `.order(...)` has been called.
#[derive(Debug)]
pub struct RingBuilder<F, O = ()> {
    field: Option<F>,
    order: Option<O>,
    nvars: Option<usize>,
}

impl<F> Default for RingBuilder<F, ()>
where
    F: FieldCtx,
{
    #[inline]
    fn default() -> Self {
        Self { field: None, order: None, nvars: None }
    }
}

impl<F, O> RingBuilder<F, O>
where
    F: FieldCtx,
{
    /// Sets the coefficient field context.
    #[inline]
    #[must_use]
    pub fn field(mut self, field: F) -> Self {
        self.field = Some(field);
        self
    }

    /// Sets the number of variables in the polynomial ring.
    #[inline]
    #[must_use]
    pub const fn nvars(mut self, nvars: usize) -> Self {
        self.nvars = Some(nvars);
        self
    }
}

impl<F> RingBuilder<F, ()>
where
    F: FieldCtx,
{
    /// Sets the monomial order.
    #[inline]
    #[must_use]
    pub fn order<O2>(self, order: O2) -> RingBuilder<F, O2>
    where
        O2: MonomialOrder,
    {
        RingBuilder { field: self.field, order: Some(order), nvars: self.nvars }
    }
}

impl<F, O> RingBuilder<F, O>
where
    F: FieldCtx,
    O: MonomialOrder,
{
    /// Builds the ring context.
    ///
    /// # Errors
    ///
    /// Returns an error if the ring configuration itself is invalid.
    pub fn build(self) -> RingResult<RingCtx<F, O>> {
        let field = self.field.ok_or(RingError::MissingField)?;
        let order = self.order.ok_or(RingError::MissingOrder)?;
        let nvars = self.nvars.ok_or(RingError::MissingNvars)?;

        RingCtx::new(field, order, nvars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::Lex;
    use crate::ring::{Ring, RingError};
    use gbx_field::fp::Fp;

    fn field_7() -> Fp {
        match Fp::prime(7) {
            Ok(field) => field,
            Err(err) => panic!("7 should be prime: {err}"),
        }
    }

    #[test]
    fn builder_builds_ring() {
        let Ok(ring) = Ring::builder().field(field_7()).order(Lex).nvars(3).build() else {
            panic!("valid ring should build");
        };

        assert_eq!(ring.nvars, 3);
    }

    #[test]
    fn builder_errors_on_nvars_zero() {
        let Err(err) = Ring::builder().field(field_7()).order(Lex).nvars(0).build() else {
            panic!("zero-variable ring should be invalid");
        };

        assert!(matches!(err, RingError::InvalidNvars { .. }));
    }

    #[test]
    fn builder_errors_when_field_missing() {
        assert!(matches!(
            RingBuilder::<Fp, ()>::default().order(Lex).nvars(2).build(),
            Err(RingError::MissingField)
        ));
    }

    #[test]
    fn builder_errors_when_nvars_missing() {
        assert!(matches!(
            Ring::builder().field(field_7()).order(Lex).build(),
            Err(RingError::MissingNvars)
        ));
    }
}
