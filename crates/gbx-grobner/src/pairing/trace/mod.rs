//! Tracing decorators for pairing-layer components.
//!
//! This module contains lightweight wrappers around reusable pairing
//! abstractions such as queues, local criteria, state-aware filters, keys, and
//! update strategies.
//!
//! These wrappers do not change algorithmic behavior. They only forward calls
//! while reporting mechanical events to a shared tracer handle.
//!
//! The tracer type is generic. Any tracer implementing the required trace event
//! traits can be used, which makes these wrappers reusable across Buchberger,
//! F4, and future algorithms.

mod tracing_pair_criterion;
mod tracing_pair_filter;
mod tracing_pair_key;
mod tracing_pair_update;
mod tracing_queue;

pub use tracing_pair_criterion::TracingPairCriterion;
pub use tracing_pair_filter::TracingPairFilter;
pub use tracing_pair_key::TracingPairKey;
pub use tracing_pair_update::TracingPairUpdate;
pub use tracing_queue::TracingQueue;
