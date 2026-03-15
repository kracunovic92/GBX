//! Buchberger-specific tracing and reporting.
//!
//! This module contains the tracing subsystem used by the Buchberger engine.
//!
//! It provides:
//! - configuration,
//! - counters and timing buckets,
//! - trace snapshots,
//! - a mutable tracer state,
//! - wrappers for mechanical events,
//! - pure reporting helpers.
//!
//! # Responsibility split
//!
//! - [`BuchbergerTracer`] collects semantic and mechanical events.
//! - wrapper adapters collect low-level mechanical events automatically.
//! - reporter helpers format or print immutable snapshots.
//!
//! The Buchberger engine should depend on the high-level tracer API rather than
//! mutating counter/timing fields directly.

mod config;
mod context;
mod counters;
mod memory;
mod reporter;
mod snapshot;
mod timings;
mod tracer;
mod wrappers;

pub use config::BuchbergerTraceConfig;
pub use context::BuchbergerTraceCtx;
pub use counters::BuchbergerTraceCounters;
pub use memory::{current_memory_snapshot, MemorySnapshot};
pub use reporter::{fmt_bytes, format_iteration_line, format_progress_line, format_summary_lines, print_iteration_line, print_progress_line, print_summary};
pub use snapshot::BuchbergerTraceSnapshot;
pub use timings::{measure_duration, PhaseKind, PhaseTimes, WhileKind, WhileTimes};
pub use tracer::{BuchbergerTracer, SharedBuchbergerTracer};
pub use wrappers::TracingQueue;
