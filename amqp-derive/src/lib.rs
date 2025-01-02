use crate::derive_for_enum::derive_for_enum;
use crate::derive_for_struct::derive_for_struct;
use crate::derive_for_union::derive_for_union;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Meta};

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


fn parse_descriptor(span: Span, attrs: &Vec<Attribute>) -> syn::Result<String> {
    for attr in attrs {
        if attr.path().is_ident("amqp") {
            if let Meta::List(meta_list) = &attr.meta {
                for token in meta_list.tokens.clone() {
                    match token {
                        TokenTree::Ident(desc) => {
                            if desc != "descriptor" {
                                let message = format!("found '{}' but expected 'descriptor'", desc);
                                return Err(syn::Error::new_spanned(desc.clone(), message));
                            }
                        }
                        TokenTree::Literal(lit) => {
                            let value = lit.to_string();
                            return if value.is_ascii() {
                                if value.starts_with('"') && value.ends_with('"') {
                                    Ok(value[1..value.len() - 1].to_string())
                                } else {
                                    Ok(value)
                                }
                            } else {
                                Err(syn::Error::new_spanned(
                                    lit.clone(),
                                    "Descriptor literal must only contain ASCII Characters",
                                ))
                            };
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Err(syn::Error::new(
        span,
        "Missing `amqp` attribute with `descriptor` key",
    ))
}