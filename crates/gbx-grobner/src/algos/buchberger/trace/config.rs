use crate::trace::TraceConfig;

/// Buchberger-specific tracing policy.
///
/// `core` controls generic tracing behavior shared across algorithms.
/// The remaining fields control Buchberger-specific progress sampling and
/// output verbosity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuchbergerTraceConfig {
    /// Generic tracing policy shared across algorithms.
    pub core: TraceConfig,

    /// Emit progress after every `N` successful queue pops.
    ///
    /// `0` disables periodic progress output.
    pub progress_every: usize,

    /// Print a line whenever a nonzero remainder is inserted.
    pub print_on_insert: bool,

    /// Print a compact line for every processed pair.
    pub print_each_iteration: bool,

    /// Include per-step timings in iteration output.
    ///
    /// Has no effect unless `print_each_iteration` is enabled.
    pub print_iteration_timings: bool,

    /// Print phase totals in the final summary.
    pub print_phase_summary: bool,

    /// Print while-loop breakdown totals in the final summary.
    pub print_breakdown: bool,

    /// Include memory usage in periodic progress output.
    pub sample_memory_on_progress: bool,

    /// Include memory usage in the final summary.
    pub sample_memory_on_summary: bool,
}

impl Default for BuchbergerTraceConfig {
    fn default() -> Self {
        Self {
            core: TraceConfig::off(),
            progress_every: 0,
            print_on_insert: false,
            print_each_iteration: false,
            print_iteration_timings: false,
            print_phase_summary: true,
            print_breakdown: true,
            sample_memory_on_progress: false,
            sample_memory_on_summary: true,
        }
    }
}
