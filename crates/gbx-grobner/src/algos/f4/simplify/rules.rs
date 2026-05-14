use std::collections::HashMap;

use gbx_poly::monomial::Monomial;

use super::target::{DivisorRewrite, RewriteTarget};

/// Simplify rewrite rules for one product source.
///
/// These rules represent historical products with the same source `f`.
/// For a current product `t * f`, exact rewrites are checked first.
/// If no exact rewrite exists, divisor rewrites are searched.
#[derive(Debug, Default, Clone)]
pub(super) struct SourceSimplifyRules {
    /// Exact rewrite:
    ///
    /// ```text
    /// t * f -> 1 * p
    /// ```
    pub exact_rewrites: HashMap<Monomial, RewriteTarget>,

    /// Divisor rewrites:
    ///
    /// ```text
    /// t * f -> (t / u) * p
    /// ```
    ///
    /// where `u | t`.
    pub divisor_rewrites: Vec<DivisorRewrite>,
}

impl SourceSimplifyRules {
    pub(super) fn new() -> Self {
        Self { exact_rewrites: HashMap::new(), divisor_rewrites: Vec::new() }
    }

    pub(super) fn insert_exact(&mut self, multiplier: Monomial, target: RewriteTarget) {
        self.exact_rewrites.insert(multiplier, target);
    }
    pub(super) fn insert_divisor_rewrite(&mut self, divisor: Monomial, target: RewriteTarget) {
        self.divisor_rewrites
            .push(DivisorRewrite { divisor, target });
    }
}
