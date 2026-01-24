//! Trait for exponent vector containers.
//!
//! This is intentionally tiny: it only exposes length and a slice view.
//! More advanced behavior (normalization, arithmetic, etc.) belongs in `gbx-poly`.

/// Exponent interface
pub trait Exps: Clone + Eq {
    /// Exponent word type (typically `u32`).
    type Word: Copy + Eq;

    /// Number of variables (length of the exponent vector).
    fn len(&self) -> usize;

    /// Borrow the exponent vector as a slice.
    fn as_slice(&self) -> &[Self::Word];
}
