use crate::algos::post::BasisPostOptionsKind;

#[derive(Debug, Clone)]
pub struct F4Options {
    /// Maximum number of critical pairs processed in one F4 iteration.
    pub batch_size: usize,

    /// Normalize input generators before starting.
    pub normalize_inputs: bool,

    /// Normalize extracted rows before inserting into the basis.
    pub normalize_extracted: bool,

    /// For the first prototype, optionally re-check extracted polynomials
    /// with a classical reduction safety pass before insertion.
    pub safety_reduce_extracted: bool,

    /// Final post-processing applied after Buchberger's main loop.
    pub post: BasisPostOptionsKind,
}

impl Default for F4Options {
    fn default() -> Self {
        Self { batch_size: 8, normalize_inputs: true, normalize_extracted: true, safety_reduce_extracted: true, post: BasisPostOptionsKind::Reduced }
    }
}
