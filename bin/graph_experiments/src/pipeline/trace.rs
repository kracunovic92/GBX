use gbx_grobner::buchberger::{SharedTracer, TraceCfg, Tracer};

pub fn make_tracer() -> SharedTracer {
    Tracer::shared(TraceCfg {
        progress_every: 100,
        print_on_insert: false,
        print_each_iteration: true,
        print_iteration_timings: false,
        print_phase_summary: true,
        print_breakdown: true,
        collect_memory: false,
        sample_memory_on_progress: false,
        sample_memory_on_summary: true,
    })
}
