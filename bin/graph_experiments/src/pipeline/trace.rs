use gbx_grobner::{F4TraceConfig, F4Tracer, SharedF4Tracer, TraceConfig, TraceLevel, TraceReportMode};

pub fn make_f4_tracer() -> SharedF4Tracer {
    F4Tracer::shared(F4TraceConfig {
        core: TraceConfig { level: TraceLevel::Verbose, report_mode: TraceReportMode::Verbose, snapshot_every: 100, memory_every: 0, final_memory: true },
        progress_every: 50,
        print_on_insert: false,
        print_each_batch: true,
        print_batch_timings: true,
        print_phase_summary: true,
        print_breakdown: true,
        sample_memory_on_progress: false,
        sample_memory_on_summary: true,
    })
}
