use proc_macro2::TokenStream;
use quote::quote;

use super::abi::ABI;
use super::action::Action;
use super::ty::Type;

pub struct Contract {
    pub types: Vec<Type>,
    pub actions: Vec<Action>,
}

impl Contract {
    pub fn generate(&self) -> TokenStream {
        let types: Vec<_> = self.types.iter().filter(|ty| !ty.is_entity).map(Type::generate).collect();
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
        let mut types = abi.structs.into_iter().map(Type::from).collect::<Vec<_>>();

        let actions: Vec<Action> = abi
            .actions
            .into_iter()
            .map(|abi_action| {
                match types.iter_mut().find(|ty|abi_action.ty == ty.name) {
                    Some(ty) => {
                        ty.is_entity = true;
                        Action::from((abi_action.name, ty.clone()))
                    }
                    None => panic!("No type found for action {}", abi_action.name)
                }
            })
            .collect();

        Contract { types, actions }
    }
}
