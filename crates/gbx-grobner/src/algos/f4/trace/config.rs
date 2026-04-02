use crate::trace::TraceConfig;

#[derive(Debug, Clone)]
pub struct F4TraceConfig {
    pub core: TraceConfig,

    pub progress_every: usize,
    pub print_on_insert: bool,
    pub print_each_batch: bool,
    pub print_batch_timings: bool,
    pub print_phase_summary: bool,
    pub print_breakdown: bool,
    pub sample_memory_on_progress: bool,
    pub sample_memory_on_summary: bool,
}

impl Default for F4TraceConfig {
    fn default() -> Self {
        Self {
            core: TraceConfig::default(),
            progress_every: 50,
            print_on_insert: false,
            print_each_batch: false,
            print_batch_timings: false,
            print_phase_summary: false,
            print_breakdown: false,
            sample_memory_on_progress: false,
            sample_memory_on_summary: false,
        }
    }
}
