use proc_macro2::TokenStream;
use quote::quote;

use super::abi::ABI;
use super::action::Action;
use super::ty::Type;

#[derive(Debug, Clone, Default)]
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
                use substreams_antelope::types::*;
                #(#types)*
            }
            pub mod actions {
                use substreams_antelope::types::*;
                use super::types::*;
                use substreams_antelope::decoder::decode;
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
            .map(|abi_action| match types.iter_mut().find(|ty| abi_action.ty == ty.name) {
                Some(ty) => {
                    ty.is_entity = true;
                    Action::from((abi_action.name, ty.clone()))
                }
                None => panic!("No type found for action {}", abi_action.name),
            })
            .collect();

        Contract { types, actions }
    }
}

#[cfg(test)]
mod test {
    use crate::abi::ABI;
    use crate::assert::assert_ast_eq;
    use quote::quote;

    use super::Contract;

    #[test]
    fn test_empty() {
        let abi_json = r#"{
            "version": "eosio::abi/1.2",
            "types": [],
            "structs": [],
            "actions": [],
            "tables": []
        }"#;
        let abi_contract = ABI::try_from(abi_json).expect("failed to load ABI from JSON");
        let c = Contract::from(abi_contract);

        assert_ast_eq(
            c.generate(),
            quote! {
                pub mod types {
                    use substreams_antelope::types::*;
                }
                pub mod actions {
                    use substreams_antelope::types::*;
                    use super::types::*;
                    use substreams_antelope::decoder::decode;
                }
            },
        );
    }

    #[test]
    fn test_token() {
        let abi_contract = Contract { ..Default::default() };

        let c = Contract::from(abi_contract);

        assert_ast_eq(
            c.generate(),
            quote! {
                pub mod types {
                    use substreams_antelope::types::*;
                }
                pub mod actions {
                    use substreams_antelope::types::*;
                    use super::types::*;
                    use substreams_antelope::decoder::decode;
                }
            },
        );
    }
}
