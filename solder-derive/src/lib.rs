//! Derive macros for the Solder framework.
//!
//! This crate provides the `#[derive(Solder)]` macro that generates
//! serialization, deserialization, and zero-copy access implementations.

extern crate proc_macro;

mod parse;
mod expand;
mod layout;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for Solder account structs.
///
/// Generates implementations of:
/// - `SolderDeserialize`
/// - `SolderSerialize`
/// - `SolderZeroCopy` (for fixed-size structs)
/// - `Discriminator`
/// - `AccountSize`
///
/// # Attributes
///
/// - `#[solder(discriminator = N)]` — discriminator size: 0, 4, or 8 (default: 8)
/// - `#[solder(discriminator_value = [bytes])]` — custom discriminator bytes
/// - `#[solder(skip)]` — exclude field from serialization
/// - `#[solder(padding = N)]` — mark field as reserved padding
/// - `#[solder(len_prefix = "u16"|"u32")]` — variable-length field prefix
#[proc_macro_derive(Solder, attributes(solder))]
pub fn derive_solder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::expand_solder(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
