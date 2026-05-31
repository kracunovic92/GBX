//! Critical-pair management for the F4 algorithm.
//!
//! This module contains the data structures and strategies used to create,
//! filter, store, and select critical pairs during F4 iterations.

pub mod criterion;
pub mod critical_pair;
pub mod pending;
pub mod selector;
pub mod update;

pub use criterion::{NoCriterion, PairCriterion, ProductCriterion};
pub use critical_pair::{CriticalPair, PairSide};
pub use pending::{PendingPairs, add_pairs_with_new_basis_element, build_all_pairs, make_pair};
pub use selector::{MinDegreeSelector, PairSelector, Selection};
pub use update::update_with_polynomial;
