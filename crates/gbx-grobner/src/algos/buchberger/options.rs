use crate::algos::post::BasisPostOptionsKind;

/// Configuration for Buchberger's algorithm.
///
/// These options control normalization of inputs and newly discovered basis
/// elements, as well as final post-processing of the resulting basis.
///
/// # Defaults
///
/// The default configuration:
///
/// - normalizes inputs,
/// - normalizes nonzero remainders before insertion,
/// - returns a reduced Gröbner basis.
#[derive(Debug, Clone, Copy)]
pub struct BuchbergerOptions {
    /// If `true`, normalize each input polynomial before the algorithm starts.
    ///
    /// Zero polynomials are discarded after normalization.
    pub normalize_inputs: bool,

    /// If `true`, normalize each nonzero remainder before inserting it into the
    /// working basis.
    pub normalize_remainders: bool,

    /// Final post-processing applied after Buchberger's main loop.
    pub post: BasisPostOptionsKind,
}

impl Default for BuchbergerOptions {
    #[inline]
    fn default() -> Self {
        Self { normalize_inputs: true, normalize_remainders: true, post: BasisPostOptionsKind::Reduced }
    }
}
