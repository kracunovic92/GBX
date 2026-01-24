//! # gbx-poly
//!
//! Polynomial data structures and algorithms for the GBX ecosystem.

#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod prelude;

pub mod monomial;
mod polynomial;
mod term;
