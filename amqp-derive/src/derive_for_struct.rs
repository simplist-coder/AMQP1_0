use crate::{parse_descriptor, Descriptors};
use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Index};

/// should create the required impls for a struct
/// should look something like this:
///
///
///
///
/// #[derive(CompositeType)]
/// #[amqp(name = "my:teststruct", code = 123)]
/// struct TestStruct { }

pub(crate) fn derive_for_struct(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    if let Data::Struct(ref st) = input.data {
        let descriptor = parse_descriptor(input.span(), &input.attrs)?;
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
    descriptor: Descriptors,
    fields: &FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
    let try_from_fields_expression = try_from_named_fields(fields);
    let builder_push_expression = builder_push_named_fields(fields);
    let try_from_for_optional = try_from_primitive_for_optional(name);

    let name_value = descriptor.name_value()?;
    let code_value = descriptor.code_value()?;

    Ok(quote! {
        impl #name {
            pub const NAME: &'static str = #name_value;
            pub const CODE: u64 = #code_value;
        }


        impl crate::composite::CompositeType for #name {
            fn descriptor(&self) -> crate::composite::Descriptor {
                crate::primitive::variable_width::symbol::Symbol::with_ascii(#name_value).into()
            }
        }

        impl ::core::convert::TryFrom<crate::primitive::Primitive> for #name {
            type Error = crate::error::AppError;

            fn try_from(value: crate::primitive::Primitive) -> Result<Self, Self::Error> {
                match value {
                    crate::primitive::Primitive::Composite(mut comp) => {
                        Ok(Self {
                            #try_from_fields_expression
                        })
                    }
                    _ => Err(crate::error::amqp_error::AmqpError::DecodeError)?
                }
            }
        }

        impl ::core::convert::From<#name> for crate::primitive::Primitive {
            fn from(value: #name) -> Self {
                crate::composite::builder::CompositeBuilder::new(crate::composite::CompositeType::descriptor(&value))

                    #builder_push_expression

                    .build()
                    .into()
            }
        }

        #try_from_for_optional

    })
}

fn generate_unnamed_impl(
    name: &Ident,
    descriptor: Descriptors,
    fields: &FieldsUnnamed,
) -> syn::Result<proc_macro2::TokenStream> {
    let try_from_fields_expression = try_from_unnamed_fields(fields);
    let builder_push_expression = builder_push_unnamed_fields(fields);
    let try_from_for_optional = try_from_primitive_for_optional(name);

    let name_value = descriptor.name_value()?;
    let code_value = descriptor.code_value()?;

    Ok(quote! {

        impl #name {
            pub const NAME: &'static str = #name_value;
            pub const CODE: u64 = #code_value;
        }

        impl crate::composite::CompositeType for #name {

            fn descriptor(&self) -> crate::composite::Descriptor {
                crate::primitive::variable_width::symbol::Symbol::with_ascii(#name_value).into()
            }
        }

        impl ::core::convert::TryFrom<crate::primitive::Primitive> for #name {
            type Error = crate::error::AppError;

            fn try_from(value: crate::primitive::Primitive) -> Result<Self, Self::Error> {
                match value {
                    crate::primitive::Primitive::Composite(mut comp) => {
                        Ok(Self (
                            #try_from_fields_expression
                        ))
                    }
                    _ => Err(crate::error::amqp_error::AmqpError::DecodeError)?
                }
            }
        }

        impl ::core::convert::From<#name> for crate::primitive::Primitive {
            fn from(value: #name) -> Self {
                crate::composite::builder::CompositeBuilder::new(crate::composite::CompositeType::descriptor(&value))

                    #builder_push_expression

                    .build()
                    .into()
            }
        }

        #try_from_for_optional
    })
}

fn try_from_primitive_for_optional(name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        impl ::core::convert::TryFrom<crate::primitive::Primitive> for std::option::Option<#name> {
            type Error = crate::error::AppError;

            fn try_from(value: crate::primitive::Primitive) -> Result<Self, Self::Error> {
                match value {
                    crate::primitive::Primitive::Null => Ok(None),
                    crate::primitive::Primitive::Composite(c) => {
                        let source = #name::try_from(crate::primitive::Primitive::from(c));
                        match source {
                            Ok(s)=> Ok(Some(s)),
                            Err(_) => Ok(None),
                        }
                    }
                    _ => Err(crate::error::amqp_error::AmqpError::DecodeError)?
                }
            }
        }
    }
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
        #(#recurse)*
    }
}








// todo: add better error messages
// todo: add validation for only ascii text in descriptor with corresponding error message
//

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn test_parse_descriptor_valid() {
        let input: DeriveInput = parse_quote! {
            #[amqp(name = "my:teststruct", code = 123)]
            struct TestStruct;
        };

        let result = parse_descriptor(input.span(), &input.attrs).unwrap();
        assert_eq!(result.name_value().unwrap(), "my:teststruct");
        assert_eq!(result.code_value().unwrap().to_string(), "123");
    }

    #[test]
    fn test_parse_descriptor_missing_descriptor() {
        let input: DeriveInput = parse_quote! {
            #[amqp(other_key = "value")]
            struct TestStruct;
        };

        let result = parse_descriptor(input.span(), &input.attrs);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_descriptor_no_amqp_attribute() {
        let input: DeriveInput = parse_quote! {
            struct TestStruct;
        };

        let result = parse_descriptor(input.span(), &input.attrs);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_descriptor_invalid_literal() {
        let input: DeriveInput = parse_quote! {
            #[amqp(name = 123)]
            struct TestStruct;
        };

        let result = parse_descriptor(input.span(), &input.attrs);
        assert!(result.is_err());
    }
}
