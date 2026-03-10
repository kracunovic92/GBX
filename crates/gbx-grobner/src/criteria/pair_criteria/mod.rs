//! Pair-level Buchberger criteria.
//!
//! These criteria implement [`PairCriterion`](crate::criteria::PairCriterion)
//! and decide whether a candidate pair `(i, j)` should remain in the active
//! pair set.

mod and_criterion;
mod baseline_criterion;
mod product_criterion;

pub use and_criterion::*;
pub use baseline_criterion::*;
pub use product_criterion::*;
