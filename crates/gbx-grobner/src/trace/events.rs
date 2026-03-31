/// Trace events for queue-like mechanical behavior.
///
/// These events are generic and reusable across algorithms.
pub trait QueueTrace {
    /// Records that a pair was pushed and the queue now has `queue_len` items.
    #[inline]
    fn on_push(&mut self, _queue_len: usize) {}

    /// Records that a pair was popped and the queue now has `queue_len` items.
    #[inline]
    fn on_pop(&mut self, _queue_len: usize) {}
}

/// Trace events for reusable pairing-layer instrumentation.
///
/// These events correspond to pair generation, rejection, and update mechanics
/// shared by algorithms such as Buchberger and F4.
pub trait PairingTrace {
    /// Records that a pair survived seeding and was inserted.
    #[inline]
    fn on_seeded_pair(&mut self, _queue_len: usize) {}

    /// Records rejection by a local pair criterion.
    #[inline]
    fn on_pair_rejected_by_criterion(&mut self) {}

    /// Records rejection by a state-aware pair filter.
    #[inline]
    fn on_pair_rejected_by_filter(&mut self) {}

    /// Records that no queue key could be produced for a pair.
    #[inline]
    fn on_pair_key_missing(&mut self) {}

    /// Records the number of pairs inserted by a pair-update pass.
    #[inline]
    fn on_pairs_added_by_update(&mut self, _n: usize) {}
}

/// Trace events for basis growth and lifecycle.
///
/// These are generic basis-level events that multiple Gröbner algorithms may
/// care about.
pub trait BasisTrace {
    /// Records completion of initial basis setup.
    #[inline]
    fn on_init_complete(&mut self, _initial_basis_len: usize) {}

    /// Updates the currently known basis length.
    #[inline]
    fn set_gb_len(&mut self, _gb_len: usize) {}

    /// Records insertion of a new polynomial into the basis.
    #[inline]
    fn on_inserted_poly(&mut self, _gb_len: usize, _queue_len: usize) {}
}

/// Trace events for reduction outcomes.
///
/// These are useful for algorithms that explicitly compute remainders and care
/// about zero/unit outcomes.
pub trait ReductionTrace {
    /// Records a zero reduction.
    #[inline]
    fn on_zero_reduction(&mut self) {}

    /// Records a unit reduction.
    #[inline]
    fn on_unit_reduction(&mut self) {}
}
