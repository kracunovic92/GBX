//! Builder for [`RingCtx`](RingCtx).

use crate::order::MonomialOrder;
use crate::ring::ctx::RingCtx;
use crate::ring::field::FieldCtx;
use crate::ring::RingResult;

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
    pub fn field(mut self, field: F) -> Self {
        self.field = Some(field);
        self
    }

    /// Sets the number of variables in the polynomial ring.
    #[inline]
    pub fn nvars(mut self, nvars: usize) -> Self {
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
    /// Returns an error if the ring configuration itself is invalid.
    ///
    /// # Panics
    ///
    /// Panics if `field` or `nvars` were not provided.
    /// Missing `order` is prevented at compile time.
    pub fn build(self) -> RingResult<RingCtx<F, O>> {
        RingCtx::new(
            self.field.expect("ring builder missing field"),
            self.order.expect("ring builder missing order"),
            self.nvars.expect("ring builder missing nvars"),
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::order::Lex;
    use crate::ring::{Ring, RingError};
    use gbx_field::fp::Fp;

    #[test]
    fn builder_builds_ring() {
        let ring = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .nvars(3)
            .build()
            .unwrap();

        assert_eq!(ring.nvars, 3);
    }

    #[test]
    fn builder_errors_on_nvars_zero() {
        let err = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .nvars(0)
            .build()
            .unwrap_err();

        assert!(matches!(err, RingError::InvalidNvars { .. }));
    }

    #[test]
    #[should_panic(expected = "ring builder missing field")]
    fn builder_panics_when_field_missing() {
        let _ = RingBuilder::<Fp, ()>::default().order(Lex).nvars(2).build();
    }

    #[test]
    #[should_panic(expected = "ring builder missing nvars")]
    fn builder_panics_when_nvars_missing() {
        let _ = Ring::builder()
            .field(Fp::prime(7).unwrap())
            .order(Lex)
            .build();
    }
}
