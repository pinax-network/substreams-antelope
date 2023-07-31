use heck::ToUpperCamelCase;

pub fn abi_type_to_rust_type(tp: &str) -> Option<&str> {
    Some(match tp {
        "bool" => "Bool",
        "int8" => "Int8",
        "uint8" => "Uint8",
        "int16" => "Int16",
        "uint16" => "Uint16",
        "int32" => "Int32",
        "uint32" => "Uint32",
        "int64" => "Int64",
        "uint64" => "Uint64",
        "int128" => "Int128",
        "uint128" => "Uint128",
        "varint32" => "VarInt32",
        "varuint32" => "VarUint32",
        "float32" => "Float32",
        "float64" => "Float64",
        "float128" => "Float128",
        "time_point" => "TimePoint",
        "time_point_sec" => "TimePointSec",
        "block_timestamp_type" => "BlockTimeStampType",
        "name" => "Name",
        "&[u8]" => "Bytes",
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

pub fn custom_deserializer(ty: &str) -> Option<String> {
    let de = match ty {
        "uint64" => "substreams_antelope::decoder::str_or_u64".to_string(),
        "int64" => "substreams_antelope::decoder::str_or_i64".to_string(),
        _ => return None,
    };
    Some(de)
}

const RESERVED: [&str; 56] = [
    "as", "use", "break", "const", "continue", "crate", "else", "if", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "for",
    "let", "loop", "match", "mod", "move", "mut", "pub", "impl", "ref", "return", "Self", "self", "static", "struct", "super", "trait",
    "true", "type", "unsafe", "use", "where", "while", "abstract", "alignof", "become", "box", "do", "final", "macro", "offsetof",
    "override", "priv", "proc", "pure", "sizeof", "typeof", "unsized", "virtual", "yield",
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
