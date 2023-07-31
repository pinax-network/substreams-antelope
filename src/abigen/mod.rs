pub mod abi;
pub mod action;
pub mod build;
pub mod contract;
pub mod ty;

use crate::errors::Error;
use anyhow::format_err;
use std::{
    env, fs,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use self::abi::ABI;

pub fn generate_abi_code<S: AsRef<str>>(path: S) -> Result<proc_macro2::TokenStream, anyhow::Error> {
    let normalized_path = normalize_path(path.as_ref())?;
    let source_file = fs::File::open(normalized_path).map_err(|_| Error::AbiLoadError)?;
    let mut reader = BufReader::new(source_file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let contract = ABI::try_from(contents.as_str())?;
    let c = contract::Contract::from(contract);
    Ok(c.generate())
}

pub fn normalize_path<S: AsRef<Path>>(relative_path: S) -> Result<PathBuf, anyhow::Error> {
    // workaround for https://github.com/rust-lang/rust/issues/43860
    let cargo_toml_directory = env::var("CARGO_MANIFEST_DIR").map_err(|_| format_err!("Cannot find manifest file"))?;
    let mut path: PathBuf = cargo_toml_directory.into();
    path.push(relative_path);
    Ok(path)
}
