use std::convert::{TryFrom, TryInto};
use syn::{Attribute, Error as ParseError, ItemEnum, Result as ParseResult};

use syn::Ident;

use crate::{
    parsed_enum::{ParsedEnum, ParsedEnumVariant},
    parsers::get_derive_attr,
    types::RustType,
    DERIVE_INSTRUCTION_ATTR,
};

use super::account_attrs::{InstructionAccount, InstructionAccounts};

// -----------------
// Instruction
// -----------------
#[derive(Debug)]
pub struct Instruction {
    pub ident: Ident,
    pub variants: Vec<InstructionVariant>,
}

impl Instruction {
    pub fn try_from_item_enum(
        item_enum: &ItemEnum,
    ) -> ParseResult<Option<Instruction>> {
        match get_derive_attr(&item_enum.attrs, DERIVE_INSTRUCTION_ATTR)
            .map(|_| item_enum)
        {
            Some(ix_enum) => {
                let parsed_enum: ParsedEnum = ix_enum.try_into()?;
                (&parsed_enum).try_into().map(Some)
            }
            None => Ok(None),
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
            ident, variants, ..
        } = parsed_enum;

        let variants = variants
            .iter()
            .map(InstructionVariant::try_from)
            .collect::<ParseResult<Vec<InstructionVariant>>>()?;
        Ok(Self {
            ident: ident.clone(),
            variants,
        })
    }
}

// -----------------
// Instruction Variant
// -----------------
#[derive(Debug)]
pub struct InstructionVariant {
    pub ident: Ident,
    pub field_ty: Option<RustType>,
    pub accounts: Vec<InstructionAccount>,
    pub discriminant: usize,
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

        if fields.len() > 1 {
            return Err(ParseError::new_spanned(
                fields.get(1).map(|x| &x.rust_type.ident),
                "An Instruction can only have one arg field",
            ));
        }
        let field_ty = fields.first().map(|x| x.rust_type.clone());
        let attrs: &[Attribute] = attrs.as_ref();
        let accounts: InstructionAccounts = attrs.try_into()?;

        Ok(Self {
            ident: ident.clone(),
            field_ty,
            accounts: accounts.0,
            discriminant: *discriminant,
        })
    }
}
