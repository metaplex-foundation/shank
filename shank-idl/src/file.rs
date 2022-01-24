use anyhow::Result;
use std::path::Path;

use crate::idl::Idl;

// Parse an entire interface file.
pub fn parse(_filename: impl AsRef<Path>, _version: String) -> Result<Option<Idl>> {
    // https://github.com/project-serum/anchor/blob/51aeb08ae1c93f9f759c6e244e3fa724c9a916a7/lang/syn/src/idl/file.rs
    todo!()
}
