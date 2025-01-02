use crate::derive_for_enum::derive_for_enum;
use crate::derive_for_struct::derive_for_struct;
use crate::derive_for_union::derive_for_union;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

mod derive_for_enum;
mod derive_for_struct;
mod derive_for_union;

#[proc_macro_derive(CompositeType, attributes(amqp))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match &input.data {
        Data::Struct(_) => derive_for_struct(input),
        Data::Enum(_) => derive_for_enum(input),
        Data::Union(_) => derive_for_union(input),
    };

    output.unwrap_or_else(|err| err.to_compile_error()).into()
}
