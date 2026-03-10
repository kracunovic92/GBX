//! The ring context.
//!
//! `RingCtx` binds together:
//! - a field arithmetic provider (`FieldCtx`)
//! - a monomial order value `O`
//! - number of variables `nvars`
//! - a unique `RingId` used to reject operations across different rings
//!
//! The order type `O` is stored but not constrained here; consumers (monomial comparisons,
//! polynomial sorting) can bound `O` with whatever order trait you use.

use crate::ring::error::{Result, RingError};
use crate::ring::field::FieldCtx;
use core::sync::atomic::{AtomicU64, Ordering};

static NEXT_RING_ID: AtomicU64 = AtomicU64::new(1);

/// Unique identity for a ring context.
///
/// Intended to be stored inside polynomials as a tiny tag so you can detect
/// mixed-ring usage early and provide meaningful errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RingId(u64);

impl RingId {
    /// Simple getter :)
    #[inline]
    pub fn get(self) -> u64 {
        self.0
    }
}

/// Ring context for polynomial/Groebner operations.
#[derive(Clone, Debug)]
pub struct RingCtx<F, O>
where
    F: FieldCtx,
{
    /// Field arithmetic provider (dynamic field context or static adapter).
    pub field: F,

    /// Term order (e.g. Lex, Grevlex).
    pub order: O,

    /// Number of variables in the polynomial ring.
    ///
    /// For dynamic monomials, this is the ring invariant arity.
    pub nvars: usize,

    id: RingId,
}

impl<F, O> RingCtx<F, O>
where
    F: FieldCtx,
{
    /// Construct a ring context.
    ///
    /// # Errors
    /// Returns `RingError::InvalidNvars` if `nvars == 0`.
    pub fn new(field: F, order: O, nvars: usize) -> Result<Self> {
        if nvars == 0 {
            return Err(RingError::InvalidNvars { nvars });
        }

        let id = RingId(NEXT_RING_ID.fetch_add(1, Ordering::Relaxed));
        Ok(Self { field, order, nvars, id })
    }

    /// Ring identity tag.
    #[inline]
    pub fn id(&self) -> RingId {
        self.id
    }

    /// Ensure `poly_ring_id` matches this ring's id.
    ///
    /// Use this in polynomial ops to prevent mixing rings.
    #[inline]
    pub fn assert_same_ring_id(&self, poly_ring_id: RingId) -> Result<()> {
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
    use crate::ring::field::StaticFpCtx;

    #[test]
    fn ring_id_changes_between_instances() {
        let a = RingCtx::new(StaticFpCtx::<7>::new(), Lex, 3).unwrap();
        let b = RingCtx::new(StaticFpCtx::<7>::new(), Lex, 3).unwrap();
        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn invalid_nvars_is_error() {
        let err = RingCtx::new(StaticFpCtx::<7>::new(), Lex, 0).unwrap_err();
        assert!(matches!(err, RingError::InvalidNvars { .. }));
    }

    #[test]
    fn assert_same_ring_id_detects_mismatch() {
        let a = RingCtx::new(StaticFpCtx::<7>::new(), Lex, 3).unwrap();
        let b = RingCtx::new(StaticFpCtx::<7>::new(), Lex, 3).unwrap();
        let err = a.assert_same_ring_id(b.id()).unwrap_err();
        assert!(matches!(err, RingError::MismatchedRing { .. }));
    }
}
