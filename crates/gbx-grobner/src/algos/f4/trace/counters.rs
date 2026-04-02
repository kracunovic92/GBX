use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct F4TraceCounters {
    pub iterations: usize,
    pub selected_batches: usize,
    pub selected_pairs: usize,

    pub symbolic_rows_total: usize,
    pub reduced_rows_total: usize,
    pub extracted_total: usize,
    pub inserted_total: usize,
    pub skipped_zero_extracted_total: usize,

    pub gb_len: usize,
    pub queue_len: usize,
    pub max_queue_len: usize,

    pub t_selection: Duration,
    pub t_build_ld: Duration,
    pub t_symbolic: Duration,
    pub t_reduction: Duration,
    pub t_extraction: Duration,
    pub t_normalize_extracted: Duration,
    pub t_insert_update: Duration,
    pub t_post_process: Duration,
    pub t_main_loop: Duration,
    pub t_total_run: Duration,
}
