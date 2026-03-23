//! # Solder
//!
//! Zero-copy account deserialization for Solana programs.
//!
//! Solder provides a derive macro that generates efficient serialization
//! and deserialization code for Solana account structs, without requiring
//! the Anchor framework.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use solder::Solder;
//!
//! #[derive(Solder)]
//! pub struct MyAccount {
//!     pub authority: Pubkey,
//!     pub balance: u64,
//! }
//! ```

pub use solder_core::{
    SolderDeserialize, SolderSerialize, SolderZeroCopy,
    SolderError, SolderResult,
    Discriminator, AccountSize,
};

pub use solder_derive::Solder;
