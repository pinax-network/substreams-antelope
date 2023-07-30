use proc_macro2::{TokenStream, Span};
use quote::quote;

use super::action::{Action, get_action_by_name};
use super::abi::ABI;
use super::typ::Type;

pub struct Contract {
    pub types: Vec<Type>,
    pub actions: Vec<Action>,
}


impl Contract {
    /// Generates rust interface for a contract.
    pub fn generate(&self) -> TokenStream {
        let types: Vec<_> = self.types.iter().filter(|t| t.is_used).map(Type::generate).collect();
        let actions: Vec<_> = self.actions.iter().map(Action::generate).collect();

        quote! {
            /// Contract types
            pub mod types {
                #(#types)*
            }
            /// Contract actions
            pub mod actions {
                #(#actions)*
            }
        }
    }
}


impl<'a> From<&'a ABI> for Contract {
    fn from(c: &'a ABI) -> Self {
        let types: Vec<_> = c
            .structs
            .iter()
            .map(Type::from)
            .collect();

        let actions: Vec<_> = c
            .actions
            .iter()
            .map(|action| get_action_by_name(&c.structs, &action.name).unwrap())
            .collect();

        Contract {
            actions,
            types,
        }
    }
}