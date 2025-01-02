use proc_macro2::{Ident, TokenTree};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Index, Meta};

/// should create the required impls for a struct
/// should look something like this:
///
///
///
///
/// #[derive(CompositeType)]
/// #[amqp(descriptor = "my:teststruct")]
/// struct TestStruct { }
///
pub(crate) fn derive_for_struct(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    if let Data::Struct(ref st) = input.data {
        let descriptor = parse_descriptor(&input)?;
        match st.fields {
            Fields::Named(ref fields) => generate_named_impl(name, descriptor, fields),
            Fields::Unnamed(ref fields) => generate_unnamed_impl(name, descriptor, fields),
            Fields::Unit => Err(syn::Error::new_spanned(
                name,
                "Unit structs are not supported",
            )),
        }
    } else {
        unreachable!("This should be unreachable, as we verify the input.data type in the top level function before calling this.")
    }
}

fn generate_named_impl(
    name: &Ident,
    descriptor: String,
    fields: &FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
    let try_from_fields_expression = try_from_named_fields(fields);
    let builder_push_expression = builder_push_named_fields(fields);
    Ok(quote! {
        use amqp_type::primitive::composite::Descriptor;

        impl amqp_type::primitive::composite::CompositeType for #name {
            fn descriptor(&self) -> Descriptor {
                amqp_type::primitive::variable_width::symbol::Symbol::with_ascii(#descriptor).into()
            }
        }

        impl ::core::convert::TryFrom<amqp_type::primitive::Primitive> for #name {
            type Error = amqp_type::error::AppError;

            fn try_from(value: amqp_type::primitive::Primitive) -> Result<Self, Self::Error> {
                match value {
                    amqp_type::primitive::Primitive::Composite(mut comp) => {
                        Ok(Self {
                            #try_from_fields_expression
                        })
                    }
                    _ => Err(amqp_type::error::amqp_error::AmqpError::DecodeError)?
                }
            }
        }

        impl ::core::convert::From<#name> for amqp_type::primitive::Primitive {
            fn from(value: #name) -> Self {
                amqp_type::primitive::composite::builder::CompositeBuilder::new(value.descriptor())

                    #builder_push_expression

                    .build()
                    .into()
            }
        }
    })
}

fn generate_unnamed_impl(
    name: &Ident,
    descriptor: String,
    fields: &FieldsUnnamed,
) -> syn::Result<proc_macro2::TokenStream> {
    let try_from_fields_expression = try_from_unnamed_fields(fields);
    let builder_push_expression = builder_push_unnamed_fields(fields);

    Ok(quote! {
        use amqp_type::primitive::composite::Descriptor;

        impl amqp_type::primitive::composite::CompositeType for #name {
            fn descriptor(&self) -> Descriptor {
                amqp_type::primitive::variable_width::symbol::Symbol::with_ascii(#descriptor).into()
            }
        }

        impl ::core::convert::TryFrom<amqp_type::primitive::Primitive> for #name {
            type Error = amqp_type::error::AppError;

            fn try_from(value: amqp_type::primitive::Primitive) -> Result<Self, Self::Error> {
                match value {
                    amqp_type::primitive::Primitive::Composite(mut comp) => {
                        Ok(Self (
                            #try_from_fields_expression
                        ))
                    }
                    _ => Err(amqp_type::error::amqp_error::AmqpError::DecodeError)?
                }
            }
        }

        impl ::core::convert::From<#name> for amqp_type::primitive::Primitive {
            fn from(value: #name) -> Self {
                amqp_type::primitive::composite::builder::CompositeBuilder::new(value.descriptor())

                    #builder_push_expression

                    .build()
                    .into()
            }
        }
    })
}

fn try_from_named_fields(fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let recurse = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote_spanned! {
            f.span()=> #name: comp.pop_front().try_into()?
        }
    });

    quote! {
        #(#recurse),*
    }
}

fn builder_push_named_fields(fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let recurse = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote_spanned! {
            f.span()=> .push(value.#name.into())
        }
    });

    quote! {
        #(#recurse)*
    }
}

fn try_from_unnamed_fields(fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let recurse = fields.unnamed.iter().map(|f| {
        quote_spanned! {
            f.span()=> comp.pop_front().try_into()?
        }
    });

    quote! {
        #(#recurse),*
    }
}

fn builder_push_unnamed_fields(fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
        let index = Index::from(i);
        quote_spanned! {
            f.span()=> .push(value.#index.into())
        }
    });

    quote! {
        #(#recurse),*
    }
}








// todo: add better error messages
// todo: add validation for only ascii text in descriptor with corresponding error message
//
fn parse_descriptor(input: &DeriveInput) -> syn::Result<String> {
    for attr in &input.attrs {
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
        input.span(),
        "Missing `amqp` attribute with `descriptor` key",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn test_parse_descriptor_valid() {
        let input: DeriveInput = parse_quote! {
            #[amqp(descriptor="my:teststruct")]
            struct TestStruct;
        };

        let result = parse_descriptor(&input);
        assert_eq!(result.unwrap(), "my:teststruct");
    }

    #[test]
    fn test_parse_descriptor_missing_descriptor() {
        let input: DeriveInput = parse_quote! {
            #[amqp(other_key = "value")]
            struct TestStruct;
        };

        let result = parse_descriptor(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_descriptor_no_amqp_attribute() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct;
        };

        let result = parse_descriptor(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_descriptor_invalid_literal() {
        let input: DeriveInput = parse_quote! {
            #[amqp(descriptor = 123)]
            struct TestStruct;
        };

        let result = parse_descriptor(&input);
        assert_eq!(result.unwrap(), "123");
    }
}
