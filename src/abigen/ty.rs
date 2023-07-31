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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use crate::abigen::abi::{ABIStruct, ABIType};
    #[test]
    fn test_type_generate() {
        let abi_struct = ABIStruct {
            name: "my_struct".to_string(),
            fields: vec![
                ABIType {
                    name: "bool_field".to_string(),
                    ty: "bool".to_string(),
                },
                ABIType {
                    name: "name_field".to_string(),
                    ty: "name".to_string(),
                },
                ABIType {
                    name: "uint64_field".to_string(),
                    ty: "uint64".to_string(),
                },
            ],
            base: "".to_string(),
        };
        let typ = Type::from(abi_struct);

        let generated_tokens = typ.generate();

        let expected_tokens = quote! {
            #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct MyStruct {
                pub bool_field: Bool,
                pub name_field: Name,
                #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
                pub uint64_field: Uint64
            }
        };
        assert_eq!(generated_tokens.to_string(), expected_tokens.to_string());
    }
}