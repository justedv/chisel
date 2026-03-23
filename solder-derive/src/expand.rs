//! Code generation for the Solder derive macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Result};

use crate::parse::{ContainerAttrs, FieldAttrs};
use crate::layout::compute_layout;

pub fn expand_solder(input: DeriveInput) -> Result<TokenStream> {
    let container_attrs = ContainerAttrs::from_input(&input)?;
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => return Err(syn::Error::new_spanned(
                &input, "Solder only supports structs with named fields"
            )),
        },
        _ => return Err(syn::Error::new_spanned(
            &input, "Solder only supports structs"
        )),
    };

    let layout = compute_layout(fields, &container_attrs)?;
    let disc_len = container_attrs.discriminator_size;
    let disc_bytes = layout.discriminator_bytes();

    let deser_fields = layout.deserialize_tokens();
    let ser_fields = layout.serialize_tokens();
    let size = layout.total_size();

    let expanded = quote! {
        impl ::solder_core::SolderDeserialize for #name {
            fn try_deserialize(data: &[u8]) -> ::solder_core::SolderResult<Self> {
                if data.len() < #disc_len {
                    return Err(::solder_core::SolderError::AccountDataTooSmall);
                }
                let disc = &data[..#disc_len];
                if disc != &[#(#disc_bytes),*] {
                    return Err(::solder_core::SolderError::InvalidDiscriminator);
                }
                Self::try_deserialize_unchecked(&data[#disc_len..])
            }

            fn try_deserialize_unchecked(data: &[u8]) -> ::solder_core::SolderResult<Self> {
                #deser_fields
            }
        }

        impl ::solder_core::SolderSerialize for #name {
            fn try_serialize(&self, buf: &mut [u8]) -> ::solder_core::SolderResult<usize> {
                let needed = self.serialized_size();
                if buf.len() < needed {
                    return Err(::solder_core::SolderError::BufferTooSmall);
                }
                buf[..#disc_len].copy_from_slice(&[#(#disc_bytes),*]);
                let offset = #disc_len;
                #ser_fields
                Ok(needed)
            }

            fn serialized_size(&self) -> usize {
                #size
            }
        }

        impl ::solder_core::Discriminator for #name {
            const DISCRIMINATOR: &'static [u8] = &[#(#disc_bytes),*];
        }

        impl ::solder_core::AccountSize for #name {
            const SPACE: usize = #size;
        }
    };

    Ok(expanded)
}
