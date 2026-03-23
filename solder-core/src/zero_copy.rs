//! Zero-copy account access.
//!
//! Provides direct memory-mapped access to account data without
//! copying or allocating.

use crate::error::{SolderError, SolderResult};
use core::mem;

/// Trait for zero-copy access to account data.
///
/// Implementors can be loaded directly from a byte slice without
/// any deserialization overhead.
pub trait SolderZeroCopy: Sized {
    /// Discriminator length in bytes (0, 4, or 8).
    const DISCRIMINATOR_LEN: usize;

    /// Expected discriminator bytes.
    const DISCRIMINATOR: &'static [u8];

    /// Load an immutable reference from raw account data.
    fn load(data: &[u8]) -> SolderResult<&Self> {
        if data.len() < Self::DISCRIMINATOR_LEN + mem::size_of::<Self>() {
            return Err(SolderError::AccountDataTooSmall);
        }

        if Self::DISCRIMINATOR_LEN > 0 {
            let disc = &data[..Self::DISCRIMINATOR_LEN];
            if disc != Self::DISCRIMINATOR {
                return Err(SolderError::InvalidDiscriminator);
            }
        }

        let offset = Self::DISCRIMINATOR_LEN;
        let ptr = data[offset..].as_ptr() as *const Self;

        // Alignment check
        if (ptr as usize) % mem::align_of::<Self>() != 0 {
            return Err(SolderError::AlignmentError);
        }

        Ok(unsafe { &*ptr })
    }

    /// Load a mutable reference from raw account data.
    fn load_mut(data: &mut [u8]) -> SolderResult<&mut Self> {
        if data.len() < Self::DISCRIMINATOR_LEN + mem::size_of::<Self>() {
            return Err(SolderError::AccountDataTooSmall);
        }

        if Self::DISCRIMINATOR_LEN > 0 {
            let disc = &data[..Self::DISCRIMINATOR_LEN];
            if disc != Self::DISCRIMINATOR {
                return Err(SolderError::InvalidDiscriminator);
            }
        }

        let offset = Self::DISCRIMINATOR_LEN;
        let ptr = data[offset..].as_mut_ptr() as *mut Self;

        if (ptr as usize) % mem::align_of::<Self>() != 0 {
            return Err(SolderError::AlignmentError);
        }

        Ok(unsafe { &mut *ptr })
    }
}
