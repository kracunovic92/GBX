use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;

use crate::algos::f4::trace::config::F4TraceConfig;
use crate::algos::f4::trace::counters::F4TraceCounters;
use crate::trace::{TraceLevel, TraceReportMode};

pub type SharedF4Tracer = Arc<Mutex<F4Tracer>>;

#[derive(Debug)]
pub struct F4Tracer {
    config: F4TraceConfig,
    counters: F4TraceCounters,
}

impl F4Tracer {
    #[must_use]
    pub fn new(config: F4TraceConfig) -> Self {
        Self { config, counters: F4TraceCounters::default() }
    }

    #[must_use]
    pub fn shared(config: F4TraceConfig) -> SharedF4Tracer {
        Arc::new(Mutex::new(Self::new(config)))
    }

    #[must_use]
    pub fn config(&self) -> &F4TraceConfig {
        &self.config
    }

    #[must_use]
    pub fn counters(&self) -> &F4TraceCounters {
        &self.counters
    }

    pub fn on_init_complete(&mut self, initial_basis_len: usize) {
        self.counters.gb_len = initial_basis_len;
        self.maybe_print_init();
    }

    pub fn set_gb_len(&mut self, n: usize) {
        self.counters.gb_len = n;
    }

    pub fn set_queue_len(&mut self, n: usize) {
        self.counters.queue_len = n;
        self.counters.max_queue_len = self.counters.max_queue_len.max(n);
    }

    pub fn on_iteration_start(&mut self, iteration: usize) {
        self.counters.iterations = iteration;

        if self.should_print_progress(iteration) {
            self.print_progress_line();
        }
    }

    pub fn on_pairs_selected(&mut self, count: usize) {
        if count == 0 {
            return;
        }

        self.counters.selected_batches += 1;
        self.counters.selected_pairs += count;

        if self.config.print_each_batch && self.is_verboseish() {
            println!(
                "[f4][batch] iter={} selected_pairs={} gb_len={} queue_len={}",
                self.counters.iterations, count, self.counters.gb_len, self.counters.queue_len,
            );
        }
    }

    pub fn on_symbolic_complete(&mut self, row_count: usize) {
        self.counters.symbolic_rows_total += row_count;

        if self.config.print_breakdown && self.is_verboseish() {
            println!(
                "[f4][symbolic] iter={} rows={} total_symbolic_rows={}",
                self.counters.iterations, row_count, self.counters.symbolic_rows_total,
            );
        }
    }

    pub fn on_reduction_complete(&mut self, row_count: usize) {
        self.counters.reduced_rows_total += row_count;

        if self.config.print_breakdown && self.is_verboseish() {
            println!(
                "[f4][linear] iter={} reduced_rows={} total_reduced_rows={}",
                self.counters.iterations, row_count, self.counters.reduced_rows_total,
            );
        }
    }

    pub fn on_rows_extracted(&mut self, extracted: usize) {
        self.counters.extracted_total += extracted;

        if self.config.print_phase_summary && self.is_verboseish() {
            println!(
                "[f4][extract] iter={} extracted={} total_extracted={}",
                self.counters.iterations, extracted, self.counters.extracted_total,
            );
        }
    }

    pub fn on_inserted(&mut self, count: usize) {
        self.counters.inserted_total += count;

        if self.config.print_on_insert && self.is_verboseish() {
            println!(
                "[f4][insert] iter={} inserted={} total_inserted={} gb_len={} queue_len={}",
                self.counters.iterations, count, self.counters.inserted_total, self.counters.gb_len, self.counters.queue_len,
            );
        }
    }

    pub fn on_skipped_zero_extracted(&mut self, count: usize) {
        self.counters.skipped_zero_extracted_total += count;
    }

    pub fn add_selection_time(&mut self, dt: Duration) {
        self.counters.t_selection += dt;
    }

    pub fn add_build_ld_time(&mut self, dt: Duration) {
        self.counters.t_build_ld += dt;
    }

    pub fn add_symbolic_time(&mut self, dt: Duration) {
        self.counters.t_symbolic += dt;
    }

    pub fn add_reduction_time(&mut self, dt: Duration) {
        self.counters.t_reduction += dt;
    }

    pub fn add_extraction_time(&mut self, dt: Duration) {
        self.counters.t_extraction += dt;
    }

    pub fn add_normalize_extracted_time(&mut self, dt: Duration) {
        self.counters.t_normalize_extracted += dt;
    }

    pub fn add_insert_update_time(&mut self, dt: Duration) {
        self.counters.t_insert_update += dt;
    }

    pub fn add_post_process_time(&mut self, dt: Duration) {
        self.counters.t_post_process += dt;
    }

    pub fn add_main_loop_time(&mut self, dt: Duration) {
        self.counters.t_main_loop += dt;
    }

