//! Error types for Solder operations.

use core::fmt;

/// Result type alias for Solder operations.
pub type SolderResult<T> = Result<T, SolderError>;

/// Errors that can occur during account (de)serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolderError {
    /// Account data is smaller than expected struct size.
    AccountDataTooSmall,
    /// Discriminator bytes don't match expected value.
    InvalidDiscriminator,
    /// Data pointer is not properly aligned for the target type.
    AlignmentError,
    /// Buffer too small to hold serialized output.
    BufferTooSmall,
    /// UTF-8 validation failed for string field.
    InvalidUtf8,
    /// Variable-length field exceeds maximum allowed size.
    FieldTooLarge,
}

impl fmt::Display for SolderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountDataTooSmall => write!(f, "account data too small"),
            Self::InvalidDiscriminator => write!(f, "invalid discriminator"),
            Self::AlignmentError => write!(f, "data alignment error"),
            Self::BufferTooSmall => write!(f, "output buffer too small"),
            Self::InvalidUtf8 => write!(f, "invalid UTF-8 in string field"),
            Self::FieldTooLarge => write!(f, "variable-length field exceeds max size"),
        }
    }
}

#[cfg(not(feature = "no-std"))]
impl std::error::Error for SolderError {}
