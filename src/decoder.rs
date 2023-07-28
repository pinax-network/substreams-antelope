use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

pub fn decode<T: DeserializeOwned>(json_str: &str) -> Result<T, crate::errors::Error> {
    serde_json::from_str::<T>(json_str)
        .map_err(|_| crate::errors::Error::JsonDecodeError)
}

pub fn str_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU64<'a> {
        Str(&'a str),
        U64(u64),
    }

    Ok(match StrOrU64::deserialize(deserializer)? {
        StrOrU64::Str(v) => v
            .parse::<u64>()
            .map_err(|_| serde::de::Error::custom("failed to parse u64 number"))?,
        StrOrU64::U64(v) => v,
    })
}

pub fn str_or_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrI64<'a> {
        Str(&'a str),
        I64(i64),
    }

    Ok(match StrOrI64::deserialize(deserializer)? {
        StrOrI64::Str(v) => v
            .parse::<i64>()
            .map_err(|_| serde::de::Error::custom("failed to parse i64 number"))?,
        StrOrI64::I64(v) => v,
    })
}