use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DeriveInput, Fields, Path, Type};

pub(crate) fn derive_for_enum(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let enum_ident = &input.ident;
    if let Data::Enum(ref st) = input.data {
        let from_impls = generate_from_impls(st, enum_ident)?;
        let impl_try_from_primitive = generate_try_from_primitive(st, enum_ident)?;
        let impl_into_primitive = generate_into_primitive(st, enum_ident)?;
        let impl_composite_type = generate_composite_type_impl(st, enum_ident)?;
        let impl_try_from_primitive_for_option = generate_try_from_primitive_for_option(enum_ident)?;
        Ok(quote! {
            #impl_try_from_primitive

            #impl_try_from_primitive_for_option

            #impl_into_primitive

            #impl_composite_type

            #from_impls

        })
    } else {
        unreachable!("This should be unreachable, as we verify the input.data type in the top level function before calling this.")
    }
}

// Taken from here: https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
fn extract_type_path(ty: &Type) -> Option<&Path> {
    match *ty {
        Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
        _ => None,
    }
}

fn generate_from_impls(st: &DataEnum, enum_ident: &Ident) -> syn::Result<proc_macro2::TokenStream> {
    let from_impls = st.variants.iter().map(|v| {
        let variant_ident = &v.ident;
        match &v.fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    let field = &fields.unnamed[0];
                    if let Some(field_name) = extract_type_path(&field.ty) {
                        quote! {
                            impl From<#field_name> for #enum_ident {
                                fn from(value: #field_name) -> Self {
                                    Self::#variant_ident(value)
                                }
                            }
                        }
                    } else {
                        quote! {}
                    }
                } else {
                    syn::Error::new_spanned(&v.fields, "Amqp enums may only have one named field.")
                        .to_compile_error()
                }
            }
            _ => syn::Error::new_spanned(
                &v.fields,
                "Enum Auto derive only supports named enum fields.",
            )
            .to_compile_error(),
        }
    });
    Ok(quote! {
        #(#from_impls)*
    })
}

fn generate_try_from_primitive(
    st: &DataEnum,
    enum_ident: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    let match_cases = st.variants.iter().map(|v| {
        match &v.fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    let field = &fields.unnamed[0];
                    if let Some(field_name) = extract_type_path(&field.ty) {
                        quote_spanned! {
                            field.span() =>
                            x if x == #field_name::NAME || x == #field_name::CODE => Ok(#enum_ident::from(#field_name::try_from(value)?)),
                        }
                    } else {
                        quote! {}
                    }

                } else {
                    syn::Error::new_spanned(&v.fields, "Amqp enums may only have one named field.").to_compile_error()
                }
            }
            _ => syn::Error::new_spanned(&v.fields, "Enum Auto derive only supports named enum fields.").to_compile_error()
        }
    });

    Ok(quote! {
        impl TryFrom<crate::primitive::Primitive> for #enum_ident {
            type Error = crate::error::AppError;

            fn try_from(value: crate::primitive::Primitive) -> Result<Self, Self::Error> {
                use crate::composite::CompositeType;
                match &value {
                    crate::primitive::Primitive::Composite(ref comp) => {
                        match comp.descriptor() {
                            #(#match_cases)*
                            _ => Err(crate::error::amqp_error::AmqpError::DecodeError)?
                        }
                    }
                    _ => Err(crate::error::amqp_error::AmqpError::DecodeError)?,
                }
            }
        }
    })
}

fn generate_composite_type_impl(
    st: &DataEnum,
    enum_ident: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    let match_cases = st.variants.iter().map(|v| {
        let variant_ident = &v.ident;
        match &v.fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    let field = &fields.unnamed[0];
                    quote_spanned! {
                        field.span() =>
                        #enum_ident::#variant_ident(x) => x.descriptor(),

                    }
                } else {
                    syn::Error::new_spanned(&v.fields, "Amqp enums may only have one named field.")
                        .to_compile_error()
                }
            }
            _ => syn::Error::new_spanned(
                &v.fields,
                "Enum Auto derive only supports named enum fields.",
            )
            .to_compile_error(),
        }
    });

    Ok(quote! {
        impl crate::composite::CompositeType for #enum_ident {
            fn descriptor(&self) -> crate::composite::Descriptor {
                match self {
                    #(#match_cases)*
                }
            }
        }
    })
}

fn generate_into_primitive(
    st: &DataEnum,
    enum_ident: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    let match_cases = st.variants.iter().map(|v| {
        let variant_ident = &v.ident;
        match &v.fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    let field = &fields.unnamed[0];
                    quote_spanned! {
                        field.span() =>
                        #enum_ident::#variant_ident(x) => x.into(),

                    }
                } else {
                    syn::Error::new_spanned(&v.fields, "Amqp enums may only have one named field.")
                        .to_compile_error()
                }
            }
            _ => syn::Error::new_spanned(
                &v.fields,
                "Enum Auto derive only supports named enum fields.",
            )
            .to_compile_error(),
        }
    });

    Ok(quote! {
       impl Into<crate::primitive::Primitive> for #enum_ident {
            fn into(self) -> crate::primitive::Primitive {
                match self {
                    #(#match_cases)*
                }
            }
        }
    })
}

fn generate_try_from_primitive_for_option(
    enum_ident: &Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {
        impl TryFrom<crate::primitive::Primitive> for Option<#enum_ident> {
            type Error = crate::error::AppError;

            fn try_from(value: crate::primitive::Primitive) -> Result<Self, Self::Error> {
                match &value {
                    crate::primitive::Primitive::Null => Ok(None),
                    crate::primitive::Primitive::Composite(_) => {
                        Ok(Some(#enum_ident::try_from(value)?))
                    }
                    _ => Err(crate::error::amqp_error::AmqpError::DecodeError)?
                }
            }
        }
    })
}
