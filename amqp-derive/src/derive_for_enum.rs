use syn::{Data, DeriveInput, Fields};
use syn::spanned::Spanned;
use crate::parse_descriptor;

pub(crate) fn derive_for_enum(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    if let Data::Enum(ref st) = input.data {
        for variant in &st.variants {
            let _variant_name = &variant.ident;
            let _descriptor = parse_descriptor(variant.span(), &variant.attrs);
            match &variant.fields {
                Fields::Named(_fields) => {}
                Fields::Unnamed(_fields) => {}
                Fields::Unit => {}
            }

        }
        Err(syn::Error::new_spanned(
            input,
            "Enums are not supported.",
        ))
    } else {
        unreachable!("This should be unreachable, as we verify the input.data type in the top level function before calling this.")
    }
}