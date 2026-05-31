//! Configuration for the F4 algorithm.

use crate::algos::f4::error::{F4Error, Result};
use crate::algos::post::BasisPostOptionsKind;

/// Configuration options for the F4 algorithm.
#[derive(Debug, Clone, Copy)]
pub struct F4Options {
    /// Maximum number of critical pairs processed in one F4 iteration.
    pub batch_size: usize,

    /// Whether input generators are normalized before the main loop.
    pub normalize_inputs: bool,

    /// Whether newly extracted rows are normalized before basis insertion.
    pub normalize_extracted: bool,

    /// Whether extracted rows are checked with an additional classical reduction pass.
    pub safety_reduce_extracted: bool,

    /// Final basis post-processing mode.
    pub post: BasisPostOptionsKind,

    /// Matrix-reduction backend used by the F4 linear phase.
    pub reducer_kind: F4ReducerKind,
}

impl F4Options {
    /// Validates this options value.
    ///
    /// # Errors
    ///
    /// Returns [`F4Error::InvalidBatchSize`] when `batch_size` is zero.
    pub const fn validate(&self) -> Result<()> {
        if self.batch_size == 0 {
            return Err(F4Error::InvalidBatchSize);
        }

        Ok(())
    }

    /// Validates and wraps this options value.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::validate`].
    pub const fn validated(self) -> Result<ValidatedF4Options> {
        match self.validate() {
            Ok(()) => Ok(ValidatedF4Options(self)),
            Err(err) => Err(err),
        }
    }
}

impl Default for F4Options {
    fn default() -> Self {
        Self { batch_size: 64, normalize_inputs: true, normalize_extracted: true, safety_reduce_extracted: true, post: BasisPostOptionsKind::Reduced, reducer_kind: F4ReducerKind::Roman }
    }
}

/// Matrix-reduction backend used by F4.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum F4ReducerKind {
    /// Dense matrix reducer.
    Dense,

    /// Sequential Roman/Pearce-style sparse-buffer reducer.
    #[default]
    Roman,

    /// Parallel Roman/Pearce-style sparse-buffer reducer.
    RomanParallel,
}

/// Validated F4 options.
///
/// Values of this type have passed [`F4Options::validate`].
#[derive(Debug, Clone, Copy)]
pub struct ValidatedF4Options(F4Options);

impl ValidatedF4Options {
    /// Consumes the wrapper and returns the inner options.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> F4Options {
        self.0
    }

    /// Returns the configured reducer backend.
    #[inline]
    #[must_use]
    pub const fn reducer_kind(&self) -> F4ReducerKind {
        self.0.reducer_kind
    }
}

impl core::ops::Deref for ValidatedF4Options {
    type Target = F4Options;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<F4Options> for ValidatedF4Options {
    #[inline]
    fn as_ref(&self) -> &F4Options {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn default_options_are_valid() {
        let opts = F4Options::default();

        assert!(opts.validate().is_ok());
    }

    #[test]
    fn default_reducer_is_roman() {
        let opts = F4Options::default();

        assert_eq!(opts.reducer_kind, F4ReducerKind::Roman);
    }

    #[test]
    fn zero_batch_size_is_invalid() {
        let opts = F4Options { batch_size: 0, ..F4Options::default() };

        assert!(matches!(opts.validate(), Err(F4Error::InvalidBatchSize)));
    }

    #[test]
    fn validated_wraps_valid_options() {
        let opts = F4Options { batch_size: 32, reducer_kind: F4ReducerKind::Dense, ..F4Options::default() };

        let validated = opts.validated().unwrap();

        assert_eq!(validated.batch_size, 32);
        assert_eq!(validated.reducer_kind(), F4ReducerKind::Dense);
        assert_eq!(validated.as_ref().reducer_kind, F4ReducerKind::Dense);
        assert_eq!(validated.into_inner().batch_size, opts.batch_size);
    }
}
