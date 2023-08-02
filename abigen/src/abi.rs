use serde::{Deserialize, Serialize};

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
    #[serde(default)]
    pub variants: Vec<ABIVariant>,
    #[serde(default)]
    pub abi_extensions: Vec<String>,
    #[serde(default)]
    pub error_messages: Vec<ABIErrorMessage>,
    #[serde(default)]
    pub ricardian_clauses: Vec<ABIRicardianClause>,
    #[serde(default)]
    pub action_results: Vec<ABIActionResult>,
}

impl TryFrom<&str> for ABI {
    type Error = serde_json::Error;
    #[inline]
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(str)
    }
}
