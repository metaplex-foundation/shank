use anyhow::{anyhow, Result};
use idl::Idl;
use manifest::Manifest;
use shank_macro_impl::custom_type::DetectCustomTypeConfig;

use std::path::PathBuf;

mod file;
pub mod idl;
mod idl_error_code;
pub mod idl_field;
pub mod idl_instruction;
pub mod idl_metadata;
pub mod idl_type;
pub mod idl_type_definition;
pub mod idl_variant;
pub mod manifest;

pub use file::*;

// -----------------
// ParseIdlOpts
// -----------------
pub struct ParseIdlOpts {
    pub detect_custom_struct: DetectCustomTypeConfig,
    pub require_program_address: bool,
    pub program_address_override: Option<String>,
}

impl Default for ParseIdlOpts {
    fn default() -> Self {
        Self {
            detect_custom_struct: Default::default(),
            require_program_address: true,
            program_address_override: None,
        }
    }
}

// -----------------
// extract_idl
// -----------------
pub fn extract_idl(file: &str, opts: ParseIdlOpts) -> Result<Option<Idl>> {
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
            detect_custom_struct: opts.detect_custom_struct,
            require_program_address: opts.require_program_address,
            program_address_override: opts.program_address_override,
        },
    )
}
