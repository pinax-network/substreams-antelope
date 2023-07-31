use crate::abigen::abi::{custom_deserializer, is_reserved, rust_type};
use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;

use super::abi::{ABIStruct, ABIType};

#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_entity: bool,
}

impl From<ABIStruct> for Type {
    fn from(abi_struct: ABIStruct) -> Self {
        Type {
            name: abi_struct.name,
            fields: abi_struct.fields.into_iter().map(Field::from).collect(),
            is_entity: false,
        }
    }
}

impl Type {
    pub fn generate(&self) -> TokenStream {
        let camel_name = syn::Ident::new(&self.name.to_upper_camel_case(), Span::call_site());
        let fields: Vec<_> = self
            .fields
            .iter()
            .map(|param| {
                let (ident, is_reserved) = if is_reserved(param.name.as_str()) {
                    (syn::Ident::new(&format!("{}_", param.name.as_str()), Span::call_site()), true)
                } else {
                    (syn::Ident::new(param.name.as_str(), Span::call_site()), false)
                };
                let deserializer = match custom_deserializer(param.ty.as_str()) {
                    Some(de) => quote! { #[serde(deserialize_with = #de)] },
                    None => quote! {},
                };
                let ty = rust_type(&param.ty);
                let rename = if is_reserved {
                    let ty = param.name.as_str();
                    quote! { #[serde(rename = #ty)] }
                } else {
                    quote! {}
                };
                quote! { #deserializer #rename pub #ident: #ty}
            })
            .collect();

        quote! {
            #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct #camel_name {
                #(#fields),*
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub is_optional: bool,
    pub is_array: bool,
}

impl From<ABIType> for Field {
    fn from(abi_type: ABIType) -> Self {
        Field {
            name: abi_type.name.to_string(),
            ty: abi_type.ty.trim_end_matches('?').trim_end_matches("[]").to_string(),
            is_optional: abi_type.ty.ends_with('?'),
            is_array: abi_type.ty.ends_with("[]"),
        }
    }
}
