//! Code generation for tuple serialization.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};

/// Generate `Serialize` impl for tuple format.
pub fn gen_serialize_impl(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Serialize does not support tuple structs",
                ));
            }
        },
        _ => return Err(syn::Error::new_spanned(name, "Only supports structs")),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_count = fields.len();

    Ok(quote! {
        impl #impl_generics ::serde::Serialize for #name #ty_generics #where_clause {
            fn serialize<__S>(&self, serializer: __S) -> ::std::result::Result<__S::Ok, __S::Error>
            where
                __S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeTuple;
                let mut tuple = serializer.serialize_tuple(#field_count)?;
                #(
                    tuple.serialize_element(&self.#field_names)?;
                )*
                tuple.end()
            }
        }
    })
}
