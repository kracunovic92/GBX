use std::time::Duration;

use crate::algos::f4::pairs::critical_pair::CriticalPair;
use crate::algos::f4::trace::SharedF4Tracer;

#[derive(Clone)]
pub struct F4TraceCtx {
    tracer: Option<SharedF4Tracer>,
}

impl F4TraceCtx {
    #[must_use]
    pub fn new(tracer: Option<SharedF4Tracer>) -> Self {
        Self { tracer }
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.tracer.is_some()
    }

    pub fn on_init_complete(&self, initial_basis_len: usize, initial_pending_len: usize) {
        if let Some(tr) = &self.tracer {
            let mut tr = tr.lock();
            tr.set_queue_len(initial_pending_len);
            tr.on_init_complete(initial_basis_len);
            tr.set_gb_len(initial_basis_len);
        }
    }

    pub fn on_iteration_start(&self, iteration: usize, pending_len: usize, basis_len: usize) {
        if let Some(tr) = &self.tracer {
            let mut tr = tr.lock();
            tr.set_gb_len(basis_len);
            tr.set_queue_len(pending_len);
            tr.on_iteration_start(iteration);
        }
    }

    pub fn on_pairs_selected<M>(&self, selected: &[CriticalPair<M>], remaining_len: usize) {
        if let Some(tr) = &self.tracer {
            let mut tr = tr.lock();
            tr.on_pairs_selected(selected.len());
            tr.set_queue_len(remaining_len);
        }
    }

    pub fn on_symbolic_complete(&self, row_count: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_symbolic_complete(row_count);
        }
    }

    pub fn on_reduction_complete(&self, row_count: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_reduction_complete(row_count);
        }
    }

    pub fn on_rows_extracted(&self, extracted: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_rows_extracted(extracted);
        }
    }

    pub fn on_inserted(&self, basis_len: usize, pending_len: usize) {
        if let Some(tr) = &self.tracer {
            let mut tr = tr.lock();
            tr.on_inserted(1);
            tr.set_gb_len(basis_len);
            tr.set_queue_len(pending_len);
        }
    }

    pub fn on_zero_extracted(&self) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_skipped_zero_extracted(1);
        }
    }

    pub fn add_selection_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_selection_time(dt);
        }
    }

    pub fn add_build_ld_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_build_ld_time(dt);
        }
    }

    pub fn add_symbolic_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_symbolic_time(dt);
        }
    }

    pub fn add_reduction_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_reduction_time(dt);
        }
    }

    pub fn add_extraction_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_extraction_time(dt);
        }
    }

    pub fn add_normalize_extracted_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_normalize_extracted_time(dt);
        }
    }

    pub fn add_insert_update_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_insert_update_time(dt);
        }
    }

    pub fn add_post_process_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_post_process_time(dt);
        }
    }

    pub fn add_main_loop_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_main_loop_time(dt);
        }
    }

    pub fn set_total_run_time(&self, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().set_total_run_time(dt);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn on_iteration_timing(
        &self,
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
        if let Some(tr) = &self.tracer {
            tr.lock().on_iteration_timing(
                iteration,
                total,
                selection,
                build_ld,
                symbolic,
                reduction,
                extraction,
                normalize_extracted,
                insert_update,
            );
        }
    }

    pub fn on_loop_complete(&self, basis_len: usize, pending_len: usize) {
        if let Some(tr) = &self.tracer {
            let mut tr = tr.lock();
            tr.set_gb_len(basis_len);
            tr.set_queue_len(pending_len);
            tr.on_finish();
        }
    }
}
