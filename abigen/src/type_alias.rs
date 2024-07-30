use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::abi::ABITypes;

#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub new_name: String,
    pub old_name: String,
}

impl From<ABITypes> for TypeAlias {
    fn from(abi_type: ABITypes) -> Self {
        TypeAlias {
            new_name: abi_type.new_type_name,
            old_name: abi_type.ty,
        }
    }
}

impl TypeAlias {
    pub fn generate(&self) -> TokenStream {
        let camel_new_name = syn::Ident::new(&self.new_name.to_upper_camel_case(), Span::call_site());
        let camel_old_name = syn::Ident::new(&self.old_name.to_upper_camel_case(), Span::call_site());

        quote! {
            pub type #camel_new_name = #camel_old_name;
        }
    }
}
