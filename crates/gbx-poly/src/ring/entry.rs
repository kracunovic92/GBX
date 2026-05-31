//! Entry point for constructing ring contexts.

use crate::ring::FieldCtx;
use crate::ring::builder::RingBuilder;

/// Ergonomic entry point for creating a [`RingCtx`](crate::ring::RingCtx).
///
/// Use [`Ring::builder`] instead of spelling the full generic type manually.
#[derive(Debug, Copy, Clone)]
pub struct Ring;

impl Ring {
    /// Starts building a ring context.
    ///
    /// # Example
    ///
    /// ```
    /// use gbx_field::fp::Fp;
    /// use gbx_poly::order::Lex;
    /// use gbx_poly::ring::Ring;
    ///
    /// let ring = Ring::builder()
    ///     .field(Fp::prime(32003).unwrap())
    ///     .order(Lex)
    ///     .nvars(4)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(ring.nvars, 4);
    /// ```
    #[inline]
    #[must_use]
    pub fn builder<F>() -> RingBuilder<F, ()>
    where
        F: FieldCtx,
    {
        RingBuilder::default()
    }
}
