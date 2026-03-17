//! Ready-to-use Buchberger pair-management presets.
//!
//! This module provides small constructor helpers for common configurations
//! built from:
//!
//! - a local pair criterion
//! - a pair key
//! - a pair update strategy
//!
//! These presets are convenience constructors only.
//! They do not introduce new algorithmic behavior beyond composing the
//! underlying building blocks.

use crate::pairing::{NoPairCriterion, ProductCriterion};
use crate::{ConstantPairKey, LcmDegreeKey, NaivePairUpdater};

/// Baseline Buchberger pair update.
///
/// This preset:
///
/// - keeps every candidate pair locally
/// - assigns constant key `0` to every pair
/// - inserts all pairs `(i, new_index)` naively
///
/// # Notes
///
/// This corresponds to the simplest Buchberger pair-management setup and is
/// useful as a correctness baseline or debugging reference.
#[inline]
#[must_use]
pub fn baseline_update() -> NaivePairUpdater<NoPairCriterion, ConstantPairKey> {
    NaivePairUpdater::new(NoPairCriterion, ConstantPairKey)
}

/// Buchberger product-criterion baseline.
///
/// This preset:
///
/// - applies Buchberger's product criterion
/// - assigns priority by `deg(lcm(LM_i, LM_j))`
/// - inserts surviving pairs `(i, new_index)` naively
///
/// # Notes
///
/// This is a standard first optimization over the pure baseline algorithm.
/// It is still structurally simple, but often substantially reduces the number
/// of useless S-pairs compared with [`baseline_update`].
#[inline]
#[must_use]
pub fn product_update() -> NaivePairUpdater<ProductCriterion, LcmDegreeKey> {
    NaivePairUpdater::new(ProductCriterion, LcmDegreeKey)
}
