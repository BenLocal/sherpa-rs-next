use proc_macro::TokenStream;

mod derives;

#[proc_macro_derive(FromBaseConfig, attributes(base_config))]
pub fn derive_from_base_config(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match derives::expand_derive_from_base_config(&input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
