use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
};
use syn::{Attribute, Error as ParseError, ItemEnum, Result as ParseResult};

use syn::Ident;

use crate::{
    parsed_enum::{ParsedEnum, ParsedEnumVariant},
    parsers::get_derive_attr,
    types::RustType,
    DERIVE_INSTRUCTION_ATTR,
};

use super::{
    account_attrs::InstructionAccount, IdlInstruction, InstructionStrategies,
    InstructionStrategy,
};

// -----------------
// Instruction
// -----------------
#[derive(Debug)]
pub struct Instruction {
    pub ident: Ident,
    pub variants: Vec<InstructionVariant>,
    pub discriminator_size: Option<u8>,
    pub discriminator_offset: Option<usize>,
}

impl Instruction {
    pub fn try_from_item_enum(
        item_enum: &ItemEnum,
        skip_derive_attr_check: bool,
    ) -> ParseResult<Option<Instruction>> {
        if skip_derive_attr_check
            || get_derive_attr(&item_enum.attrs, DERIVE_INSTRUCTION_ATTR)
                .is_some()
        {
            let parsed_enum = ParsedEnum::try_from(item_enum)?;
            Instruction::try_from(&parsed_enum).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<&ParsedEnum> for Option<Instruction> {
    type Error = ParseError;

    fn try_from(parsed_enum: &ParsedEnum) -> ParseResult<Self> {
        match get_derive_attr(&parsed_enum.attrs, DERIVE_INSTRUCTION_ATTR)
            .map(|_| parsed_enum)
        {
            Some(ix_enum) => ix_enum.try_into().map(Some),
            None => Ok(None),
        }
    }
}

impl TryFrom<&ParsedEnum> for Instruction {
    type Error = ParseError;

    fn try_from(parsed_enum: &ParsedEnum) -> ParseResult<Self> {
        let ParsedEnum {
            ident,
            variants,
            attrs,
            ..
        } = parsed_enum;

        // Parse discriminator size from attributes
        let discriminator_size = attrs
            .iter()
            .find(|attr| attr.path.is_ident("discriminator_size"))
            .and_then(|attr| {
                attr.parse_args::<syn::LitInt>()
                    .ok()
                    .and_then(|lit| lit.base10_parse::<u8>().ok())
            });

        // Parse discriminator offset from attributes
        let discriminator_offset = attrs
            .iter()
            .find(|attr| attr.path.is_ident("discriminator_offset"))
            .and_then(|attr| {
                attr.parse_args::<syn::LitInt>()
                    .ok()
                    .and_then(|lit| lit.base10_parse::<usize>().ok())
            });

        let variants = variants
            .iter()
            .map(|variant| {
                let mut variant = InstructionVariant::try_from(variant)?;
                if let Some(offset) = discriminator_offset {
                    variant.discriminant += offset;
                }
                Ok(variant)
            })
            .collect::<ParseResult<Vec<InstructionVariant>>>()?;

        println!("variants: {:?}", variants);
        println!("discriminator_size: {:?}", discriminator_size);
        println!("discriminator_offset: {:?}", discriminator_offset);

        Ok(Self {
            ident: ident.clone(),
            variants,
            discriminator_size,
            discriminator_offset,
        })
    }
}

#[derive(Debug)]
pub enum InstructionVariantFields {
    Unnamed(Vec<RustType>),
    Named(Vec<(String, RustType)>),
}

// -----------------
// Instruction Variant
// -----------------
#[derive(Debug)]
pub struct InstructionVariant {
    pub ident: Ident,
    pub field_tys: InstructionVariantFields,
    pub accounts: Vec<InstructionAccount>,
    pub strategies: HashSet<InstructionStrategy>,
    pub discriminant: usize,
    pub discriminant_size: Option<u8>,
}

impl TryFrom<&ParsedEnumVariant> for InstructionVariant {
    type Error = ParseError;

    fn try_from(variant: &ParsedEnumVariant) -> ParseResult<Self> {
        let ParsedEnumVariant {
            ident,
            fields,
            discriminant,
            attrs,
            ..
        } = variant;

        let mut field_tys: InstructionVariantFields = if !fields.is_empty() {
            // Determine if the InstructionType is tuple or struct variant
            let field = fields.first().unwrap();
            match &field.ident {
                Some(_) => InstructionVariantFields::Named(
                    fields
                        .iter()
                        .map(|x| {
                            (
                                x.ident.as_ref().unwrap().to_string(),
                                x.rust_type.clone(),
                            )
                        })
                        .collect(),
                ),
                None => InstructionVariantFields::Unnamed(
                    fields.iter().map(|x| x.rust_type.clone()).collect(),
                ),
            }
        } else {
            InstructionVariantFields::Unnamed(vec![])
        };

        let attrs: &[Attribute] = attrs.as_ref();
        let (accounts, strategies) = match IdlInstruction::try_from(attrs) {
            Ok(idl_ix) => {
                field_tys = idl_ix.to_instruction_fields(ident.clone());
                (
                    idl_ix.to_accounts(ident.clone()),
                    InstructionStrategies(HashSet::<InstructionStrategy>::new()),
                )
            }
            Err(_) => (attrs.try_into()?, attrs.into()),
        };

        let discriminant_size = attrs
            .iter()
            .find(|attr| attr.path.is_ident("discriminator_size"))
            .and_then(|attr| {
                attr.parse_args::<syn::LitInt>()
                    .ok()
                    .and_then(|lit| lit.base10_parse::<u8>().ok())
            });

        Ok(Self {
            ident: ident.clone(),
            field_tys,
            accounts: accounts.0,
            strategies: strategies.0,
            discriminant: *discriminant,
            discriminant_size,
        })
    }
}
