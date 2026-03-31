use crate::trace::MechanicalTraceCounters;

/// Counters for F4 tracing.
///
/// These counters combine reusable mechanical queue/pairing/basis events with
/// F4-specific batch/matrix/extraction metrics.
#[derive(Debug, Default, Clone, Copy)]
pub struct F4TraceCounters {
    /// Reusable queue/pairing/basis mechanics.
    pub mech: MechanicalTraceCounters,

    /// Number of selected batches.
    pub selected_batches: u64,

    /// Total number of selected pairs across all batches.
    pub selected_pairs: u64,

    /// Total number of seeded rows across all batches.
    pub seed_rows_total: u64,

    /// Total number of reducer rows found during symbolic preprocessing.
    pub symbolic_reducer_rows_total: u64,

    /// Total number of rows produced by symbolic preprocessing.
    pub symbolic_rows_total: u64,

    /// Total matrix row count accumulated over all batches.
    pub matrix_rows_total: u64,

    /// Total matrix column count accumulated over all batches.
    pub matrix_cols_total: u64,

    /// Total number of extracted rows/polynomials.
    pub extracted_total: u64,

    /// Number of inserted nonzero extracted polynomials.
    pub inserted_total: u64,

    /// Number of extracted rows skipped because they reduced to zero.
    pub skipped_zero_extracted_total: u64,

    /// Number of emitted progress samples.
    pub progress_samples: u64,
}

impl F4TraceCounters {
    #[inline]
    pub fn set_initial_basis_len(&mut self, n: usize) {
        self.mech.set_initial_basis_len(n);
    }

    #[inline]
    pub fn set_gb_len(&mut self, n: usize) {
        self.mech.set_gb_len(n);
    }

    #[inline]
    pub fn record_push(&mut self, queue_len: usize) {
        self.mech.record_push(queue_len);
    }

    #[inline]
    pub fn record_pop(&mut self, queue_len: usize) {
        self.mech.record_pop(queue_len);
    }

    #[inline]
    pub fn record_seeded_pair(&mut self, queue_len: usize) {
        self.mech.record_seeded_pair(queue_len);
    }

    #[inline]
    pub fn record_pair_rejected_by_criterion(&mut self) {
        self.mech.record_pair_rejected_by_criterion();
    }

    #[inline]
    pub fn record_pair_rejected_by_filter(&mut self) {
        self.mech.record_pair_rejected_by_filter();
    }

    #[inline]
    pub fn record_pair_key_missing(&mut self) {
        self.mech.record_pair_key_missing();
    }

    #[inline]
    pub fn record_pairs_added_by_update(&mut self, n: usize) {
        self.mech.record_pairs_added_by_update(n);
    }

    #[inline]
    pub fn record_batch_selected(&mut self, batch_len: usize) {
        self.selected_batches += 1;
        self.selected_pairs += batch_len as u64;
    }

    #[inline]
    pub fn record_seed_rows(&mut self, count: usize) {
        self.seed_rows_total += count as u64;
    }

    #[inline]
    pub fn record_symbolic(&mut self, reducer_rows: usize, total_rows: usize) {
        self.symbolic_reducer_rows_total += reducer_rows as u64;
        self.symbolic_rows_total += total_rows as u64;
    }

    #[inline]
    pub fn record_matrix_shape(&mut self, rows: usize, cols: usize) {
        self.matrix_rows_total += rows as u64;
        self.matrix_cols_total += cols as u64;
    }

    #[inline]
    pub fn record_extracted(&mut self, count: usize) {
        self.extracted_total += count as u64;
    }

    #[inline]
    pub fn record_inserted_poly(&mut self, gb_len: usize) {
        self.inserted_total += 1;
        self.mech.set_gb_len(gb_len);
    }

    #[inline]
    pub fn record_skipped_zero_extracted(&mut self) {
        self.skipped_zero_extracted_total += 1;
    }

    #[inline]
    pub fn record_progress_sample(&mut self) {
        self.progress_samples += 1;
    }
}
