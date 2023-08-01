use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::types::*;

pub fn decode<T: de::DeserializeOwned>(json_str: &str) -> Result<T, crate::errors::Error> {
    serde_json::from_str::<T>(json_str).map_err(|_| crate::errors::Error::JsonDecodeError)
}

pub fn str_or_u64<'de, D>(deserializer: D) -> Result<Uint64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Deserialize::deserialize(deserializer)? {
        Value::String(v) => v
            .parse::<Uint64>()
            .map_err(|_| de::Error::custom("failed to parse u64 number"))?,
        Value::Number(v) => v.as_u64().ok_or(de::Error::custom("failed to get u64 number"))?,
        _ => return Err(de::Error::custom("Invalid u64 number type")),
    })
}

pub fn str_or_i64<'de, D>(deserializer: D) -> Result<Int64, D::Error>
where
    D: Deserializer<'de>,
{

    Ok(match Deserialize::deserialize(deserializer)? {
        Value::String(v) => v
            .parse::<Int64>()
            .map_err(|_| de::Error::custom("failed to parse i64 number"))?,
        Value::Number(v) => v.as_i64().ok_or(de::Error::custom("failed to get i64 number"))?,
        _ => return Err(de::Error::custom("Invalid u64 number type")),
    })
}

pub fn vec_str_or_u64<'de, D>(deserializer: D) -> Result<Vec<Uint64>, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        Value::Array(values) => {
            values
                .into_iter()
                .map(|strnum| match strnum {
                    Value::String(str) => str.parse::<Uint64>().map_err(|_| de::Error::custom(format!("Failed to parse strnum: {}", str))),
                    Value::Number(num) => num.as_u64().ok_or(de::Error::custom(format!("Failed to convert strnum to u64: {}", num))),
                    _ => Err(de::Error::custom("Invalid strnum type")),
                })
                .collect()
        }
        _ => Err(de::Error::custom("Invalid array")),
    }
}

pub fn vec_str_or_i64<'de, D>(deserializer: D) -> Result<Vec<Int64>, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        Value::Array(values) => {
            values
                .into_iter()
                .map(|strnum| match strnum {
                    Value::String(str) => str.parse::<Int64>().map_err(|_| de::Error::custom(format!("Failed to parse strnum: {}", str))),
                    Value::Number(num) => num.as_i64().ok_or(de::Error::custom(format!("Failed to convert strnum to i64: {}", num))),
                    _ => Err(de::Error::custom("Invalid strnum type")),
                })
                .collect()
        }
        _ => Err(de::Error::custom("Invalid array")),
    }
}