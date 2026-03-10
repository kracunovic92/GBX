//! Ring context and context-driven field arithmetic.
//!
//! This module defines the **ring context** used by polynomial operations and
//! Gröbner basis algorithms.
//!
//! # Design goals
//! - Keep polynomials/terms **thin**: no modulus/order/nvars stored per term.
//! - Ensure all polynomial operations occur in one consistent **ring context**.
//! - Support both:
//!   - **dynamic** prime fields (`FpDyn` + `FpDynElem` where arithmetic lives in the ctx)
//!   - **static** prime fields (`Fp<P>` where arithmetic lives on the element type)
//!
//! # What lives in `RingCtx`?
//! - field arithmetic provider (`FieldCtx`)
//! - term order value `O` (often a ZST like `Lex`)
//! - `nvars` (number of variables)
//! - a unique `RingId` for mismatch detection
//!
//! # What does NOT live in polynomials?
//! - modulus `p`
//! - term order
//! - number of variables
//!
//! Instead, polynomial operations take `&RingCtx` explicitly and can validate
//! ring identity via `RingId`.

mod builder;
mod ctx;
mod error;
mod field;
mod ring;

pub use builder::RingBuilder;
pub use ctx::{RingCtx, RingId};
pub use error::{Result, RingError};
pub use field::{FieldCtx, StaticFpCtx};
pub use ring::Ring;
