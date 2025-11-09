//! Code generation for tuple deserialization (strict parsing).

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};

/// Generate `Deserialize` impl for tuple format with strict parsing.
pub fn gen_deserialize_impl(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Tuple does not support tuple structs",
                ))
            }
            Fields::Unit => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Tuple does not support unit structs",
                ))
            }
        },
        _ => return Err(syn::Error::new_spanned(name, "Tuple only supports structs")),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let field_count = fields.len();
    let field_indices = 0..field_count;

    let mut de_generics = generics.clone();
    let de_lifetime = syn::Lifetime::new("'de", proc_macro2::Span::call_site());
    de_generics.params.insert(
        0,
        syn::GenericParam::Lifetime(syn::LifetimeParam::new(de_lifetime.clone())),
    );
    let (de_impl_generics, _, _) = de_generics.split_for_impl();

    Ok(quote! {
        impl #de_impl_generics ::serde::Deserialize<'de> for #name #ty_generics #where_clause {
            fn deserialize<__D>(deserializer: __D) -> ::std::result::Result<Self, __D::Error>
            where
                __D: ::serde::Deserializer<'de>,
            {
                struct TupleVisitor #impl_generics #where_clause {
                    marker: ::std::marker::PhantomData<#name #ty_generics>,
                }

                impl #de_impl_generics ::serde::de::Visitor<'de> for TupleVisitor #ty_generics #where_clause {
                    type Value = #name #ty_generics;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str(concat!("a tuple of ", stringify!(#field_count), " elements"))
                    }

                    fn visit_seq<__A>(self, mut seq: __A) -> ::std::result::Result<Self::Value, __A::Error>
                    where
                        __A: ::serde::de::SeqAccess<'de>,
                    {
                        #(
                            let #field_names = seq.next_element::<#field_types>()?
                                .ok_or_else(|| ::serde::de::Error::invalid_length(#field_indices, &self))?;
                        )*

                        Ok(#name {
                            #(#field_names,)*
                        })
                    }
                }

                deserializer.deserialize_tuple(
                    #field_count,
                    TupleVisitor {
                        marker: ::std::marker::PhantomData,
                    },
                )
            }
        }
    })
}
