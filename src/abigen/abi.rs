use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;

use serde::{
    // de::DeserializeOwned,
    // de::Deserializer,
    Deserialize,
    Serialize,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]

pub struct ABIType {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIStruct {
    pub name: String,
    pub base: String,
    pub fields: Vec<ABIType>,
}

///
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIAction {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub ricardian_contract: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABITable {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    index_type: String,
    key_names: Vec<String>,
    key_types: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIVariant {
    name: String,
    // #[serde(deserialize_with = "string_or_seq_string")]
    types: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABITypes {
    new_type_name: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIRicardianClause {
    id: String,
    body: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIActionResult {
    name: String,
    result_type: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABIErrorMessage {
    error_code: u64,
    error_msg: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ABI {
    pub version: String,
    pub types: Vec<ABITypes>,
    pub structs: Vec<ABIStruct>,
    pub actions: Vec<ABIAction>,
    pub tables: Vec<ABITable>,
    pub variants: Vec<ABIVariant>,
    pub abi_extensions: Vec<String>,
    pub error_messages: Vec<ABIErrorMessage>,
    pub ricardian_clauses: Vec<ABIRicardianClause>,
    pub action_results: Vec<ABIActionResult>,
}

pub fn rust_type_to_abi_type(tp: &str) -> &str {
    match tp {
        "bool" => "bool",
        "i8" => "int8",
        "u8" => "uint8",
        "i16" => "int16",
        "u16" => "uint16",
        "i32" => "int32",
        "u32" => "uint32",
        "i64" => "int64",
        "u64" => "uint64",
        "i128" => "int128",
        "u128" => "uint128",
        "Varint32" => "varint32",
        "VarUint32" => "varuint32",
        "f32" => "float32",
        "f64" => "float64",
        "Float128" => "float128",
        "TimePoint" => "time_point",
        "TimePointSec" => "time_point_sec",
        "BlockTimeStampType" => "block_timestamp_type",
        "Name" => "name",
        "&[u8]" => "bytes",
        "String" => "string",
        "Checksum160" => "checksum160",
        "Checksum256" => "checksum256",
        "Uint256" => "checksum256",
        "Checksum512" => "checksum512",
        "PublicKey" => "public_key",
        "Signature" => "signature",
        "Symbol" => "symbol",
        "SymbolCode" => "symbol_code",
        "Asset" => "asset",
        "ExtendedAsset" => "extended_asset",
        _ => panic!("rust_type_to_abi_type(): unknown type {}", tp),
    }
}

pub fn abi_type_to_rust_type(tp: &str) -> Option<&str> {
    Some(match tp {
        "bool" => "bool",
        "int8" => "i8",
        "uint8" => "u8",
        "int16" => "i16",
        "uint16" => "u16",
        "int32" => "i32",
        "uint32" => "u32",
        "int64" => "i64",
        "uint64" => "u64",
        "int128" => "i128",
        "uint128" => "u128",
        "varint32" => "Varint32",
        "varuint32" => "VarUint32",
        "float32" => "f32",
        "float64" => "f64",
        "float128" => "Float128",
        "time_point" => "TimePoint",
        "time_point_sec" => "TimePointSec",
        "block_timestamp_type" => "BlockTimeStampType",
        "name" => "Name",
        "&[u8]" => "bytes",
        "string" => "String",
        "checksum160" => "Checksum160",
        "checksum256" => "Checksum256",
        "checksum512" => "Checksum512",
        "public_key" => "PublicKey",
        "signature" => "Signature",
        "symbol" => "Symbol",
        "symbol_code" => "SymbolCode",
        "asset" => "Asset",
        "extended_asset" => "ExtendedAsset",
        _ => return None,
    })
}

pub const BUILTIN_TYPES: [&str; 31] = [
    "bool",
    "i8",
    "u8",
    "i16",
    "u16",
    "i32",
    "u32",
    "i64",
    "u64",
    "f32",
    "f64",
    "i128",
    "u128",
    "String",
    "Varint32",
    "VarUint32",
    "Float128",
    "TimePoint",
    "TimePointSec",
    "BlockTimeStampType",
    "Name",
    "Checksum160",
    "Checksum256",
    "Uint256",
    "Checksum512",
    "PublicKey",
    "Signature",
    "Symbol",
    "SymbolCode",
    "Asset",
    "ExtendedAsset",
];

pub fn is_builtin_type(name: &str) -> bool {
    BUILTIN_TYPES.contains(&name)
}

pub fn custom_deserializer(ty: &str) -> Option<String> {
    let de = match ty {
        "uint64" => "substreams_antelope::decoder::str_or_u64".to_string(),
        "int64" => "substreams_antelope::decoder::str_or_i64".to_string(),
        _ => return None,
    };
    Some(de)
}

impl TryFrom<&str> for ABI {
    type Error = serde_json::Error;
    #[inline]
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(str)
    }
}

const RESERVED: [&str; 58] = [
    "as",
    "use",
    "extern crate",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "if",
    "if let",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "for",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "impl",
    "ref",
    "return",
    "Self",
    "self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "abstract",
    "alignof",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "offsetof",
    "override",
    "priv",
    "proc",
    "pure",
    "sizeof",
    "typeof",
    "unsized",
    "virtual",
    "yield",
];

pub fn is_reserved(id: &str) -> bool {
    RESERVED.contains(&id)
}

pub fn rust_type(ty: &str) -> TokenStream {
    if abi_type_to_rust_type(ty).is_none() {
        let ty = ty.to_string().to_upper_camel_case();
        let ty = syn::Ident::new(ty.as_str(), Span::call_site());
        return quote! { super::types::#ty };
    }
    match ty {
        "bool" => quote! { substreams_antelope::types::Bool },
        "int8" => quote! { substreams_antelope::types::Int8 },
        "uint8" => quote! { substreams_antelope::types::Uint8 },
        "int16" => quote! { substreams_antelope::types::Int16 },
        "uint16" => quote! { substreams_antelope::types::Uint16 },
        "int32" => quote! { substreams_antelope::types::Int32 },
        "uint32" => quote! { substreams_antelope::types::Uint32 },
        "int64" => quote! { substreams_antelope::types::Int64 },
        "uint64" => quote! { substreams_antelope::types::Uint64 },
        "int128" => quote! { substreams_antelope::types::Int128 },
        "uint128" => quote! { substreams_antelope::types::Uint128 },
        "varint32" => quote! { substreams_antelope::types::VarInt32 },
        "varuint32" => quote! { substreams_antelope::types::VarUint32 },
        "float32" => quote! { substreams_antelope::types::Float32 },
        "float64" => quote! { substreams_antelope::types::Float64 },
        "float128" => quote! { substreams_antelope::types::Float128 },
        "time_point" => quote! { substreams_antelope::types::TimePoint },
        "time_point_sec" => quote! { substreams_antelope::types::TimePointSec },
        "block_timestamp_type" => quote! { substreams_antelope::types::BlockTimeStampType },
        "name" => quote! { substreams_antelope::types::Name },
        "bytes" => quote! { substreams_antelope::types::Bytes },
        "string" => quote! { substreams_antelope::types::String },
        "checksum160" => quote! { substreams_antelope::types::Checksum160 },
        "checksum256" => quote! { substreams_antelope::types::Checksum256 },
        "checksum512" => quote! { substreams_antelope::types::Checksum512 },
        "public_key" => quote! { substreams_antelope::types::PublicKey },
        "signature" => quote! { substreams_antelope::types::Signature },
        "symbol" => quote! { substreams_antelope::types::Symbol },
        "symbol_code" => quote! { substreams_antelope::types::SymbolCode },
        "asset" => quote! { substreams_antelope::types::Asset },
        "extended_asset" => quote! { substreams_antelope::types::ExtendedAsset },

        _ => panic!("rust_type(): unknown type: {}", ty),
    }
}
