//! Tests for zero-copy account access.

#[cfg(test)]
mod tests {
    use solder_core::{SolderZeroCopy, SolderError};

    #[repr(C)]
    struct TestAccount {
        value_a: u64,
        value_b: u64,
    }

    impl SolderZeroCopy for TestAccount {
        const DISCRIMINATOR_LEN: usize = 0;
        const DISCRIMINATOR: &'static [u8] = &[];
    }

    #[test]
    fn test_load_valid_data() {
        let mut data = vec![0u8; 16];
        data[0..8].copy_from_slice(&42u64.to_le_bytes());
        data[8..16].copy_from_slice(&100u64.to_le_bytes());

        let account = TestAccount::load(&data).unwrap();
        assert_eq!(account.value_a, 42);
        assert_eq!(account.value_b, 100);
    }

    #[test]
    fn test_load_too_small() {
        let data = vec![0u8; 8]; // need 16
        let result = TestAccount::load(&data);
        assert_eq!(result.unwrap_err(), SolderError::AccountDataTooSmall);
    }

    #[test]
    fn test_load_mut() {
        let mut data = vec![0u8; 16];
        {
            let account = TestAccount::load_mut(&mut data).unwrap();
            account.value_a = 999;
        }
        let val = u64::from_le_bytes(data[0..8].try_into().unwrap());
        assert_eq!(val, 999);
    }
}
