use heck::ToUpperCamelCase;

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

pub fn rust_type(ty: &str) -> String {
    match abi_type_to_rust_type(ty) {
        Some(ty) => ty.to_string(),
        _ => ty.to_string().to_upper_camel_case(),
    }
}
