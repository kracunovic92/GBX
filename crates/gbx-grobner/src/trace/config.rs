use crate::trace::level::TraceLevel;
use crate::trace::reporter::TraceReportMode;

/// Global tracing configuration shared by all algorithms.
///
/// This controls collection policy and high-level reporting policy. It does not
/// define algorithm-specific events or output formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceConfig {
    /// Overall tracing collection level.
    pub level: TraceLevel,

    /// High-level reporting mode.
    pub report_mode: TraceReportMode,

    /// Record a snapshot every `N` logical steps.
    ///
    /// A value of `0` disables periodic snapshots.
    pub snapshot_every: usize,

    /// Sample memory every `N` logical steps.
    ///
    /// A value of `0` disables periodic memory sampling.
    pub memory_every: usize,

    /// Whether final memory usage should be sampled.
    pub final_memory: bool,
}

impl TraceConfig {
    /// Returns a configuration with tracing fully disabled.
    #[must_use]
    #[inline]
    pub const fn off() -> Self {
        Self { level: TraceLevel::Off, report_mode: TraceReportMode::Silent, snapshot_every: 0, memory_every: 0, final_memory: false }
    }

    /// Returns a counters-only configuration.
    #[must_use]
    #[inline]
    pub const fn counters() -> Self {
        Self { level: TraceLevel::Counters, report_mode: TraceReportMode::Silent, snapshot_every: 0, memory_every: 0, final_memory: false }
    }

    /// Returns a counters+timings configuration.
    #[must_use]
    #[inline]
    pub const fn timings() -> Self {
        Self { level: TraceLevel::Timings, report_mode: TraceReportMode::Silent, snapshot_every: 0, memory_every: 0, final_memory: false }
    }

    /// Returns a snapshot-oriented configuration.
    #[must_use]
    #[inline]
    pub const fn snapshots(snapshot_every: usize) -> Self {
        Self { level: TraceLevel::Snapshots, report_mode: TraceReportMode::Summary, snapshot_every, memory_every: 0, final_memory: false }
    }

    /// Returns a verbose tracing configuration.
    #[must_use]
    #[inline]
    pub const fn verbose(snapshot_every: usize) -> Self {
        Self { level: TraceLevel::Verbose, report_mode: TraceReportMode::Verbose, snapshot_every, memory_every: 0, final_memory: false }
    }

    /// Returns whether this configuration collects counters.
    #[must_use]
    #[inline]
    pub const fn collects_counters(self) -> bool {
        self.level.collects_counters()
    }

    /// Returns whether this configuration collects timings.
    #[must_use]
    #[inline]
    pub const fn collects_timings(self) -> bool {
        self.level.collects_timings()
    }

    /// Returns whether this configuration collects snapshots.
    #[must_use]
    #[inline]
    pub const fn collects_snapshots(self) -> bool {
        self.level.collects_snapshots()
    }

    /// Returns whether this configuration should sample memory.
    #[must_use]
    #[inline]
    pub const fn collects_memory(self) -> bool {
        self.memory_every != 0 || self.final_memory
    }

    /// Returns whether reporting is silent.
    #[must_use]
    #[inline]
    pub const fn is_silent(self) -> bool {
        matches!(self.report_mode, TraceReportMode::Silent)
    }

    /// Returns whether summary reporting is enabled.
    #[must_use]
    #[inline]
    pub const fn reports_summary(self) -> bool {
        matches!(
            self.report_mode,
            TraceReportMode::Summary | TraceReportMode::Verbose
        )
    }

    /// Returns whether verbose reporting is enabled.
    #[must_use]
    #[inline]
    pub const fn reports_verbose(self) -> bool {
        matches!(self.report_mode, TraceReportMode::Verbose)
    }
}

impl Default for TraceConfig {
    fn default() -> Self {
        Self::off()
    }
}
