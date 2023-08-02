pub type Asset = std::string::String;
pub type Name = std::string::String;
pub type Bool = std::string::String;
pub type String = std::string::String;
pub type Bytes = std::string::String;
pub type Checksum160 = std::string::String;
pub type Checksum256 = std::string::String;
pub type Checksum512 = std::string::String;
pub type PublicKey = std::string::String;
pub type Signature = std::string::String;
pub type Symbol = std::string::String;
pub type SymbolCode = std::string::String;
pub type TimePointSec = std::string::String;
pub type TimePoint = std::string::String;
pub type BlockTimestampType = std::string::String;
pub type Uint8 = u8;
pub type Uint16 = u16;
pub type Uint32 = u32;
pub type VarUint32 = u32;
pub type Uint64 = u64;
pub type Uint128 = std::string::String;
pub type Int8 = i8;
pub type Int16 = i16;
pub type Int32 = i32;
pub type VarInt32 = u32;
pub type Int64 = i64;
pub type Int128 = i128;
pub type Float32 = std::string::String;
pub type Float64 = std::string::String;
pub type Float128 = std::string::String;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ExtendedAsset {
    pub quantity: Asset,
    pub contract: Name,
}
