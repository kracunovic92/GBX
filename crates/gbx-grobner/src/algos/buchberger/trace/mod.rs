//! Buchberger-specific tracing and progress reporting.
//!
//! This module contains tracing infrastructure tailored to Buchberger's
//! algorithm:
//!
//! - queue/criterion/key wrappers,
//! - semantic counters,
//! - phase and loop timings,
//! - optional process-memory sampling,
//! - progress and summary reporting.
//!
//! # Design
//!
//! Mechanical events such as queue push/pop and criterion rejection are traced
//! by wrappers.
//!
//! Semantic events such as:
//!
//! - initial basis prepared,
//! - remainder reduced to zero,
//! - nonzero remainder inserted,
//! - unit remainder found,
//! - phase boundaries,
//!
//! should be emitted directly by the Buchberger engine.
//!
//! This avoids ambiguity during initial seeding versus true basis growth.

mod config;
mod counters;
mod memory;
mod reporter;
mod snapshot;
mod timings;
mod tracer;
mod wrappers;

pub use config::TraceCfg;
pub use counters::TraceCounters;
pub use memory::{current_memory_snapshot, MemorySnapshot};
pub use reporter::{fmt_bytes, print_progress_line, print_summary};
pub use snapshot::TraceSnapshot;
pub use timings::{measure_duration, PhaseTimes, WhileTimes};
pub use tracer::{SharedTracer, Tracer};
pub use wrappers::{TracingPairCriterion, TracingPairKey, TracingQueue};
