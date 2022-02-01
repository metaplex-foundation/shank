use anyhow::{format_err, Result};

use std::{
    convert::{TryFrom, TryInto},
    path::Path,
};

use crate::{
    idl::{Idl, IdlConst, IdlErrorCode, IdlEvent, IdlState},
    idl_instruction::{IdlInstruction, IdlInstructions},
    idl_type_definition::IdlTypeDefinition,
};
use shank_macro_impl::{
    account::extract_account_structs,
    custom_type::{CustomStruct, DetectCustomStructConfig},
    instruction::extract_instruction_enums,
    krate::CrateContext,
};

// -----------------
// ParseIdlConfig
// -----------------
#[derive(Default, Debug)]
pub struct ParseIdlConfig {
    detect_custom_struct: DetectCustomStructConfig,
}

// -----------------
// Parse File
// -----------------

/// Parse an entire interface file.
pub fn parse_file(
    filename: impl AsRef<Path>,
    version: String,
    config: &ParseIdlConfig,
) -> Result<Option<Idl>> {
    let ctx = CrateContext::parse(filename)?;

    let constants = constants(&ctx)?;
    let instructions = instructions(&ctx)?;
    let state = state(&ctx)?;
    let accounts = accounts(&ctx)?;
    let types = types(&ctx, &config.detect_custom_struct)?;
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

fn instructions(ctx: &CrateContext) -> Result<Vec<IdlInstruction>> {
    let instruction_enums = extract_instruction_enums(ctx.enums())?;
    let mut instructions: Vec<IdlInstruction> = Vec::new();
    // TODO(thlorenz): Should we enforce only one Instruction Enum Arg?
    // TODO(thlorenz): Should unfold that only arg?
    // TODO(thlorenz): Better way to combine those if we don't to the above.
    for ix in instruction_enums {
        let idl_instructions: IdlInstructions = ix.try_into()?;
        for ix in idl_instructions.0 {
            instructions.push(ix);
        }
    }
    Ok(instructions)
}

fn constants(_ctx: &CrateContext) -> Result<Vec<IdlConst>> {
    // TODO(thlorenz): Implement
    let constants: Vec<IdlConst> = Vec::new();
    Ok(constants)
}

fn state(_ctx: &CrateContext) -> Result<Option<IdlState>> {
    // TODO(thlorenz): Implement
    Ok(None)
}

fn types(
    ctx: &CrateContext,
    detect_custom_struct: &DetectCustomStructConfig,
) -> Result<Vec<IdlTypeDefinition>> {
    let custom_structs = ctx
        .structs()
        .filter(|x| {
            CustomStruct::are_custom_struct_attrs(
                &x.attrs,
                detect_custom_struct,
            )
        })
        .map(|x| {
            CustomStruct::try_from(x).map_err(|err| format_err!("{}", err))
        })
        .collect::<Result<Vec<CustomStruct>>>()?;

    let types = custom_structs
        .into_iter()
        .map(IdlTypeDefinition::try_from)
        .collect::<Result<Vec<IdlTypeDefinition>>>()?;

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
