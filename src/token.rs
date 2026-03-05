//! SPL Token Program CPI helpers.
//!
//! Zero-dependency wrappers for common SPL Token instructions.

use crate::invoke::{invoke, invoke_signed, Instruction, AccountMeta, AccountInfo};
use crate::program_ids::TOKEN_PROGRAM_ID;
use crate::AnvilResult;

/// Transfer tokens between accounts.
#[inline(always)]
pub fn transfer(
    source: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
) -> AnvilResult {
    let mut data = [0u8; 9];
    data[0] = 3; // Transfer
    data[1..9].copy_from_slice(&amount.to_le_bytes());

    let accounts = [
        AccountMeta { pubkey: source as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: destination as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 3,
        data: data.as_ptr(),
        data_len: 9,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Transfer tokens with PDA signer.
#[inline(always)]
pub fn transfer_signed(
    source: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> AnvilResult {
    let mut data = [0u8; 9];
    data[0] = 3;
    data[1..9].copy_from_slice(&amount.to_le_bytes());

    let accounts = [
        AccountMeta { pubkey: source as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: destination as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 3,
        data: data.as_ptr(),
        data_len: 9,
    };

    invoke_signed(&ix, &[], signer_seeds).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Transfer tokens with decimal check.
pub fn transfer_checked(
    source: &AccountInfo,
    mint: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
    decimals: u8,
) -> AnvilResult {
    let mut data = [0u8; 10];
    data[0] = 12; // TransferChecked
    data[1..9].copy_from_slice(&amount.to_le_bytes());
    data[9] = decimals;

    let accounts = [
        AccountMeta { pubkey: source as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: mint as *const _ as *const u8, is_signer: false, is_writable: false },
        AccountMeta { pubkey: destination as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 4,
        data: data.as_ptr(),
        data_len: 10,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Mint tokens to an account.
pub fn mint_to(
    mint: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
) -> AnvilResult {
    let mut data = [0u8; 9];
    data[0] = 7; // MintTo
    data[1..9].copy_from_slice(&amount.to_le_bytes());

    let accounts = [
        AccountMeta { pubkey: mint as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: destination as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 3,
        data: data.as_ptr(),
        data_len: 9,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Burn tokens.
pub fn burn(
    account: &AccountInfo,
    mint: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
) -> AnvilResult {
    let mut data = [0u8; 9];
    data[0] = 8; // Burn
    data[1..9].copy_from_slice(&amount.to_le_bytes());

    let accounts = [
        AccountMeta { pubkey: account as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: mint as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 3,
        data: data.as_ptr(),
        data_len: 9,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Close a token account, recovering the rent lamports.
pub fn close_account(
    account: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
) -> AnvilResult {
    let data = [9u8]; // CloseAccount

    let accounts = [
        AccountMeta { pubkey: account as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: destination as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 3,
        data: data.as_ptr(),
        data_len: 1,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Approve a delegate to transfer tokens.
pub fn approve(
    source: &AccountInfo,
    delegate: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
) -> AnvilResult {
    let mut data = [0u8; 9];
    data[0] = 4; // Approve
    data[1..9].copy_from_slice(&amount.to_le_bytes());

    let accounts = [
        AccountMeta { pubkey: source as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: delegate as *const _ as *const u8, is_signer: false, is_writable: false },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 3,
        data: data.as_ptr(),
        data_len: 9,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Revoke a delegate's authority.
pub fn revoke(
    source: &AccountInfo,
    authority: &AccountInfo,
) -> AnvilResult {
    let data = [5u8]; // Revoke

    let accounts = [
        AccountMeta { pubkey: source as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 2,
        data: data.as_ptr(),
        data_len: 1,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Freeze a token account.
pub fn freeze_account(
    account: &AccountInfo,
    mint: &AccountInfo,
    authority: &AccountInfo,
) -> AnvilResult {
    let data = [10u8]; // FreezeAccount

    let accounts = [
        AccountMeta { pubkey: account as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: mint as *const _ as *const u8, is_signer: false, is_writable: false },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 3,
        data: data.as_ptr(),
        data_len: 1,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}

/// Thaw a frozen token account.
pub fn thaw_account(
    account: &AccountInfo,
    mint: &AccountInfo,
    authority: &AccountInfo,
) -> AnvilResult {
    let data = [11u8]; // ThawAccount

    let accounts = [
        AccountMeta { pubkey: account as *const _ as *const u8, is_signer: false, is_writable: true },
        AccountMeta { pubkey: mint as *const _ as *const u8, is_signer: false, is_writable: false },
        AccountMeta { pubkey: authority as *const _ as *const u8, is_signer: true, is_writable: false },
    ];

    let ix = Instruction {
        program_id: TOKEN_PROGRAM_ID.as_ptr(),
        accounts: accounts.as_ptr(),
        accounts_len: 3,
        data: data.as_ptr(),
        data_len: 1,
    };

    invoke(&ix, &[]).map_err(|_| crate::error::AnvilError::InvokeFailed)
}
