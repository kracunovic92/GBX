use crate::symbolic::ProductSource;
use gbx_poly::monomial::Monomial;

/// Target of one Simplify rewrite.
///
/// If a historical product `u * f` can be reused, the target points to the
/// corresponding reduced historical row `p`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RewriteTarget {
    pub source: ProductSource,
}

/// A historical divisor rewrite.
///
/// For a current product `t * f`, this rule applies when:
///
/// ```text
/// divisor | t
/// ```
///
/// and rewrites:
///
/// ```text
/// t * f  ->  (t / divisor) * target
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DivisorRewrite {
    pub divisor: Monomial,
    pub target: RewriteTarget,
}
