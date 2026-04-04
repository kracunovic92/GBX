//! Critical-pair management for the F4 algorithm.
//!
//! This module is responsible for:
//! - constructing critical pairs from basis elements,
//! - filtering them with Buchberger-style criteria,
//! - storing pending pairs,
//! - selecting the next batch for one F4 iteration,
//! - updating the pending set after basis insertion.

pub mod criterion;
pub mod critical_pair;
pub mod pending;
pub mod selector;
pub mod update;

pub use criterion::{NoCriterion, PairCriterion, ProductCriterion};
pub use critical_pair::{CriticalPair, PairSide};
pub use pending::{add_pairs_with_new_basis_element, build_all_pairs, make_pair, PendingPairs};
pub use selector::{MinDegreeSelector, PairSelector, Selection};
pub use update::update_with_polynomial;
