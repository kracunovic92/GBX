/// Configuration for Buchberger tracing.
///
/// This controls both collection and printing behavior.
///
/// The tracing system is intended for debugging and performance diagnosis.
/// Configuration for Buchberger tracing.
///
/// This controls both collection and printing behavior.
///
/// The tracing system is intended for debugging and performance diagnosis.
/// Expensive sampling should remain optional.
#[derive(Debug, Clone, Copy)]
pub struct TraceCfg {
    /// Print progress every `N` successful queue pops.
    ///
    /// A value of `0` disables periodic progress printing.
    pub progress_every: usize,

    /// Print a line whenever a nonzero remainder is inserted into the basis.
    pub print_on_insert: bool,

    /// Print a compact per-iteration line for each processed pair.
    ///
    /// This is primarily intended for debugging and may produce a lot of output.
    pub print_each_iteration: bool,

    /// Print detailed timings for each processed pair.
    ///
    /// Has no effect unless [`Self::print_each_iteration`] is enabled.
    pub print_iteration_timings: bool,

    /// Print phase totals at the end (`init`, `seed`, `while`, `post`).
    pub print_phase_summary: bool,

    /// Print while-loop breakdown at the end (`s_poly`, `normal_form`,
    /// `remainder_normalize`, `pair_update`).
    pub print_breakdown: bool,

    /// Sample process memory usage.
    ///
    /// This is currently Linux-oriented and best-effort.
    pub collect_memory: bool,

    /// Include memory usage in periodic progress lines.
    ///
    /// Has no effect unless [`Self::collect_memory`] is enabled.
    pub sample_memory_on_progress: bool,

    /// Include memory usage in the final summary.
    ///
    /// Has no effect unless [`Self::collect_memory`] is enabled.
    pub sample_memory_on_summary: bool,
}

impl Default for TraceCfg {
    fn default() -> Self {
        Self {
            progress_every: 0,
            print_on_insert: false,
            print_each_iteration: false,
            print_iteration_timings: false,
            print_phase_summary: true,
            print_breakdown: true,
            collect_memory: false,
            sample_memory_on_progress: false,
            sample_memory_on_summary: true,
        }
    }
}
