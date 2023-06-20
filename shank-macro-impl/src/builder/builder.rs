use std::convert::{TryFrom, TryInto};
use syn::{Attribute, Error as ParseError, ItemEnum, Result as ParseResult};

use syn::Ident;

use crate::instruction::InstructionVariantFields;
use crate::parsed_enum::ParsedEnum;
use crate::parsers::get_derive_attr;
use crate::DERIVE_BUILDER_ATTR;
use crate::{
    instruction::{InstructionAccount, InstructionAccounts},
    parsed_enum::ParsedEnumVariant,
};

use super::{BuilderArgument, BuilderArguments};

// -----------------
// Instruction
// -----------------
#[derive(Debug)]
pub struct Builder {
    pub ident: Ident,
    pub variants: Vec<BuilderVariant>,
}

impl Builder {
    pub fn try_from_item_enum(
        item_enum: &ItemEnum,
        skip_derive_attr_check: bool,
    ) -> ParseResult<Option<Builder>> {
        if skip_derive_attr_check
            || get_derive_attr(&item_enum.attrs, DERIVE_BUILDER_ATTR).is_some()
        {
            let parsed_enum = ParsedEnum::try_from(item_enum)?;
            Builder::try_from(&parsed_enum).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<&ParsedEnum> for Option<Builder> {
    type Error = ParseError;

    fn try_from(parsed_enum: &ParsedEnum) -> ParseResult<Self> {
        match get_derive_attr(&parsed_enum.attrs, DERIVE_BUILDER_ATTR)
            .map(|_| parsed_enum)
        {
            Some(builder_enum) => builder_enum.try_into().map(Some),
            None => Ok(None),
        }
    }
}

impl TryFrom<&ParsedEnum> for Builder {
    type Error = ParseError;

    fn try_from(parsed_enum: &ParsedEnum) -> ParseResult<Self> {
        let ParsedEnum {
            ident, variants, ..
        } = parsed_enum;

        let variants = variants
            .iter()
            .map(BuilderVariant::try_from)
            .collect::<ParseResult<Vec<BuilderVariant>>>()?;
        Ok(Self {
            ident: ident.clone(),
            variants,
        })
    }
}

// -----------------
// Builder Variant
// -----------------
#[derive(Debug)]
pub struct BuilderVariant {
    pub ident: Ident,
    pub field_tys: InstructionVariantFields,
    pub accounts: Vec<InstructionAccount>,
    pub arguments: Vec<BuilderArgument>,
    pub discriminant: usize,
}

impl TryFrom<&ParsedEnumVariant> for BuilderVariant {
    type Error = ParseError;

    fn try_from(variant: &ParsedEnumVariant) -> ParseResult<Self> {
        let ParsedEnumVariant {
            ident,
            fields,
            discriminant,
            attrs,
            ..
        } = variant;

        let field_tys: InstructionVariantFields = if !fields.is_empty() {
            // Determine if the InstructionType is tuple or struct variant
            let field = fields.get(0).unwrap();
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
        let accounts: InstructionAccounts = attrs.try_into()?;
        let arguments: BuilderArguments = attrs.try_into()?;

        Ok(Self {
            ident: ident.clone(),
            field_tys,
            accounts: accounts.0,
            arguments: arguments.0,
            discriminant: *discriminant,
        })
    }
}
