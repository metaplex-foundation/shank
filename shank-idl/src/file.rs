use anyhow::Result;

use std::{
    convert::{TryFrom, TryInto},
    path::Path,
};

use crate::{
    idl::{Idl, IdlConst, IdlEvent, IdlState},
    idl_error_code::IdlErrorCode,
    idl_instruction::{IdlInstruction, IdlInstructions},
    idl_metadata::IdlMetadata,
    idl_type_definition::IdlTypeDefinition,
};
use shank_macro_impl::{
    account::extract_account_structs,
    converters::parse_error_into,
    custom_type::{CustomEnum, CustomStruct, DetectCustomTypeConfig},
    error::extract_this_errors,
    instruction::extract_instruction_enums,
    krate::CrateContext,
    macros::ProgramId,
};

// -----------------
// ParseIdlConfig
// -----------------
#[derive(Debug)]
pub struct ParseIdlConfig {
    pub program_version: String,
    pub program_name: String,
    pub detect_custom_struct: DetectCustomTypeConfig,
    pub require_program_address: bool,
}

impl Default for ParseIdlConfig {
    fn default() -> Self {
        Self {
            program_version: Default::default(),
            program_name: Default::default(),
            detect_custom_struct: Default::default(),
            require_program_address: true,
        }
    }
}

impl ParseIdlConfig {
    pub fn optional_program_address() -> Self {
        Self {
            require_program_address: false,
            ..Self::default()
        }
    }
}

// -----------------
// Parse File
// -----------------

/// Parse an entire interface file.
pub fn parse_file(
    filename: impl AsRef<Path>,
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
    let metadata = metadata(&ctx, config.require_program_address)?;

    let idl = Idl {
        version: config.program_version.to_string(),
        name: config.program_name.to_string(),
        constants,
        instructions,
        state,
        accounts,
        types,
        events,
        errors,
        metadata,
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
    let instruction_enums =
        extract_instruction_enums(ctx.enums()).map_err(parse_error_into)?;

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
    detect_custom_type: &DetectCustomTypeConfig,
) -> Result<Vec<IdlTypeDefinition>> {
    let custom_structs = ctx
        .structs()
        .filter(|x| detect_custom_type.are_custom_type_attrs(&x.attrs))
        .map(|x| CustomStruct::try_from(x).map_err(parse_error_into))
        .collect::<Result<Vec<CustomStruct>>>()?;

    let custom_enums = ctx
        .enums()
        .filter(|x| detect_custom_type.are_custom_type_attrs(&x.attrs))
        .map(|x| CustomEnum::try_from(x).map_err(parse_error_into))
        .collect::<Result<Vec<CustomEnum>>>()?;

    let types = custom_structs
        .into_iter()
        .map(IdlTypeDefinition::try_from)
        .chain(custom_enums.into_iter().map(IdlTypeDefinition::try_from))
        .collect::<Result<Vec<IdlTypeDefinition>>>()?;

    Ok(types)
}

fn metadata(
    ctx: &CrateContext,
    require_program_address: bool,
) -> Result<IdlMetadata> {
    let macros: Vec<_> = ctx.macros().cloned().collect();
    let address = match ProgramId::try_from(&macros[..]) {
        Ok(ProgramId { id }) => Ok(Some(id)),
        Err(err) if require_program_address => Err(err),
        Err(_) => Ok(None),
    }?;
    Ok(IdlMetadata {
        origin: "shank".to_string(),
        address,
    })
}

fn events(_ctx: &CrateContext) -> Result<Option<Vec<IdlEvent>>> {
    // TODO(thlorenz): Implement
    Ok(None)
}

fn errors(ctx: &CrateContext) -> Result<Option<Vec<IdlErrorCode>>> {
    let program_errors = extract_this_errors(ctx.enums())?;
    if program_errors.is_empty() {
        Ok(None)
    } else {
        let error_codes = program_errors
            .into_iter()
            .map(IdlErrorCode::from)
            .collect::<Vec<IdlErrorCode>>();
        Ok(Some(error_codes))
    }
}
