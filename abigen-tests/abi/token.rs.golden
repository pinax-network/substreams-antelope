#[allow(dead_code)]
pub const ACCOUNT: Option<&'static str> = None;
pub mod types {
    use substreams_antelope::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Account {
        pub balance: Asset,
    }
    impl std::str::FromStr for Account {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CurrencyStats {
        pub supply: Asset,
        pub max_supply: Asset,
        pub issuer: Name,
    }
    impl std::str::FromStr for CurrencyStats {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
}
pub mod actions {
    use substreams_antelope::types::*;
    use substreams_antelope::decoder::decode;
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Close {
        pub owner: Name,
        pub symbol: Symbol,
    }
    impl std::str::FromStr for Close {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Close {
        const NAME: &'static str = "close";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Create {
        pub issuer: Name,
        pub maximum_supply: Asset,
    }
    impl std::str::FromStr for Create {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Create {
        const NAME: &'static str = "create";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Issue {
        pub to: Name,
        pub quantity: Asset,
        pub memo: String,
    }
    impl std::str::FromStr for Issue {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Issue {
        const NAME: &'static str = "issue";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Open {
        pub owner: Name,
        pub symbol: Symbol,
        pub ram_payer: Name,
    }
    impl std::str::FromStr for Open {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Open {
        const NAME: &'static str = "open";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Retire {
        pub quantity: Asset,
        pub memo: String,
    }
    impl std::str::FromStr for Retire {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Retire {
        const NAME: &'static str = "retire";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Transfer {
        pub from: Name,
        pub to: Name,
        pub quantity: Asset,
        pub memo: String,
    }
    impl std::str::FromStr for Transfer {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Transfer {
        const NAME: &'static str = "transfer";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
}