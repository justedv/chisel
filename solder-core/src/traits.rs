//! Serialization and deserialization traits.

use crate::error::SolderResult;

/// Trait for types that can be deserialized from raw account data.
pub trait SolderDeserialize: Sized {
    /// Deserialize from a byte slice, validating the discriminator.
    fn try_deserialize(data: &[u8]) -> SolderResult<Self>;

    /// Deserialize without discriminator validation.
    fn try_deserialize_unchecked(data: &[u8]) -> SolderResult<Self>;
}

/// Trait for types that can be serialized to raw account data.
pub trait SolderSerialize {
    /// Serialize into a byte slice, prepending the discriminator.
    fn try_serialize(&self, buf: &mut [u8]) -> SolderResult<usize>;

    /// Serialized size including discriminator.
    fn serialized_size(&self) -> usize;
}
