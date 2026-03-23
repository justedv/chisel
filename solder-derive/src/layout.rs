//! Memory layout computation for Solder structs.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field, Result};

use crate::parse::{ContainerAttrs, FieldAttrs};

/// Computed layout information for a struct.
pub struct StructLayout {
    disc_size: usize,
    struct_name: String,
    fields: Vec<FieldLayout>,
}

struct FieldLayout {
    name: String,
    fixed_size: Option<usize>,
    is_skip: bool,
}

impl StructLayout {
    /// Compute discriminator bytes (SHA256 of "account:{Name}" truncated).
    pub fn discriminator_bytes(&self) -> Vec<u8> {
        if self.disc_size == 0 {
            return vec![];
        }
        // Simple hash: we use a basic hash of the struct name
        let input = format!("account:{}", self.struct_name);
        let mut hash = [0u8; 32];
        simple_hash(input.as_bytes(), &mut hash);
        hash[..self.disc_size].to_vec()
    }

    pub fn deserialize_tokens(&self) -> TokenStream {
        // placeholder — actual impl generated per field
        quote! { todo!("deserialize") }
    }

    pub fn serialize_tokens(&self) -> TokenStream {
        quote! { todo!("serialize") }
    }

    pub fn total_size(&self) -> TokenStream {
        let disc = self.disc_size;
        quote! { #disc + core::mem::size_of::<Self>() }
    }
}

/// Compute the layout for a set of fields.
pub fn compute_layout(
    fields: &Punctuated<Field, Comma>,
    attrs: &ContainerAttrs,
) -> Result<StructLayout> {
    let mut field_layouts = Vec::new();

    for field in fields {
        let field_attrs = FieldAttrs::from_field(field)?;
        let name = field.ident.as_ref().unwrap().to_string();

        field_layouts.push(FieldLayout {
            name,
            fixed_size: None, // computed at compile time via size_of
            is_skip: field_attrs.skip,
        });
    }

    Ok(StructLayout {
        disc_size: attrs.discriminator_size,
        struct_name: String::new(), // filled by caller
        fields: field_layouts,
    })
}

/// Simple non-cryptographic hash for discriminator generation.
fn simple_hash(data: &[u8], out: &mut [u8; 32]) {
    let mut state: [u64; 4] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
    ];

    for (i, &byte) in data.iter().enumerate() {
        state[i % 4] = state[i % 4].wrapping_mul(31).wrapping_add(byte as u64);
    }

    for (i, s) in state.iter().enumerate() {
        let bytes = s.to_le_bytes();
        out[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
    }
}
