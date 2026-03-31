//! Shared tracing framework for algorithm instrumentation.
//!
//! This module provides the generic building blocks used by algorithm-specific
//! tracers:
//!
//! - collection levels,
//! - reporting modes,
//! - shared tracing configuration,
//! - a thread-safe shared handle,
//! - reusable trace event traits,
//! - generic mechanical counters,
//! - generic timing / memory helpers.
//!
//! Algorithm-specific trace modules should build on top of this framework by
//! defining their own counters, snapshots, semantic events, and reporters.

pub mod config;
pub mod counters;
pub mod events;
pub mod format;
pub mod handle;
pub mod level;
pub mod memory;
pub mod phases;
pub mod reporter;
pub mod timing;

pub use config::TraceConfig;
pub use counters::MechanicalTraceCounters;
pub use events::{BasisTrace, PairingTrace, QueueTrace, ReductionTrace};
pub use format::{fmt_bytes, fmt_duration_ms};
pub use handle::TraceHandle;
pub use level::TraceLevel;
pub use memory::{current_memory_snapshot, MemorySnapshot};
pub use phases::{CorePhaseKind, CorePhaseTimes};
pub use reporter::TraceReportMode;
pub use timing::measure_duration;
