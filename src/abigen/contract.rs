use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

use super::abi::ABI;
use super::action::Action;
use super::ty::Type;

pub struct Contract {
    pub types: HashMap<String, Type>,
    pub actions: Vec<Action>,
}

impl Contract {
    pub fn generate(&self) -> TokenStream {
        let types: Vec<_> = self.types.values().filter(|ty| !ty.is_entity).map(Type::generate).collect();
        let actions: Vec<_> = self.actions.iter().map(Action::generate).collect();

        quote! {
            pub mod types {
                #(#types)*
            }
            pub mod actions {
                #(#actions)*
            }
        }
    }
}

impl From<ABI> for Contract {
    fn from(abi: ABI) -> Self {
        let mut types = abi.structs.into_iter().fold(HashMap::new(), |mut acc, abi_struct| {
            acc.insert(abi_struct.name.clone(), Type::from(abi_struct));
            acc
        });

        let actions: Vec<Action> = abi
            .actions
            .into_iter()
            .map(|abi_action| {
                if let Some(ty) = types.get_mut(&abi_action.ty) {
                    ty.is_entity = true;
                }
                Action::from((abi_action.name, types[&abi_action.ty].clone()))
            })
            .collect();

        Contract { types, actions }
    }
}
