use proc_macro2::{Span, TokenStream};
use quote::quote;

use super::abi::ABIType;
use super::rust::{custom_deserializer, is_reserved, rust_type};

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub is_optional: bool,
    pub is_array: bool,
}

impl From<ABIType> for Field {
    fn from(abi_type: ABIType) -> Self {
        Field {
            name: abi_type.name.to_string(),
            ty: abi_type.ty.trim_end_matches('?').trim_end_matches("[]").to_string(),
            is_optional: abi_type.ty.ends_with('?'),
            is_array: abi_type.ty.ends_with("[]"),
        }
    }
}

impl Field {
    pub fn generate(&self) -> TokenStream {
        let (ident, is_reserved) = if is_reserved(self.name.as_str()) {
            (syn::Ident::new(&format!("{}_", self.name.as_str()), Span::call_site()), true)
        } else {
            (syn::Ident::new(self.name.as_str(), Span::call_site()), false)
        };
        let deserializer = match custom_deserializer(self.ty.as_str()) {
            Some(de) => quote! { #[serde(deserialize_with = #de)] },
            None => quote! {},
        };
        let ty_ident = syn::Ident::new(rust_type(self.ty.as_str()).as_str(), Span::call_site());
        let ty = match self.is_array {
            true => quote!{ Vec<#ty_ident> },
            false => quote!{  #ty_ident },
        };

        let rename = if is_reserved {
            let ty = self.name.as_str();
            quote! { #[serde(rename = #ty)] }
        } else {
            quote! {}
        };
        quote! { #deserializer #rename pub #ident: #ty}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use crate::abigen::abi::ABIType;

    #[test]
    fn test_all_fields() {
        let tests = [
            ("bool", quote! { Bool }, quote!{ }),
            ("int8", quote! { Int8 }, quote!{ }),
            ("uint8", quote! { Uint8 }, quote!{ }),
            ("int16", quote! { Int16 }, quote!{ }),
            ("uint16", quote! { Uint16 }, quote!{ }),
            ("int32", quote! { Int32 }, quote!{ }),
            ("uint32", quote! { Uint32 }, quote!{ }),
            ("int64", quote! { Int64 }, quote!{ #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")] }),
            ("uint64", quote! { Uint64 }, quote!{ #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")] }),
            ("int128", quote! { Int128 }, quote!{ }),
            ("uint128", quote! { Uint128 }, quote!{ }),
            ("varint32", quote! { VarInt32 }, quote!{ }),
            ("varuint32", quote! { VarUint32 }, quote!{ }),
            ("float32", quote! { Float32 }, quote!{ }),
            ("float64", quote! { Float64 }, quote!{ }),
            ("float128", quote! { Float128 }, quote!{ }),
            ("time_point", quote! { TimePoint }, quote!{ }),
            ("time_point_sec", quote! { TimePointSec }, quote!{ }),
            ("block_timestamp_type", quote! { BlockTimeStampType }, quote!{ }),
            ("name", quote! { Name }, quote!{ }),
            ("&[u8]", quote! { Bytes }, quote!{ }),
            ("string", quote! { String }, quote!{ }),
            ("checksum160", quote! { Checksum160 }, quote!{ }),
            ("checksum256", quote! { Checksum256 }, quote!{ }),
            ("checksum512", quote! { Checksum512 }, quote!{ }),
            ("public_key", quote! { PublicKey }, quote!{ }),
            ("signature", quote! { Signature }, quote!{ }),
            ("symbol", quote! { Symbol }, quote!{ }),
            ("symbol_code", quote! { SymbolCode }, quote!{ }),
            ("asset", quote! { Asset }, quote!{ }),
            ("extended_asset", quote! { ExtendedAsset }, quote!{ }),
            ("name[]", quote! { Vec<Name> }, quote!{ }),
            ("uint64[]", quote! { Vec<Uint64> }, quote!{ #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")] }),
        ];

        for (abi_type, rs_type, deserializer) in &tests {
            let abi_def = ABIType {
                name: "some_field".to_string(),
                ty: abi_type.to_string(),
            };
            let field = Field::from(abi_def);

            let expected_tokens = quote! { #deserializer pub some_field: #rs_type };

            assert_eq!(field.generate().to_string(), expected_tokens.to_string());
        }
    }

    #[test]
    fn test_field_with_reserved_name() {
        let abi_type = ABIType {
            name: "type".to_string(),
            ty: "name".to_string(),
        };
        let field = Field::from(abi_type);
        let generated_tokens = field.generate();
        let expected_tokens = quote! { #[serde(rename = "type")] pub type_: Name };

        assert_eq!(generated_tokens.to_string(), expected_tokens.to_string());
    }

}