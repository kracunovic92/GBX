//! Pair-set update strategies.
//!
//! This module contains implementations of [`PairUpdate`](crate::criteria::PairUpdate),
//! which are responsible for updating the active pair set after a new
//! polynomial is appended to the Gröbner basis.
//!
//! # Role in the architecture
//!
//! A pair update strategy decides how the active pair set evolves:
//!
//! - which new pairs `(i, new_index)` are inserted
//! - whether existing pairs should remain untouched
//! - later, whether some existing pairs should be removed or replaced
//!
//! The simplest implementation is [`NaivePairUpdate`], which seeds every
//! candidate pair `(i, new_index)` that survives the configured
//! [`PairCriterion`](crate::criteria::PairCriterion) and for which a
//! [`PairKey`](crate::criteria::PairKey) can be computed.
//!
//! More advanced update strategies, such as Gebauer–Möller style pair-set
//! maintenance, can be added here later.

mod naive_pair_update;

pub use naive_pair_update::*;
