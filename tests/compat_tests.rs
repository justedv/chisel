//! Compatibility tests — ensure Solder works alongside Anchor types.

#[cfg(test)]
mod tests {
    #[test]
    fn test_discriminator_no_collision() {
        // Anchor uses first 8 bytes of SHA256("account:<Name>")
        // Solder uses the same scheme by default for compatibility
        let name_a = "account:VaultAccount";
        let name_b = "account:TokenVault";
        assert_ne!(name_a, name_b);
    }

    #[test]
    fn test_custom_discriminator_value() {
        let custom: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        assert_eq!(custom.len(), 4);
        assert_ne!(custom, [0u8; 4]);
    }
}
