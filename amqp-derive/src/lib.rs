use crate::derive_for_enum::derive_for_enum;
use crate::derive_for_struct::derive_for_struct;
use crate::derive_for_union::derive_for_union;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Meta, Token};

mod derive_for_enum;
mod derive_for_struct;
mod derive_for_union;

#[proc_macro_derive(AmqpComposite, attributes(amqp))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match &input.data {
        Data::Struct(_) => derive_for_struct(input),
        Data::Enum(_) => derive_for_enum(input),
        Data::Union(_) => derive_for_union(input),
    };

    output.unwrap_or_else(|err| err.to_compile_error()).into()
}

#[allow(dead_code)]
struct Descriptors {
    name: Ident,
    eq_1: Token![=],
    name_value: Literal,
    sep: Token![,],
    code: Ident,
    eq_2: Token![=],
    code_value: Literal,
}

impl Parse for Descriptors {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Descriptors {
            name: input.parse()?,
            eq_1: input.parse()?,
            name_value: input.parse()?,
            sep: input.parse()?,
            code: input.parse()?,
            eq_2: input.parse()?,
            code_value: input.parse()?,
        })
    }
}

impl Descriptors {
    fn name_value(&self) -> syn::Result<String> {
        let value = self.name_value.to_string();
        if value.is_ascii() && value.starts_with('"') && value.ends_with('"') {
            Ok(value[1..value.len() - 1].to_string())
        } else {
            Err(syn::Error::new_spanned(
                self.name_value.clone(),
                "Descriptor 'name' literal must only contain ASCII Characters and must be a string",
            ))
        }
    }

    fn code_value(&self) -> syn::Result<Literal> {
        Ok(self.code_value.clone())
    }
}

fn parse_descriptor(span: Span, attrs: &Vec<Attribute>) -> syn::Result<Descriptors> {
    for attr in attrs {
        if attr.path().is_ident("amqp") {
            if let Meta::List(meta_list) = &attr.meta {
                return Ok(syn::parse2(meta_list.clone().tokens)?)
            }
        }
    }

    Err(syn::Error::new(
        span,
        "Missing `amqp` attribute with `descriptor` key",
    ))
}
