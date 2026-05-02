use crate::algos::f4::error::{F4Error, Result};
use crate::algos::post::BasisPostOptionsKind;

#[derive(Debug, Clone)]
pub struct F4Options {
    /// Maximum number of critical pairs processed in one F4 iteration.
    pub batch_size: usize,

    /// Normalize input generators before starting.
    pub normalize_inputs: bool,

    /// Normalize extracted rows before inserting into the basis.
    pub normalize_extracted: bool,

    /// Re-check extracted polynomials with a classical reduction safety pass.
    pub safety_reduce_extracted: bool,

    /// Final post-processing after the main loop.
    pub post: BasisPostOptionsKind,
}

#[derive(Debug, Clone)]
pub struct ValidatedF4Options(F4Options);

impl Default for F4Options {
    fn default() -> Self {
        Self { batch_size: 64, normalize_inputs: true, normalize_extracted: true, safety_reduce_extracted: true, post: BasisPostOptionsKind::Reduced }
    }
}

impl F4Options {
    pub fn validate(&self) -> Result<()> {
        if self.batch_size == 0 {
            return Err(F4Error::InvalidBatchSize);
        }

        Ok(())
    }

    pub fn validated(self) -> Result<ValidatedF4Options> {
        self.validate()?;
        Ok(ValidatedF4Options(self))
    }
}

impl ValidatedF4Options {
    #[must_use]
    pub fn as_ref(&self) -> &F4Options {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> F4Options {
        self.0
    }
}

impl core::ops::Deref for ValidatedF4Options {
    type Target = F4Options;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
