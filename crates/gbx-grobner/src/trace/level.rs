/// Coarse-grained tracing collection level.
///
/// Higher levels include the capabilities of lower levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TraceLevel {
    /// Tracing is disabled.
    #[default]
    Off,

    /// Collect cheap counters only.
    Counters,

    /// Collect counters and timings.
    Timings,

    /// Collect counters, timings, and periodic snapshots.
    Snapshots,

    /// Collect all supported tracing data, including verbose diagnostics.
    Verbose,
}

impl TraceLevel {
    /// Returns whether this level collects counters.
    #[must_use]
    #[inline]
    pub const fn collects_counters(self) -> bool {
        !matches!(self, Self::Off)
    }

    /// Returns whether this level collects timings.
    #[must_use]
    #[inline]
    pub const fn collects_timings(self) -> bool {
        matches!(self, Self::Timings | Self::Snapshots | Self::Verbose)
    }

    /// Returns whether this level collects snapshots.
    #[must_use]
    #[inline]
    pub const fn collects_snapshots(self) -> bool {
        matches!(self, Self::Snapshots | Self::Verbose)
    }

    /// Returns whether this level enables verbose tracing behavior.
    #[must_use]
    #[inline]
    pub const fn is_verbose(self) -> bool {
        matches!(self, Self::Verbose)
    }
}
