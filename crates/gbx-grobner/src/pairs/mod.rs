//! Critical pair queues for Buchberger-style Gröbner basis algorithms.
//!
//! A **critical pair** is an unordered pair of indices `(i, j)` into the current
//! basis `G = [g₀, g₁, ...]` used to form an S-polynomial `S(gᵢ, gⱼ)`.
//!
//! Most implementations store pairs with `i < j` to avoid duplicates.
//! This module does not enforce that; pairing/pair-generation should do so.
//!
//! # Pair queue backends
//! - [`StackPairs`]: LIFO (baseline, cache-friendly)
//! - [`FifoPairs`]: FIFO

mod heap_pairs;
mod stack_pair;
mod traits;

pub use heap_pairs::*;
pub use stack_pair::*;
pub use traits::*;
