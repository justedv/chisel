//! Tests for memory layout calculation.

#[cfg(test)]
mod tests {
    use core::mem;

    #[repr(C)]
    struct SimpleAccount {
        authority: [u8; 32], // Pubkey
        balance: u64,
        bump: u8,
    }

    #[test]
    fn test_simple_layout_size() {
        // With repr(C): 32 + 8 + 1 + 7 padding = 48
        let size = mem::size_of::<SimpleAccount>();
        assert_eq!(size, 48);
    }

    #[repr(C, packed)]
    struct PackedAccount {
        authority: [u8; 32],
        balance: u64,
        bump: u8,
    }

    #[test]
    fn test_packed_layout_size() {
        let size = mem::size_of::<PackedAccount>();
        assert_eq!(size, 41); // no padding
    }

    #[test]
    fn test_discriminator_sizes() {
        for &size in &[0usize, 4, 8] {
            assert!(size <= 8);
            assert!(size % 4 == 0 || size == 0);
        }
    }
}
