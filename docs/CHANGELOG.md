# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1] - 2026-01-16

### Fixed
- Correct instruction index for `system::allocate`
- Associated token `create_idempotent` instruction byte
- Token program ID trailing byte correction
- Handle zero-lamport transfer edge case in `system::transfer`

### Added
- `initialize_mint` instruction to token module
- `initialize_account` instruction to token module
- `set_authority` instruction to token module
- `create_account_with_seed` to system module
- `InsufficientAccounts` error variant
- Rent sysvar ID to `program_ids`
- Token-2022 feature flag

### Changed
- Extract common account meta patterns in invoke module
- Use const for token instruction indices
- Improved doc comments across all modules

## [0.1.0] - 2025-12-28

### Added
- Initial release
- System Program CPI helpers (transfer, create_account, allocate, assign)
- SPL Token CPI helpers (transfer, transfer_checked, mint_to, burn, approve, revoke, close_account, freeze, thaw)
- Associated Token Account helpers (create, create_idempotent)
- Raw invoke/invoke_signed wrappers
- Compile-time program ID constants
- PDA signer variants (`_signed`) for all mutating operations
- Zero-dependency, no_std, zero-copy design
