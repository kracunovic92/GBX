//! Ready-to-use Buchberger pair-management presets.
//!
//! This module provides small constructor helpers for common baseline
//! configurations built from:
//!
//! - a pair criterion
//! - a pair key
//! - a pair update strategy
//!
//! These presets are convenience constructors only.
//! They do not introduce new algorithmic behavior beyond composing the
//! underlying building blocks.
//!
//! # Purpose
//!
//! Presets are useful for:
//!
//! - keeping call sites concise
//! - exposing a stable public API for common configurations
//! - preserving familiar entry points while the internal architecture evolves
//!
//! # Examples
//!
//! A plain baseline configuration:
//!
//! - keep all pairs
//! - assign constant key `0`
//! - insert all new pairs naively
//!
//! A standard optimized baseline:
//!
//! - apply Buchberger's product criterion
//! - prioritize by LCM degree
//! - insert surviving new pairs naively

use crate::criteria::pair_criteria::{BaselineCriterion, ProductCriterion};
use crate::criteria::updates::NaivePairUpdate;
use crate::criteria::{LcmDegreeKey, ZeroPairKey};

/// Baseline Buchberger pair update.
///
/// This preset:
///
/// - keeps every candidate pair
/// - assigns constant key `0` to every pair
/// - inserts all pairs `(i, new_index)` naively
///
/// # Notes
///
/// This corresponds to the simplest Buchberger pair-management setup and is
/// useful as a correctness baseline or debugging reference.
#[inline]
#[must_use]
pub fn baseline_update() -> NaivePairUpdate<BaselineCriterion, ZeroPairKey> {
    NaivePairUpdate::new(BaselineCriterion, ZeroPairKey)
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
/// This is a standard "first optimization" over the pure baseline algorithm.
/// It is still structurally simple, but often substantially reduces the number
/// of useless S-pairs compared with [`baseline_update`].
#[inline]
#[must_use]
pub fn product_update() -> NaivePairUpdate<ProductCriterion, LcmDegreeKey> {
    NaivePairUpdate::new(ProductCriterion, LcmDegreeKey)
}
