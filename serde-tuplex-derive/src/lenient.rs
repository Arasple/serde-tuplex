//! Code generation for lenient deserialization.

use crate::analysis::{is_option_type, should_be_lenient};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};

/// Generate `Deserialize` impl with lenient numeric parsing.
///
/// If `as_tuple` is true, deserializes from tuple format; otherwise from struct format.
pub fn gen_lenient_deserialize(
    input: &DeriveInput,
    as_tuple: bool,
) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Lenient does not support tuple structs",
                ))
            }
            Fields::Unit => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Lenient does not support unit structs",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Lenient only supports structs",
            ))
        }
    };

    let mut de_generics = generics.clone();
    let de_lifetime = syn::Lifetime::new("'de", proc_macro2::Span::call_site());
    de_generics.params.insert(
        0,
        syn::GenericParam::Lifetime(syn::LifetimeParam::new(de_lifetime.clone())),
    );
    let (de_impl_generics, _, _) = de_generics.split_for_impl();

    let deserialize_body = if as_tuple {
        gen_tuple_visitor(fields, name, &impl_generics, &ty_generics, &where_clause)
    } else {
        gen_struct_visitor(fields, name, &impl_generics, &ty_generics, &where_clause)
    };

    Ok(quote! {
        impl #de_impl_generics ::serde::Deserialize<'de> for #name #ty_generics #where_clause {
            fn deserialize<__D>(deserializer: __D) -> ::std::result::Result<Self, __D::Error>
            where
                __D: ::serde::Deserializer<'de>,
            {
                #deserialize_body
            }
        }
    })
}

/// Generate visitor for tuple format with lenient parsing.
fn gen_tuple_visitor(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream {
    let field_count = fields.len();
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let tuple_field_deserializers: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            let field_name = &field.ident;
            let field_ty = &field.ty;

            if should_be_lenient(field) {
                if is_option_type(&field.ty) {
                    quote! {
                        let #field_name = seq.next_element::<::serde_tuplex::__private::OptionalLenientValue>()?
                            .ok_or_else(|| ::serde::de::Error::invalid_length(#idx, &self))?
                            .into_option()
                            .map(|v| v.parse())
                            .transpose()
                            .map_err(|e: String| ::serde::de::Error::custom(e))?;
                    }
                } else {
                    quote! {
                        let #field_name = seq.next_element::<::serde_tuplex::__private::LenientValue>()?
                            .ok_or_else(|| ::serde::de::Error::invalid_length(#idx, &self))?
                            .parse::<#field_ty>()
                            .map_err(|e: String| ::serde::de::Error::custom(e))?;
                    }
                }
            } else {
                quote! {
                    let #field_name = seq.next_element::<#field_ty>()?
                        .ok_or_else(|| ::serde::de::Error::invalid_length(#idx, &self))?;
                }
            }
        })
        .collect();

    quote! {
        struct TupleVisitor #impl_generics #where_clause {
            marker: ::std::marker::PhantomData<#name #ty_generics>,
        }

        impl<'de> ::serde::de::Visitor<'de> for TupleVisitor #ty_generics #where_clause {
            type Value = #name #ty_generics;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str(concat!("a tuple of ", stringify!(#field_count), " elements"))
            }

            fn visit_seq<__A>(self, mut seq: __A) -> ::std::result::Result<Self::Value, __A::Error>
            where
                __A: ::serde::de::SeqAccess<'de>,
            {
                #(#tuple_field_deserializers)*

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

/// Generate visitor for struct format with lenient parsing.
fn gen_struct_visitor(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream {
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_name_strs: Vec<_> = field_names
        .iter()
        .map(|name| name.as_ref().unwrap().to_string())
        .collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let field_is_optional: Vec<_> = fields.iter().map(|f| is_option_type(&f.ty)).collect();

    let field_deserializers: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;

            if should_be_lenient(field) {
                if is_option_type(&field.ty) {
                    quote! {
                        let __lenient_value = map.next_value::<::serde_tuplex::__private::OptionalLenientValue>()?;
                        let parsed: #field_ty = __lenient_value.into_option()
                            .map(|v| v.parse())
                            .transpose()
                            .map_err(|e: String| ::serde::de::Error::custom(format!("failed to parse field {}: {}", stringify!(#field_name), e)))?;
                        #field_name = Some(parsed);
                    }
                } else {
                    quote! {
                        let __lenient_value = map.next_value::<::serde_tuplex::__private::LenientValue>()?;
                        let __parsed_value = __lenient_value.parse::<#field_ty>()
                            .map_err(|e| ::serde::de::Error::custom(format!("failed to parse field {}: {}", stringify!(#field_name), e)))?;
                        #field_name = Some(__parsed_value);
                    }
                }
            } else {
                quote! {
                    #field_name = Some(map.next_value::<#field_ty>()?);
                }
            }
        })
        .collect();

    let field_var_decls: Vec<_> = field_names
        .iter()
        .zip(&field_types)
        .map(|(name, ty)| {
            quote! { let mut #name: ::std::option::Option<#ty> = None }
        })
        .collect();

    let field_unwraps: Vec<_> = field_names
        .iter()
        .zip(&field_is_optional)
        .map(|(name, is_opt)| {
            if *is_opt {
                quote! { #name: #name.unwrap_or(None) }
            } else {
                quote! { #name: #name.ok_or_else(|| ::serde::de::Error::missing_field(stringify!(#name)))? }
            }
        })
        .collect();

    quote! {
        #[allow(non_camel_case_types)]
        enum Field {
            #(#field_names,)*
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Field, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> ::serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str("field identifier")
                    }

                    fn visit_str<E>(self, value: &str) -> ::std::result::Result<Field, E>
                    where
                        E: ::serde::de::Error,
                    {
                        match value {
                            #(#field_name_strs => Ok(Field::#field_names),)*
                            _ => Err(::serde::de::Error::unknown_field(value, &[#(#field_name_strs),*])),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Visitor #impl_generics #where_clause {
            marker: ::std::marker::PhantomData<#name #ty_generics>,
        }

        impl<'de> ::serde::de::Visitor<'de> for Visitor #ty_generics #where_clause {
            type Value = #name #ty_generics;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str(concat!("struct ", stringify!(#name)))
            }

            fn visit_map<__A>(self, mut map: __A) -> ::std::result::Result<Self::Value, __A::Error>
            where
                __A: ::serde::de::MapAccess<'de>,
            {
                #(#field_var_decls;)*

                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        #(
                            Field::#field_names => {
                                if #field_names.is_some() {
                                    return Err(::serde::de::Error::duplicate_field(#field_name_strs));
                                }
                                #field_deserializers
                            }
                        )*
                    }
                }

                Ok(#name {
                    #(#field_unwraps,)*
                })
            }
        }

        deserializer.deserialize_struct(
            stringify!(#name),
            &[#(#field_name_strs),*],
            Visitor {
                marker: ::std::marker::PhantomData,
            },
        )
    }
}
