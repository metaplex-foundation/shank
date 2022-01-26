use anyhow::{anyhow, Result};
use idl::Idl;
use manifest::Manifest;
use shellexpand;

use std::path::PathBuf;

mod file;
pub mod idl;
mod idl_field;
mod idl_type;
mod idl_type_definition;
mod manifest;

pub fn extract_idl(file: &str) -> Result<Option<Idl>> {
    let file = shellexpand::tilde(file);
    let manifest_from_path =
        std::env::current_dir()?.join(PathBuf::from(&*file).parent().unwrap());
    let cargo = Manifest::discover_from_path(manifest_from_path)?
        .ok_or_else(|| anyhow!("Cargo.toml not found"))?;
    file::parse(&*file, cargo.version())
}
