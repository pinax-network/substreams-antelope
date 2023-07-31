use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;

use super::abi::ABIStruct;
use super::field::Field;

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
        let fields: Vec<_> = self.fields.iter().map(|field| field.generate()).collect();

        quote! {
            #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct #camel_name {
                #(#fields),*
            }
        }
    }
}
