use std::collections::HashMap;

use proc_macro2::{TokenStream, Span};
use quote::quote;

use super::action::{Action};
use super::abi::ABI;
use super::ty::{Type, Field};

pub struct Contract {
    pub types: HashMap<String, Type>,
    pub actions: Vec<Action>,
}

impl Contract {
    /// Generates rust interface for a contract.
    pub fn generate(&self) -> TokenStream {
        let actions: Vec<_> = self.actions.iter().map(Action::generate).collect();
        let types: Vec<_> = self.types.values().filter(|ty| ty.is_used).map(Type::generate).collect();

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


// fn get_type_by_name<'a>(types: &'a Vec<Type>, name: String) -> Option<&'a Type> {
//     types.iter().find(|t| t.name == name)
// }
impl From<ABI> for Contract {
    fn from(abi: ABI) -> Self {

        let mut types = abi.structs.into_iter().fold(
            HashMap::new(),
            |mut acc, abi_struct| {
                acc.insert(abi_struct.name.clone(), Type {
                    name: abi_struct.name,
                    fields: abi_struct.fields.into_iter().map(Field::from).collect(),
                    is_used: false,
                });
                acc
            },
        );


        let actions: Vec<Action> = abi
            .actions
            .into_iter()
            .map(|abi_action| {
                if let Some(ty) = types.get_mut(&abi_action.ty) {
                    ty.is_used = true;
                }
                Action {
                    name: abi_action.name,
                    ty: types[&abi_action.ty].clone(),
                }
            })
            .collect();

        Contract {
            types,
            actions,
        }
    }
}
