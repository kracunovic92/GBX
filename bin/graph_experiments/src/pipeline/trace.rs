use gbx_grobner::buchberger::trace::SharedBuchbergerTracer;
use gbx_grobner::{BuchbergerTraceConfig, BuchbergerTracer, F4TraceConfig, F4Tracer, SharedF4Tracer, TraceConfig, TraceLevel, TraceReportMode};

pub fn make_debug_tracer() -> SharedBuchbergerTracer {
    BuchbergerTracer::shared(BuchbergerTraceConfig {
        core: TraceConfig { level: TraceLevel::Verbose, report_mode: TraceReportMode::Verbose, snapshot_every: 100, memory_every: 0, final_memory: true },
        progress_every: 100,
        print_on_insert: false,
        print_each_iteration: true,
        print_iteration_timings: false,
        print_phase_summary: true,
        print_breakdown: true,
        sample_memory_on_progress: false,
        sample_memory_on_summary: true,
    })
}
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
pub fn make_bench_tracer() -> SharedBuchbergerTracer {
    BuchbergerTracer::shared(BuchbergerTraceConfig {
        core: TraceConfig { level: TraceLevel::Timings, report_mode: TraceReportMode::Summary, snapshot_every: 1000, memory_every: 0, final_memory: true },
        progress_every: 1000,
        print_on_insert: false,
        print_each_iteration: false,
        print_iteration_timings: false,
        print_phase_summary: true,
        print_breakdown: true,
        sample_memory_on_progress: false,
        sample_memory_on_summary: true,
    })
}
