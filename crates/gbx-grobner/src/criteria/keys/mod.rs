//! Pair queue key strategies.
//!
//! This module contains implementations of [`PairKey`](crate::criteria::PairKey),
//! which assign queue priorities to Buchberger pairs `(i, j)`.
//!
//! Different keys affect processing order and therefore performance, but they
//! should not affect correctness as long as the surrounding pair-update logic
//! is sound.

mod lcm_degree_key;
mod zero_pair_key;

pub use lcm_degree_key::*;
pub use zero_pair_key::*;
