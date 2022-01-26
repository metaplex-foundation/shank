use anyhow::Result;
use std::{convert::TryInto, path::Path};

use crate::{
    idl::{Idl, IdlConst, IdlErrorCode, IdlEvent, IdlInstruction, IdlState},
    idl_type_definition::IdlTypeDefinition,
};
use shank_macro_impl::{account::extract_account_structs, krate::CrateContext};

// Parse an entire interface file.
pub fn parse_file(
    filename: impl AsRef<Path>,
    version: String,
) -> Result<Option<Idl>> {
    let ctx = CrateContext::parse(filename)?;

    let constants = constants(&ctx)?;
    let instructions = instructions(&ctx)?;
    let state = state(&ctx)?;
    let accounts = accounts(&ctx)?;
    let types = types(&ctx)?;
    let events = events(&ctx)?;
    let errors = errors(&ctx)?;

    let idl = Idl {
        version,
        name: "TODO: program name".to_string(),
        constants,
        instructions,
        state,
        accounts,
        types,
        events,
        errors,
        metadata: None,
    };

    Ok(Some(idl))
}

fn accounts(ctx: &CrateContext) -> Result<Vec<IdlTypeDefinition>> {
    let account_structs = extract_account_structs(ctx.structs())?;

    let mut accounts: Vec<IdlTypeDefinition> = Vec::new();
    for strct in account_structs {
        let idl_def: IdlTypeDefinition = strct.try_into()?;
        accounts.push(idl_def);
    }
    Ok(accounts)
}
fn constants(_ctx: &CrateContext) -> Result<Vec<IdlConst>> {
    // TODO(thlorenz): Implement
    let constants: Vec<IdlConst> = Vec::new();
    Ok(constants)
}

fn instructions(_ctx: &CrateContext) -> Result<Vec<IdlInstruction>> {
    // TODO(thlorenz): Implement
    let instructions: Vec<IdlInstruction> = Vec::new();
    Ok(instructions)
}

fn state(_ctx: &CrateContext) -> Result<Option<IdlState>> {
    // TODO(thlorenz): Implement
    Ok(None)
}

fn types(_ctx: &CrateContext) -> Result<Vec<IdlTypeDefinition>> {
    // TODO(thlorenz): Implement
    let types: Vec<IdlTypeDefinition> = Vec::new();
    Ok(types)
}

fn events(_ctx: &CrateContext) -> Result<Option<Vec<IdlEvent>>> {
    // TODO(thlorenz): Implement
    Ok(None)
}

fn errors(_ctx: &CrateContext) -> Result<Option<Vec<IdlErrorCode>>> {
    // TODO(thlorenz): Implement
    Ok(None)
}
