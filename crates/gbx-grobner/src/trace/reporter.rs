/// Generic reporting mode for tracing output.
///
/// This controls how collected trace data should be emitted by
/// algorithm-specific reporters. It does not define algorithm-specific
/// formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TraceReportMode {
    /// Do not print any tracing output.
    #[default]
    Silent,

    /// Print concise progress and summary information.
    Summary,

    /// Print detailed per-event / per-iteration information where supported.
    Verbose,
}
