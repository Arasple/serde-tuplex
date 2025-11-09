//! Type analysis for determining lenient parsing behavior.

use syn::{Field, Type};

/// Check if a type is a primitive numeric type.
pub fn is_base_numeric(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty
        && let Some(ident) = type_path.path.get_ident()
    {
        return matches!(
            ident.to_string().as_str(),
            "u8" | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
                | "f32"
                | "f64"
        );
    }
    false
}

/// Check if type is numeric or `Option<numeric>`.
pub fn is_numeric_type(ty: &Type) -> bool {
    if is_base_numeric(ty) {
        return true;
    }

    if let Type::Path(type_path) = ty
        && type_path.path.segments.len() == 1
    {
        let segment = &type_path.path.segments[0];
        if segment.ident == "Option"
            && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
            && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
        {
            return is_base_numeric(inner_ty);
        }
    }
    false
}

/// Check if type is `Option<T>`.
pub fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty
        && type_path.path.segments.len() == 1
    {
        let segment = &type_path.path.segments[0];
        return segment.ident == "Option";
    }
    false
}

/// Extract `#[serde_tuplex(skip)]` attribute.
///
/// Returns `Some(false)` if skip is present, `Some(true)` if attribute exists without skip, `None` otherwise.
pub fn get_serde_tuplex_attr(field: &Field) -> Option<bool> {
    for attr in &field.attrs {
        if attr
            .path()
            .get_ident()
            .map(|i| i == "serde_tuplex")
            .unwrap_or(false)
        {
            if let Ok(meta_list) = attr.meta.require_list() {
                for token in meta_list.tokens.clone().into_iter() {
                    if let proc_macro2::TokenTree::Ident(ident) = token
                        && ident == "skip"
                    {
                        return Some(false);
                    }
                }
            }
            return Some(true);
        }
    }
    None
}

/// Determine if field should use lenient parsing based on type and attributes.
pub fn should_be_lenient(field: &Field) -> bool {
    match get_serde_tuplex_attr(field) {
        Some(true) => true,
        Some(false) => false,
        None => is_numeric_type(&field.ty),
    }
}
