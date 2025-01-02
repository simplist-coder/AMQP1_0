use syn::DeriveInput;

pub(crate) fn derive_for_union(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    Err(syn::Error::new_spanned(
        input,
        "Unions are not supported.",
    ))
}