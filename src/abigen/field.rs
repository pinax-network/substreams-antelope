use proc_macro2::{Span, TokenStream};
use quote::quote;

use super::abi::ABIType;
use super::rust::{custom_deserializer, is_reserved, rust_type};

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

impl Field {
    pub fn generate(&self) -> TokenStream {
        let (ident, is_reserved) = if is_reserved(self.name.as_str()) {
            (syn::Ident::new(&format!("{}_", self.name.as_str()), Span::call_site()), true)
        } else {
            (syn::Ident::new(self.name.as_str(), Span::call_site()), false)
        };
        let deserializer = match custom_deserializer(self.ty.as_str()) {
            Some(de) => quote! { #[serde(deserialize_with = #de)] },
            None => quote! {},
        };
        let ty = rust_type(self.ty.as_str());
        let ty_idents = ty.split("::").map(|s| syn::Ident::new(s, Span::call_site())).collect::<Vec<_>>();
        let ty = match self.is_array {
            true => quote!{ Vec<#(#ty_idents)::*> },
            false => quote!{  #(#ty_idents)::* },
        };

        let rename = if is_reserved {
            let ty = self.name.as_str();
            quote! { #[serde(rename = #ty)] }
        } else {
            quote! {}
        };
        quote! { #deserializer #rename pub #ident: #ty}
    }
}
