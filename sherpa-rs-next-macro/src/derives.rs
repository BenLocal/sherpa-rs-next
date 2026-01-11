use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, FnArg, ImplItem,
    PatType, Signature, Type, TypePath, TypeReference, punctuated::Punctuated, token::Comma,
};

pub fn expand_derive_from_base_config(input: &DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => expand_derive_from_base_config_struct(input, named),

        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { .. }),
            ..
        }) => Err(syn::Error::new_spanned(
            input,
            "unnamed structs are not supported",
        )),

        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => Err(syn::Error::new_spanned(
            input,
            "unit structs are not supported",
        )),

        Data::Enum(_) => Err(syn::Error::new_spanned(input, "enums are not supported")),

        Data::Union(_) => Err(syn::Error::new_spanned(input, "unions are not supported")),
    }
}

fn expand_derive_from_base_config_struct(
    input: &DeriveInput,
    named: &Punctuated<Field, Comma>,
) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    // Find all fields with #[base_config] attribute
    let mut delegate_fields = Vec::new();

    for field in named {
        if extract_base_config_attr_exists(field) {
            let path = extract_base_config_path(field)?;
            let field_name = field
                .ident
                .as_ref()
                .ok_or_else(|| syn::Error::new_spanned(field, "field must have a name"))?;
            let field_type = &field.ty;
            delegate_fields.push((field_name.clone(), field_type.clone(), path.clone()));
        }
    }

    if delegate_fields.is_empty() {
        return Err(syn::Error::new_spanned(
            input,
            "no fields found with #[base_config] attribute",
        ));
    }

    // Generate delegate methods for each field
    let mut methods = Vec::new();

    for (field_name, field_type, source_path) in &delegate_fields {
        let field_ident = field_name;

        // Extract methods from the field type using the provided source path
        let method_list = extract_with_methods_from_type(field_type, source_path)?;

        for (method_name, param_type) in method_list {
            let method_ident = syn::Ident::new(&format!("with_{}", method_name), Span::call_site());

            methods.push(quote! {
                pub fn #method_ident(&mut self, param: #param_type) -> &mut Self {
                    self.#field_ident.#method_ident(param);
                    self
                }
            });
        }
    }

    Ok(quote! {
        impl #struct_name {
            #(#methods)*
        }
    })
}

fn extract_base_config_attr_exists(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("base_config"))
}

fn extract_base_config_path(field: &Field) -> syn::Result<Option<String>> {
    for attr in &field.attrs {
        if attr.path().is_ident("base_config") {
            // Parse the attribute to extract the path parameter
            // Support both #[base_config(path = "...")] and #[base_config("...")]
            // Check if it's a simple path attribute (no arguments)
            if matches!(attr.meta, syn::Meta::Path(_)) {
                return Ok(None);
            }

            let meta = attr.parse_args::<syn::Meta>()?;

            match meta {
                syn::Meta::NameValue(name_value) => {
                    // #[base_config(path = "...")]
                    if name_value.path.is_ident("path") {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = name_value.value
                        {
                            return Ok(Some(lit_str.value()));
                        }
                    }
                }
                syn::Meta::List(list) => {
                    // #[base_config("...")]
                    if let Ok(expr) = syn::parse2::<syn::Expr>(list.tokens.clone()) {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = expr
                        {
                            return Ok(Some(lit_str.value()));
                        }
                    }
                }
                syn::Meta::Path(_) => {
                    // #[base_config] without parameters
                    return Ok(None);
                }
            }
        }
    }
    Ok(None)
}

fn get_type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let segments: Vec<String> = path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect();
            Some(segments.join("::"))
        }
        _ => None,
    }
}

fn extract_with_methods_from_type(
    ty: &Type,
    source_path: &Option<String>,
) -> syn::Result<Vec<(String, TokenStream)>> {
    // Get the type name to try to find its source file
    let type_name = match get_type_name(ty) {
        Some(name) => name,
        None => {
            // If we can't get the type name, return empty list
            // The user will need to manually specify methods or use a trait
            return Ok(vec![]);
        }
    };

    // Try to find and parse the source file for this type
    // Use the provided path if available, otherwise try to find it automatically
    find_and_parse_type_impl(&type_name, source_path)
}

