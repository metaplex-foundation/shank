use anyhow::{anyhow, Result};
use idl::Idl;
use manifest::Manifest;
use shellexpand;

use std::path::PathBuf;

mod file;
pub mod idl;
mod idl_field;
mod idl_instruction;
mod idl_type;
mod idl_type_definition;
mod idl_variant;
pub mod manifest;

pub use file::*;

pub fn extract_idl(file: &str) -> Result<Option<Idl>> {
    let file = shellexpand::tilde(file);
    let manifest_from_path =
        std::env::current_dir()?.join(PathBuf::from(&*file).parent().unwrap());
    let cargo = Manifest::discover_from_path(manifest_from_path)?
        .ok_or_else(|| anyhow!("Cargo.toml not found"))?;
    let program_name = cargo
        .lib_name()
        .map_err(|err| anyhow!("Cargo.toml is missing lib name. {}", err))?;
    file::parse_file(
        &*file,
        &ParseIdlConfig {
            program_name,
            program_version: cargo.version(),
            ..Default::default()
        },
    )
}
