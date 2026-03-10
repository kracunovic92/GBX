use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum FieldError {
    #[error("invalid modulus p={p}")]
    InvalidModulus { p: u32 },
    #[error("modulus is not prime: p={p}")]
    ModulusNotPrime { p: u32 },
    #[error("Element is not Invertible: p={p}")]
    NonInvertible { value: u32, p: u32 },
}

pub type Result<T> = core::result::Result<T, FieldError>;
