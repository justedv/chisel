//! Associated Token Account Program CPI helpers.
//!
//! Wrappers for creating associated token accounts with zero dependencies.

use crate::invoke::{invoke, Instruction, AccountMeta, AccountInfo};
use crate::program_ids::{ASSOCIATED_TOKEN_PROGRAM_ID, SYSTEM_PROGRAM_ID, TOKEN_PROGRAM_ID};
use crate::AnvilResult;

/// Create an associated token account.
///
/// # Arguments
/// * `payer` - Account paying for the ATA creation (signer, writable)
/// * `wallet` - The wallet that will own the ATA
/// * `mint` - The token mint
pub fn create(
    payer: &AccountInfo,
    wallet: &AccountInfo,
    mint: &AccountInfo,
) -> AnvilResult {
    let accounts = [
        AccountMeta { pubkey: payer as *const _ as *const u8, is_signer: true, is_writable: true },
        AccountMeta { pubkey: core::ptr::null(), is_signer: false, is_writable: true }, // ATA (derived)
        AccountMeta { pubkey: wallet as *const _ as *const u8, is_signer: false, is_writable: false },
        AccountMeta { pubkey: mint as *const _ as *const u8, is_signer: false, is_writable: false },
        AccountMeta { pubkey: SYSTEM_PROGRAM_ID.as_ptr(), is_signer: false, is_writable: false },
        AccountMeta { pubkey: TOKEN_PROGRAM_ID.as_ptr(), is_signer: false, is_writable: false },
    ];

    let ix = Instruction {
        program_id: ASSOCIATED_TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 6,
        data: core::ptr::null(),
        data_len: 0,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Create an associated token account (idempotent — no error if it already exists).
///
/// This is the safer variant for most use cases.
pub fn create_idempotent(
    payer: &AccountInfo,
    wallet: &AccountInfo,
    mint: &AccountInfo,
) -> AnvilResult {
    let data = [1u8]; // CreateIdempotent

    let accounts = [
        AccountMeta { pubkey: payer as *const _ as *const u8, is_signer: true, is_writable: true },
        AccountMeta { pubkey: core::ptr::null(), is_signer: false, is_writable: true },
        AccountMeta { pubkey: wallet as *const _ as *const u8, is_signer: false, is_writable: false },
        AccountMeta { pubkey: mint as *const _ as *const u8, is_signer: false, is_writable: false },
        AccountMeta { pubkey: SYSTEM_PROGRAM_ID.as_ptr(), is_signer: false, is_writable: false },
        AccountMeta { pubkey: TOKEN_PROGRAM_ID.as_ptr(), is_signer: false, is_writable: false },
    ];

    let ix = Instruction {
        program_id: ASSOCIATED_TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 6,
        data: data.as_ptr(),
        data_len: 1,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}
