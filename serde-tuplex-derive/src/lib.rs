//! Derive macros for serde-tuplex.
//!
//! See the main `serde-tuplex` crate for documentation.

mod analysis;
mod deserialize;
mod lenient;
mod serialize;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Serialize/deserialize structs as tuples: `[1,2]` instead of `{"x":1,"y":2}`.
///
/// ```
/// use serde_tuplex::Tuple;
///
/// #[derive(Tuple)]
/// struct Point { x: i32, y: i32 }
///
/// // Serializes as [10,20]
/// ```
#[proc_macro_derive(Tuple)]
pub fn derive_tuple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let serialize_impl = match serialize::gen_serialize_impl(&input) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };

    let deserialize_impl = match deserialize::gen_deserialize_impl(&input) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };

    let expanded = quote! {
        #serialize_impl
        #deserialize_impl
    };

    TokenStream::from(expanded)
}

/// Accept both `123` and `"123"` for numeric fields.
///
/// Numeric fields (integers, floats, `Option<numeric>`) automatically get lenient parsing.
/// Use `#[serde_tuplex(skip)]` to disable for specific fields.
///
/// ```
/// use serde_tuplex::Lenient;
///
/// #[derive(Lenient)]
/// struct Config {
///     timeout: u64,    // Accepts 3000 or "3000"
///     name: String,    // Standard
///
///     #[serde_tuplex(skip)]
///     strict: u32,     // Only accepts 42, not "42"
/// }
/// ```
#[proc_macro_derive(Lenient, attributes(serde_tuplex))]
pub fn derive_lenient(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    match lenient::gen_lenient_deserialize(&input, false) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Combine tuple format with lenient parsing.
///
/// Serializes as `[1,2]` and accepts both `[1,2]` and `["1","2"]` during deserialization.
///
/// ```
/// use serde_tuplex::TupleLenient;
///
/// #[derive(TupleLenient)]
/// struct Data {
///     timestamp: u64,
///     value: f64,
/// }
///
/// // Accepts [123, 4.5] or ["123", "4.5"]
/// ```
#[proc_macro_derive(TupleLenient, attributes(serde_tuplex))]
pub fn derive_tuple_lenient(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let serialize_impl = match serialize::gen_serialize_impl(&input) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };

    let deserialize_impl = match lenient::gen_lenient_deserialize(&input, true) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };

    let combined = quote! {
        #serialize_impl
        #deserialize_impl
    };

    TokenStream::from(combined)
}
