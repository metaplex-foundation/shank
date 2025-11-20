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
    pub program_address_override: Option<String>,
}

impl Default for ParseIdlConfig {
    fn default() -> Self {
        Self {
            program_version: Default::default(),
            program_name: Default::default(),
            detect_custom_struct: Default::default(),
            require_program_address: true,
            program_address_override: None,
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
    let metadata = metadata(
        &ctx,
        config.require_program_address,
        config.program_address_override.as_ref(),
    )?;

    let mut idl = Idl {
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

    // Populate sentinel values for PodOption<CustomType> fields from type definitions
    populate_pod_option_sentinels(&mut idl)?;

    // Validate that custom types used in PodOption have pod_sentinel defined
    validate_pod_option_sentinels(&idl)?;

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
    // TODO(thlorenz): Better way to combine those if we don't do the above.

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
    program_address_override: Option<&String>,
) -> Result<IdlMetadata> {
    let macros: Vec<_> = ctx.macros().cloned().collect();
    let address = if let Some(program_address) = program_address_override {
        Ok(Some(program_address.clone()))
    } else {
        match ProgramId::try_from(&macros[..]) {
            Ok(ProgramId { id }) => Ok(Some(id)),
            Err(err) if require_program_address => Err(err),
            Err(_) => Ok(None),
        }
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

fn populate_pod_option_sentinels(idl: &mut Idl) -> Result<()> {
    use crate::idl_type::IdlType;
    use std::collections::HashMap;

    // Build a map of custom type names to their podSentinel (owned data)
    let type_sentinels: HashMap<String, Vec<u8>> = idl
        .types
        .iter()
        .filter_map(|type_def| {
            type_def.pod_sentinel.as_ref().map(|sentinel| {
                (type_def.name.clone(), sentinel.clone())
            })
        })
        .collect();

    // Helper function to populate sentinel in FixedSizeOption types
    fn populate_type(
        ty: &mut IdlType,
        type_sentinels: &HashMap<String, Vec<u8>>,
    ) {
        match ty {
            IdlType::FixedSizeOption { inner, sentinel } => {
                if let IdlType::Defined(type_name) = inner.as_ref() {
                    // If sentinel is not already set, populate it from the type definition
                    if sentinel.is_none() {
                        if let Some(type_sentinel) = type_sentinels.get(type_name) {
                            *sentinel = Some(type_sentinel.clone());
                        }
                    }
                }
            }
            // Recursively populate nested types
            IdlType::Vec(inner) | IdlType::Option(inner) | IdlType::HashSet(inner) | IdlType::BTreeSet(inner) => {
                populate_type(inner, type_sentinels);
            }
            IdlType::Array(inner, _) => {
                populate_type(inner, type_sentinels);
            }
            IdlType::HashMap(key, val) | IdlType::BTreeMap(key, val) => {
                populate_type(key, type_sentinels);
                populate_type(val, type_sentinels);
            }
            IdlType::Tuple(types) => {
                for t in types {
                    populate_type(t, type_sentinels);
                }
            }
            _ => {}
        }
    }

    // Populate sentinels in all account fields
    for account in &mut idl.accounts {
        match &mut account.ty {
            crate::idl_type_definition::IdlTypeDefinitionTy::Struct { fields } => {
                for field in fields {
                    populate_type(&mut field.ty, &type_sentinels);
                }
            }
            crate::idl_type_definition::IdlTypeDefinitionTy::Enum { variants } => {
                for variant in variants {
                    if let Some(fields) = &mut variant.fields {
                        match fields {
                            crate::idl_variant::EnumFields::Named(named_fields) => {
                                for field in named_fields {
                                    populate_type(&mut field.ty, &type_sentinels);
                                }
                            }
                            crate::idl_variant::EnumFields::Tuple(tuple_types) => {
                                for ty in tuple_types {
                                    populate_type(ty, &type_sentinels);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Populate sentinels in all custom type fields
    for type_def in &mut idl.types {
        match &mut type_def.ty {
            crate::idl_type_definition::IdlTypeDefinitionTy::Struct { fields } => {
                for field in fields {
                    populate_type(&mut field.ty, &type_sentinels);
                }
            }
            crate::idl_type_definition::IdlTypeDefinitionTy::Enum { variants } => {
                for variant in variants {
                    if let Some(fields) = &mut variant.fields {
                        match fields {
                            crate::idl_variant::EnumFields::Named(named_fields) => {
                                for field in named_fields {
                                    populate_type(&mut field.ty, &type_sentinels);
                                }
                            }
                            crate::idl_variant::EnumFields::Tuple(tuple_types) => {
                                for ty in tuple_types {
                                    populate_type(ty, &type_sentinels);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Populate sentinels in instruction arguments
    for instruction in &mut idl.instructions {
        for arg in &mut instruction.args {
            populate_type(&mut arg.ty, &type_sentinels);
        }
    }

    Ok(())
}

fn validate_pod_option_sentinels(idl: &Idl) -> Result<()> {
    use crate::idl_type::IdlType;

    // Helper function to recursively check for PodOption with missing sentinel
    fn check_type(
        ty: &IdlType,
        errors: &mut Vec<String>,
    ) {
        match ty {
            IdlType::FixedSizeOption { inner, sentinel } => {
                if let IdlType::Defined(type_name) = inner.as_ref() {
                    // This is PodOption<CustomType>
                    // After population, sentinel should be present
                    if sentinel.is_none() {
                        errors.push(format!(
                            "Type '{}' is used in PodOption but does not define #[pod_sentinel(...)]. \
                             Custom types used with PodOption must specify a sentinel value.",
                            type_name
                        ));
                    }
                }
            }
            // Recursively check nested types
            IdlType::Vec(inner) | IdlType::Option(inner) | IdlType::HashSet(inner) | IdlType::BTreeSet(inner) => {
                check_type(inner, errors);
            }
            IdlType::Array(inner, _) => {
                check_type(inner, errors);
            }
            IdlType::HashMap(key, val) | IdlType::BTreeMap(key, val) => {
                check_type(key, errors);
                check_type(val, errors);
            }
            IdlType::Tuple(types) => {
                for t in types {
                    check_type(t, errors);
                }
            }
            _ => {}
        }
    }

    let mut errors = Vec::new();

    // Check all account fields
    for account in &idl.accounts {
        match &account.ty {
            crate::idl_type_definition::IdlTypeDefinitionTy::Struct { fields } => {
                for field in fields {
                    check_type(&field.ty, &mut errors);
                }
            }
            crate::idl_type_definition::IdlTypeDefinitionTy::Enum { variants } => {
                for variant in variants {
                    if let Some(fields) = &variant.fields {
                        match fields {
                            crate::idl_variant::EnumFields::Named(named_fields) => {
                                for field in named_fields {
                                    check_type(&field.ty, &mut errors);
                                }
                            }
                            crate::idl_variant::EnumFields::Tuple(tuple_types) => {
                                for ty in tuple_types {
                                    check_type(ty, &mut errors);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Check all custom type fields
    for type_def in &idl.types {
        match &type_def.ty {
            crate::idl_type_definition::IdlTypeDefinitionTy::Struct { fields } => {
                for field in fields {
                    check_type(&field.ty, &mut errors);
                }
            }
            crate::idl_type_definition::IdlTypeDefinitionTy::Enum { variants } => {
                for variant in variants {
                    if let Some(fields) = &variant.fields {
                        match fields {
                            crate::idl_variant::EnumFields::Named(named_fields) => {
                                for field in named_fields {
                                    check_type(&field.ty, &mut errors);
                                }
                            }
                            crate::idl_variant::EnumFields::Tuple(tuple_types) => {
                                for ty in tuple_types {
                                    check_type(ty, &mut errors);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Check instruction arguments
    for instruction in &idl.instructions {
        for arg in &instruction.args {
            check_type(&arg.ty, &mut errors);
        }
    }

    if !errors.is_empty() {
        anyhow::bail!("PodOption validation errors:\n  - {}", errors.join("\n  - "));
    }

    Ok(())
}
