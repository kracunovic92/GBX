//! Tracing decorators for pairing-layer components.
//!
//! This module contains lightweight wrappers around pairing abstractions such as
//! local criteria, state-aware filters, keys, and update strategies.
//!
//! These wrappers do not change algorithmic behavior. They only forward calls
//! while reporting events to the shared Buchberger tracer.

mod tracing_pair_criterion;
mod tracing_pair_filter;
mod tracing_pair_key;
mod tracing_pair_update;

pub use tracing_pair_criterion::*;
pub use tracing_pair_filter::*;
pub use tracing_pair_key::*;
pub use tracing_pair_update::*;
