#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Polynomial infrastructure over abstract fields.
//!
//! This crate is intended to work with any `Field` from `algebra_core`.
//! It does **not** depend on concrete field implementations like `Zp<P>`;
//! those live in `algebra_field`.

/// Monomials and monomial orders (fixed and dynamic arity).
pub mod monomial;

/// Polynomial and term types built over a `Field`.
pub mod polynomial;
pub mod term;
