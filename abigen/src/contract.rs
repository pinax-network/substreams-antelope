use proc_macro2::TokenStream;
use quote::quote;

use crate::type_alias::TypeAlias;

use super::abi::ABI;
use super::action::Action;
use super::ty::Type;

#[derive(Debug, Clone, Default)]
pub struct Contract {
    pub types: Vec<Type>,
    pub type_aliases: Vec<TypeAlias>,
    pub actions: Vec<Action>,
}

impl Contract {
    pub fn generate(&self, account: Option<String>) -> TokenStream {
        let types: Vec<_> = self.types.iter().filter(|ty| !ty.is_entity).map(Type::generate).collect();
        let type_aliases: Vec<_> = self.type_aliases.iter().map(TypeAlias::generate).collect();
        let actions: Vec<_> = self.actions.iter().map(Action::generate).collect();

        let account = match account {
            Some(value) => quote! { Some(#value) },
            None => quote! { None },
        };

        quote! {
            #[allow(dead_code)]
            pub const ACCOUNT: Option<&'static str> = #account;
            pub mod types {
                use substreams_antelope::types::*;
                #(#type_aliases)*
                #(#types)*
            }
            pub mod actions {
                use substreams_antelope::types::*;
                use substreams_antelope::decoder::decode;
                #[allow(unused_imports)]
                use super::types::*;
                #(#actions)*
            }
        }
    }
}

impl From<ABI> for Contract {
    fn from(abi: ABI) -> Self {
        let mut types = abi.structs.into_iter().map(Type::from).collect::<Vec<_>>();
        let type_aliases = abi.types.into_iter().map(TypeAlias::from).collect::<Vec<_>>();

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

        Contract {
            types,
            type_aliases,
            actions,
        }
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
            c.generate(None),
            quote! {
                #[allow(dead_code)]
                pub const ACCOUNT: Option<& 'static str> = None;
                pub mod types {
                    use substreams_antelope::types::*;
                }
                pub mod actions {
                    use substreams_antelope::types::*;
                    use substreams_antelope::decoder::decode;
                    #[allow(unused_imports)]
                    use super::types::*;
                }
            },
        );
    }

    #[test]
    fn test_token() {
        let abi_contract = Contract { ..Default::default() };

        let c = Contract::from(abi_contract);

        assert_ast_eq(
            c.generate(None),
            quote! {
                #[allow(dead_code)]
                pub const ACCOUNT: Option<& 'static str> = None;
                pub mod types {
                    use substreams_antelope::types::*;
                }
                pub mod actions {
                    use substreams_antelope::types::*;
                    use substreams_antelope::decoder::decode;
                    #[allow(unused_imports)]
                    use super::types::*;
                }
            },
        );
    }

    #[test]
    fn test_wrapped() {
        let abi_contract = Contract { ..Default::default() };

        let c = Contract::from(abi_contract);

        assert_ast_eq(
            c.generate(Some("tokencontract".to_owned())),
            quote! {
                #[allow(dead_code)]
                pub const ACCOUNT: Option<&'static str> = Some("tokencontract");
                pub mod types {
                    use substreams_antelope::types::*;
                }
                pub mod actions {
                    use substreams_antelope::types::*;
                    use substreams_antelope::decoder::decode;
                    #[allow(unused_imports)]
                    use super::types::*;
                }
            },
        );
    }
}