    pub fn set_total_run_time(&mut self, dt: Duration) {
        self.counters.t_total_run = dt;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn on_iteration_timing(
        &mut self,
        iteration: usize,
        total: Duration,
        selection: Duration,
        build_ld: Duration,
        symbolic: Duration,
        reduction: Duration,
        extraction: Duration,
        normalize_extracted: Duration,
        insert_update: Duration,
    ) {
        if !(self.config.print_batch_timings && self.is_verboseish()) {
            return;
        }

        println!(
            "[f4][timing] iter={} total={:.3}ms sel={:.3}ms({:.1}%) build={:.3}ms({:.1}%) sym={:.3}ms({:.1}%) red={:.3}ms({:.1}%) ext={:.3}ms({:.1}%) norm={:.3}ms({:.1}%) ins={:.3}ms({:.1}%)",
            iteration,
            ms(total),
            ms(selection),
            pct(selection, total),
            ms(build_ld),
            pct(build_ld, total),
            ms(symbolic),
            pct(symbolic, total),
            ms(reduction),
            pct(reduction, total),
            ms(extraction),
            pct(extraction, total),
            ms(normalize_extracted),
            pct(normalize_extracted, total),
            ms(insert_update),
            pct(insert_update, total),
        );
    }

    pub fn on_finish(&mut self) {
        match self.config.core.report_mode {
            TraceReportMode::Silent => {}
            TraceReportMode::Summary | TraceReportMode::Verbose => {
                self.print_summary();
                if self.config.print_batch_timings || self.config.print_breakdown {
                    self.print_timing_summary();
                }
            }
        }
    }

    fn maybe_print_init(&self) {
        if !self.is_verboseish() {
            return;
        }

        println!(
            "[f4][init] gb_len={} queue_len={}",
            self.counters.gb_len, self.counters.queue_len,
        );
    }

    fn should_print_progress(&self, iteration: usize) -> bool {
        if !self.is_verboseish() {
            return false;
        }

        let every = self.config.progress_every;
        every > 0 && iteration > 0 && iteration % every == 0
    }

    fn is_verboseish(&self) -> bool {
        matches!(
            self.config.core.level,
            TraceLevel::Timings | TraceLevel::Snapshots | TraceLevel::Verbose
        )
    }

    fn print_progress_line(&self) {
        println!(
            "[f4][progress] iter={} batches={} selected_pairs={} gb_len={} queue_len={} max_queue_len={} symbolic_rows={} reduced_rows={} extracted={} inserted={} skipped_zero_extracted={}",
            self.counters.iterations,
            self.counters.selected_batches,
            self.counters.selected_pairs,
            self.counters.gb_len,
            self.counters.queue_len,
            self.counters.max_queue_len,
            self.counters.symbolic_rows_total,
            self.counters.reduced_rows_total,
            self.counters.extracted_total,
            self.counters.inserted_total,
            self.counters.skipped_zero_extracted_total,
        );
    }

    fn print_summary(&self) {
        println!("--- F4 trace summary ---");
        println!("iterations: {}", self.counters.iterations);
        println!("selected batches: {}", self.counters.selected_batches);
        println!("selected pairs: {}", self.counters.selected_pairs);
        println!("final gb_len: {}", self.counters.gb_len);
        println!("final queue_len: {}", self.counters.queue_len);
        println!("max queue_len: {}", self.counters.max_queue_len);
        println!("symbolic rows total: {}", self.counters.symbolic_rows_total);
        println!("reduced rows total: {}", self.counters.reduced_rows_total);
        println!("extracted total: {}", self.counters.extracted_total);
        println!("inserted total: {}", self.counters.inserted_total);
        println!(
            "skipped zero extracted total: {}",
            self.counters.skipped_zero_extracted_total
        );
    }

    fn print_timing_summary(&self) {
        let total = self.counters.t_main_loop;

        println!("--- F4 timing summary ---");
        println!(
            "selection:            {:>10.3} ms ({:>5.1}%)",
            ms(self.counters.t_selection),
            pct(self.counters.t_selection, total),
        );
        println!(
            "build_ld:             {:>10.3} ms ({:>5.1}%)",
            ms(self.counters.t_build_ld),
            pct(self.counters.t_build_ld, total),
        );
        println!(
            "symbolic:             {:>10.3} ms ({:>5.1}%)",
            ms(self.counters.t_symbolic),
            pct(self.counters.t_symbolic, total),
        );
        println!(
            "reduction:            {:>10.3} ms ({:>5.1}%)",
            ms(self.counters.t_reduction),
            pct(self.counters.t_reduction, total),
        );
        println!(
            "extraction:           {:>10.3} ms ({:>5.1}%)",
            ms(self.counters.t_extraction),
            pct(self.counters.t_extraction, total),
        );
        println!(
            "normalize_extracted:  {:>10.3} ms ({:>5.1}%)",
            ms(self.counters.t_normalize_extracted),
            pct(self.counters.t_normalize_extracted, total),
        );
        println!(
            "insert_update:        {:>10.3} ms ({:>5.1}%)",
            ms(self.counters.t_insert_update),
            pct(self.counters.t_insert_update, total),
        );
        println!(
            "post_process:         {:>10.3} ms",
            ms(self.counters.t_post_process),
        );
        println!(
            "main_loop_total:      {:>10.3} ms",
            ms(self.counters.t_main_loop),
        );
        println!(
            "run_total:            {:>10.3} ms",
            ms(self.counters.t_total_run),
        );
    }
}

fn pct(part: Duration, total: Duration) -> f64 {
    if total.is_zero() { 0.0 } else { 100.0 * part.as_secs_f64() / total.as_secs_f64() }
}

fn ms(dt: Duration) -> f64 {
    dt.as_secs_f64() * 1000.0
}
