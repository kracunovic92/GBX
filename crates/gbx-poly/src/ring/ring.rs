//! Entry point for building ring contexts.
//!
//! This exists mostly for ergonomics: `Ring::builder()`.

use crate::ring::builder::RingBuilder;
use crate::ring::FieldCtx;

/// Entry point type for `Ring::builder()`.
///
/// # Example
/// ```
/// use gbx_poly::ring::{Ring, StaticFpCtx};
/// use gbx_poly::order::Lex;
///
/// let ring = Ring::builder()
///     .field(StaticFpCtx::<7>::new())
///     .order(Lex)
///     .nvars(3)
///     .build()
///     .unwrap();
///
/// assert_eq!(ring.nvars, 3);
/// ```
///
/// See [`RingBuilder`] for full configuration details.
#[derive(Debug, Copy, Clone)]
pub struct Ring;

impl Ring {
    /// Creates a new [`RingBuilder`] for constructing a ring context.
    ///
    /// The generic parameter `F` specifies the field context type.
    ///
    /// # Type Parameters
    ///
    /// - `F`: must implement [`FieldCtx`]
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = Ring::builder::<MyFieldCtx>();
    /// ```
    ///
    /// In practice, the type parameter is inferred from `.field(...)`.
    #[inline]
    pub fn builder<F>() -> RingBuilder<F, ()>
    where
        F: FieldCtx,
    {
        RingBuilder::default()
    }
}
