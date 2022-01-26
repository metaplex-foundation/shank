use anyhow::Result;
use std::path::Path;

use crate::{idl::Idl, idl_type_definition::IdlTypeDefinition};
use shank_macro_impl::{account::extract_account_structs, krate::CrateContext};

// Parse an entire interface file.
pub fn parse(
    filename: impl AsRef<Path>,
    _version: String,
) -> Result<Option<Idl>> {
    let ctx = CrateContext::parse(filename)?;
    let account_structs = extract_account_structs(ctx.structs());

    Ok(None)
}

fn accounts(ctx: &CrateContext) -> Result<Vec<IdlTypeDefinition>> {
    let account_structs = extract_account_structs(ctx.structs());

    let accounts: Vec<IdlTypeDefinition> = Vec::new();
    for strct in account_structs {}
    todo!()
}
