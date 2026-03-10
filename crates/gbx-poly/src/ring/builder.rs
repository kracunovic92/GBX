//! Ergonomic ring construction.
//!
//! Users can build a ring context without spelling the full `RingCtx::<F,O>` type.
//!
//! # Panics
//! `build()` will panic if any required component is missing.

use crate::order::MonomialOrder;
use crate::ring::ctx::RingCtx;
use crate::ring::error::Result;
use crate::ring::field::FieldCtx;

/// Builder for `RingCtx`.
///
/// Type-state:
/// - `O = ()` means “order not set yet”
/// - after calling `.order(...)`, builder becomes `RingBuilder<F, O>`
#[allow(missing_docs)]
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
    /// Provide the field arithmetic context.
    #[inline]
    pub fn field(mut self, field: F) -> Self {
        self.field = Some(field);
        self
    }

    /// Provide the number of variables.
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
    /// Provide the monomial order. This *sets the builder type parameter*.
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
    /// Build the ring context.
    ///
    /// # Errors
    /// Returns an error if `nvars == 0`.
    ///
    /// # Panics
    /// Panics if any builder component is missing (`field`, `order`, `nvars`).
    pub fn build(self) -> Result<RingCtx<F, O>> {
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
    use crate::ring::{Ring, RingError, StaticFpCtx};
    use gbx_field::fp::FpDyn;

    #[test]
    fn builder_builds_static() {
        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(3)
            .build()
            .unwrap();

        assert_eq!(ring.nvars, 3);
    }

    #[test]
    fn builder_builds_dynamic() {
        let ring = Ring::builder()
            .field(FpDyn::prime(7).unwrap())
            .order(Lex)
            .nvars(3)
            .build()
            .unwrap();

        assert_eq!(ring.nvars, 3);
    }

    #[test]
    fn builder_errors_on_nvars_zero() {
        let err = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(0)
            .build()
            .unwrap_err();

        assert!(matches!(err, RingError::InvalidNvars { .. }));
    }

    #[test]
    #[should_panic(expected = "ring builder missing field")]
    fn builder_panics_when_field_missing() {
        let _ = RingBuilder::<StaticFpCtx<7>, ()>::default()
            .order(Lex)
            .nvars(2)
            .build();
    }

    // With the type-state builder, "missing order" is now a compile-time issue
    // because build() doesn't exist until .order(..) is called.
    //
    // If you *want* a runtime panic test for missing order, keep Option A instead.
}
