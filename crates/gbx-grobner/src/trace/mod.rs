//! Shared tracing framework for algorithm instrumentation.
//!
//! This module provides the generic building blocks used by algorithm-specific
//! tracers:
//!
//! - collection levels,
//! - high-level reporting modes,
//! - shared tracing configuration,
//! - a thread-safe shared handle for mutable trace state.
//!
//! Algorithm-specific trace modules should build on top of this framework by
//! defining their own counters, snapshots, event methods, and reporters.

pub mod config;
pub mod handle;
pub mod level;
pub mod reporter;

pub use config::TraceConfig;
pub use handle::TraceHandle;
pub use level::TraceLevel;
pub use reporter::TraceReportMode;
