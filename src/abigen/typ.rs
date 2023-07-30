use heck::ToUpperCamelCase;
use proc_macro2::{TokenStream, Span};
use quote::quote;

pub struct Type {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_used: bool,
}

impl From<&super::abi::ABIStruct> for Type {
    fn from(abi_struct: &super::abi::ABIStruct) -> Self {
        Type {
            name: abi_struct.name.to_string(),
            fields: abi_struct.fields.iter().map(Field::from).collect(),
            is_used: false,
        }
    }
}

impl Type {
    pub fn generate(&self) -> TokenStream {

        let camel_name = syn::Ident::new(&self.name.to_upper_camel_case(), Span::call_site());
        let params: Vec<TokenStream> = vec![];

        quote! {
            #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct #camel_name {
                #(#params),*
            }
        }
    }
}

pub struct Field {
    pub name: String,
    pub ty: String,
    pub is_optional: bool,
    pub is_array: bool,
}



impl From<&super::abi::ABIType> for Field {
    fn from(abi_type: &super::abi::ABIType) -> Self {
        Field {
            name: abi_type.name.to_string(),
            ty: abi_type.ty.trim_end_matches("?").trim_end_matches("[]").to_string(),
            is_optional: abi_type.ty.ends_with("?"),
            is_array: abi_type.ty.ends_with("[]"),
        }
    }
}