fn find_and_parse_type_impl(
    type_name: &str,
    source_path: &Option<String>,
) -> syn::Result<Vec<(String, TokenStream)>> {
    // Try to get the source file path
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok();

    let paths_to_try = if let Some(path) = source_path {
        // Use the provided path
        if let Some(dir) = manifest_dir {
            // If path is relative, resolve it relative to CARGO_MANIFEST_DIR
            if path.starts_with('/') {
                vec![path.clone()]
            } else {
                vec![format!("{}/{}", dir, path)]
            }
        } else {
            vec![path.clone()]
        }
    } else {
        // If no path provided, try to find it automatically
        // This is a fallback - user should provide the path
        vec![]
    };

    for path in paths_to_try {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                match parse_with_methods_from_source(&content, type_name) {
                    Ok(methods) if !methods.is_empty() => {
                        return Ok(methods);
                    }
                    Ok(_) => {
                        // File parsed but no methods found - continue to next path
                        continue;
                    }
                    Err(_e) => {
                        // Parsing error - continue to next path
                        continue;
                    }
                }
            }
            Err(_) => {
                // File not found - continue to next path
                continue;
            }
        }
    }

    // If we can't find the file or methods, return error
    if source_path.is_some() {
        Err(syn::Error::new(
            Span::call_site(),
            format!(
                "Cannot find methods for type '{}' in the specified source file. Please check:\n1. The file path is correct\n2. The type name matches\n3. The impl block contains 'with_' methods",
                type_name
            ),
        ))
    } else {
        Err(syn::Error::new(
            Span::call_site(),
            format!(
                "Cannot find source file for type: {}. Please specify the path using #[base_config(path = \"...\")].",
                type_name
            ),
        ))
    }
}

fn parse_with_methods_from_source(
    source: &str,
    type_name: &str,
) -> syn::Result<Vec<(String, TokenStream)>> {
    // Parse the source file
    let file = syn::parse_file(source)?;

    let mut methods = Vec::new();

    // Extract the base type name from the full path (e.g., "AsrOfflineBaseConfig" from "crate::asr::offline::AsrOfflineBaseConfig")
    let base_type_name = type_name.split("::").last().unwrap_or(type_name);

    // Find the impl block for the target type
    for item in file.items {
        if let syn::Item::Impl(impl_block) = item {
            // Check if this impl block is for our target type
            // We need to match the Self type in the impl block
            let impl_self_type = match impl_block.self_ty.as_ref() {
                Type::Path(type_path) => {
                    // Get the last segment of the path (the actual type name)
                    type_path
                        .path
                        .segments
                        .last()
                        .map(|seg| seg.ident.to_string())
                }
                _ => None,
            };

            // Check if the type name matches
            // Match both the full type name and the base type name
            let matches = impl_self_type
                .map(|t| {
                    // Match if the impl type equals the base type name, or if type_name contains the impl type
                    t == base_type_name || type_name.contains(&t) || t == type_name
                })
                .unwrap_or(false);

            if matches {
                // Extract all methods starting with "with_"
                for item in impl_block.items {
                    if let ImplItem::Fn(method) = item {
                        let method_name = method.sig.ident.to_string();
                        if method_name.starts_with("with_") {
                            // Extract the parameter type
                            if let Some(param_type) = extract_first_param_type(&method.sig) {
                                // Remove "with_" prefix to get the base name
                                let base_name =
                                    method_name.strip_prefix("with_").unwrap_or(&method_name);
                                methods.push((base_name.to_string(), param_type));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(methods)
}

fn extract_first_param_type(sig: &Signature) -> Option<TokenStream> {
    // Extract the type of the first parameter (after &mut self)
    for input in &sig.inputs {
        if let FnArg::Typed(PatType { ty, .. }) = input {
            // Skip &mut self
            if let Type::Reference(TypeReference { elem, .. }) = ty.as_ref() {
                // Check if it's &mut self
                if let Type::Path(type_path) = elem.as_ref() {
                    if type_path
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident == "Self")
                        .unwrap_or(false)
                    {
                        continue;
                    }
                }
            }

            // Return the type as a TokenStream
            return Some(quote! { #ty });
        }
    }
    None
}

#[cfg(test)]
#[path = "derives_test.rs"]
mod derives_test;
