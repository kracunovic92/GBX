//! Ring context.
//!
//! A [`RingCtx`] stores the data needed to interpret and manipulate
//! polynomials:
//!
//! - coefficient field arithmetic,
//! - monomial order,
//! - number of variables,
//! - unique ring identity.
//!
//! Polynomial operations should take `&RingCtx` explicitly instead of storing
//! this information inside every polynomial.

use crate::ring::error::{RingError, RingResult};
use crate::ring::field::FieldCtx;
use core::sync::atomic::{AtomicU64, Ordering};

static NEXT_RING_ID: AtomicU64 = AtomicU64::new(1);

/// Unique identity of a ring context.
///
/// This is used to detect accidental operations between polynomials created
/// under different ring contexts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RingId(u64);

impl RingId {
    /// Id
    #[inline]
    pub fn get(self) -> u64 {
        self.0
    }
}

/// Context for polynomial and Gröbner basis operations.
#[derive(Clone, Debug)]
pub struct RingCtx<F, O>
where
    F: FieldCtx,
{
    /// Coefficient field arithmetic.
    pub field: F,

    /// Monomial order.
    pub order: O,

    /// Number of variables.
    pub nvars: usize,

    id: RingId,
}

impl<F, O> RingCtx<F, O>
where
    F: FieldCtx,
{
    /// Creates a new ring context.
    pub fn new(field: F, order: O, nvars: usize) -> RingResult<Self> {
        if nvars == 0 {
            return Err(RingError::InvalidNvars { nvars });
        }

        let id = RingId(NEXT_RING_ID.fetch_add(1, Ordering::Relaxed));

        Ok(Self { field, order, nvars, id })
    }

    /// Returns this ring context's unique identity.
    #[inline]
    pub fn id(&self) -> RingId {
        self.id
    }

    /// Checks that a polynomial belongs to this ring context.
    #[inline]
    pub fn assert_same_ring_id(&self, poly_ring_id: RingId) -> RingResult<()> {
        if poly_ring_id != self.id {
            return Err(RingError::MismatchedRing { expected: self.id.get(), got: poly_ring_id.get() });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::order::Lex;
    use gbx_field::fp::Fp;

    #[test]
    fn ring_id_changes_between_instances() {
        let a = RingCtx::new(Fp::prime(7).unwrap(), Lex, 3).unwrap();
        let b = RingCtx::new(Fp::prime(7).unwrap(), Lex, 3).unwrap();

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn invalid_nvars_is_error() {
        let err = RingCtx::new(Fp::prime(7).unwrap(), Lex, 0).unwrap_err();

        assert!(matches!(err, RingError::InvalidNvars { .. }));
    }

    #[test]
    fn assert_same_ring_id_detects_mismatch() {
        let a = RingCtx::new(Fp::prime(7).unwrap(), Lex, 3).unwrap();
        let b = RingCtx::new(Fp::prime(7).unwrap(), Lex, 3).unwrap();

        let err = a.assert_same_ring_id(b.id()).unwrap_err();

        assert!(matches!(err, RingError::MismatchedRing { .. }));
    }
}
