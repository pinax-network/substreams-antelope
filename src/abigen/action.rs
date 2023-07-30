use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::abigen::abi::{rust_type, is_reserved, custom_deserializer};

use super::abi::{ABIStruct, ABIAction};
use super::ty::{Field, Type};

/// Structure used to generate contract's action interface.
#[derive(Debug)]
pub struct Action {
    pub name: String,
    pub ty: Type, // Action owns a Type
}

// impl From<ABIAction> for Action {
//     fn from(abi_action: ABIAction) -> Self {
//         Action {
//             name: abi_action.name,
//             ty: abi_action.ty,
//         }
//     }
// }

impl Action {

    /// Generates rust interface for contract's event.
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let camel_name = syn::Ident::new(&self.name.to_upper_camel_case(), Span::call_site());
        let types: Vec<_> = self
            .ty
            .fields
            .iter()
            .map(|param| (&param.ty, rust_type(&param.ty)))
            .collect();

        let action_params: Vec<_> = self
            .ty
            .fields
            .iter()
            .map(|param| {
                if is_reserved(param.name.as_str()) {
                    syn::Ident::new(&format!("{}_", param.name.as_str()), Span::call_site())
                }
                else {
                    syn::Ident::new(param.name.as_str(), Span::call_site())
                }
            })
            .zip(types.iter())
            .map(|(name, (ty_str, ty))| {
                let de = custom_deserializer(ty_str.as_str());
                match de {
                    Some(de) => {
                        quote! { #[serde(deserialize_with = #de)] pub #name: #ty }
                    },
                    None => quote! { pub #name: #ty },
                }
            })
            .collect();

        quote! {
            #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct #camel_name {
                #(#action_params),*
            }
            impl substreams_antelope::action::Action for #camel_name {
                const NAME: &'static str = #name;
                fn decode(
                    trace: &substreams_antelope::pb::ActionTrace,
                ) -> Result<Self, substreams_antelope::errors::Error> {
                    Ok(substreams_antelope::decoder::decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
                }
            }
        }
    }
}
