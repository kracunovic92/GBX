/// Tracing config.
#[derive(Debug, Clone, Copy)]
pub struct TraceCfg {
    /// Print progress every N pops (0 disables progress printing).
    pub progress_every: usize,

    /// Print when a new polynomial is added.
    pub on_new_poly: bool,

    /// Print phase timings at the end (init/seed/while/post).
    pub phases: bool,

    /// Print breakdown inside while loop (s-poly / normal-form / remainder normalize).
    pub breakdown: bool,
}

impl Default for TraceCfg {
    fn default() -> Self {
        Self { progress_every: 0, on_new_poly: false, phases: true, breakdown: true }
    }
}
