//! Gröbner basis algorithms (context-driven).
//!
//! This module provides a correctness-first Buchberger baseline.
//!
//! The algorithms are **context-driven** via [`RingCtx`](crate::ring::RingCtx):
//! - coefficient arithmetic uses `ctx.field`
//! - monomial order (used by normalization/reduction) uses `ctx.order`
//! - ring mixing is detected using the [`RingId`](crate::ring::RingId) tag stored in polynomials
//!
//! ## Main entry points
//! - [`buchberger`] (defaults: LIFO pair processing, no pairing pruning)
//! - [`buchberger_with`] (custom pair queue + pairing)
//!
//! ## Errors
//! Errors are reported as [`BuchbergerError`] and include:
//! - polynomial/term/monomial/ring errors
//! - S-polynomial construction errors
//! - reduction errors
//!
//! ## Panics
//! None (beyond panics in user-provided storage backends).

mod algos;
mod basis;
mod display;
mod error;
mod pairing;
mod pairs;
mod trace;

pub use algos::*;
pub use basis::GrobnerBasis;
pub use display::{GbDisplay, GbStyle};
pub use error::BuchbergerError;
pub use pairing::*;
pub use pairs::{FifoPairs, HeapPairs, Pair, PairQueue, StackPairs};
pub use trace::*;
