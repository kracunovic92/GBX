//! F4-specific tracing and reporting.
//!
//! This module contains the tracing subsystem used by the F4 engine.
//!
//! It provides:
//! - configuration,
//! - counters and timing buckets,
//! - trace snapshots,
//! - a mutable tracer state,
//! - a thin optional tracing context,
//! - pure reporting helpers.
//!
//! Mechanical queue/pairing tracing is handled by reusable wrappers in the
//! pairing tracing module. The F4 tracer implements the shared trace event
//! traits required by those wrappers.

mod config;
mod context;
mod counters;
mod reporter;
mod snapshot;
mod timings;
mod tracer;

pub use config::F4TraceConfig;
pub use context::F4TraceCtx;
pub use counters::F4TraceCounters;
pub use reporter::{format_batch_line, format_progress_line, format_summary_lines, print_batch_line, print_progress_line, print_summary};
pub use snapshot::F4TraceSnapshot;
pub use timings::{WhileKind, WhileTimes};
pub use tracer::{F4Tracer, SharedF4Tracer};
