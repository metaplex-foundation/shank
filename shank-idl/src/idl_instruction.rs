use std::convert::TryFrom;

use anyhow::{anyhow, ensure, Error, Result};
use heck::MixedCase;
use serde::{Deserialize, Serialize};
use shank_macro_impl::instruction::{
    Instruction, InstructionAccount, InstructionStrategy, InstructionVariant,
    InstructionVariantFields,
};

use crate::{idl_field::IdlField, idl_type::IdlType};

// -----------------
// IdlInstructions
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstructions(pub Vec<IdlInstruction>);

impl TryFrom<Instruction> for IdlInstructions {
    type Error = Error;

    fn try_from(ix: Instruction) -> Result<Self, Self::Error> {
        let instructions = ix
            .variants
            .into_iter()
            .map(IdlInstruction::try_from)
            .collect::<Result<Vec<IdlInstruction>>>()?;
        Ok(Self(instructions))
    }
}

// -----------------
// IdlInstruction
// -----------------
/// This represents one Instruction which in the case of ShankInstruction is just
/// one variant of that enum.
/// We also expect it to only have one arg which is a custom type containing the
/// respective instruction args.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlInstruction {
    pub name: String,
    pub accounts: Vec<IdlAccountItem>,
    pub args: Vec<IdlField>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_optional_accounts: Option<bool>,
    pub discriminant: IdlInstructionDiscriminant,
}

impl TryFrom<InstructionVariant> for IdlInstruction {
    type Error = Error;

    fn try_from(variant: InstructionVariant) -> Result<Self> {
        let InstructionVariant {
            ident,
            field_tys,
            accounts,
            strategies,
            discriminant,
        } = variant;

        let name = ident.to_string();
        let parsed_idl_fields: Result<Vec<IdlField>, Error> = match field_tys {
            InstructionVariantFields::Named(args) => {
                let mut parsed: Vec<IdlField> = vec![];
                for (field_name, field_ty) in args.iter() {
                    let ty = IdlType::try_from(field_ty.clone())?;
                    parsed.push(IdlField {
                        name: field_name.to_mixed_case(),
                        ty,
                        attrs: None,
                    })
                }
                Ok(parsed)
            }
            InstructionVariantFields::Unnamed(args) => {
                let mut parsed: Vec<IdlField> = vec![];
                for (index, field_ty) in args.iter().enumerate() {
                    let name = if args.len() == 1 {
                        if field_ty.kind.is_custom() {
                            field_ty.ident.to_string().to_mixed_case()
                        } else {
                            "args".to_string()
                        }
                    } else {
                        format!("arg{}", index).to_string()
                    };
                    let ty = IdlType::try_from(field_ty.clone())?;
                    parsed.push(IdlField {
                        name,
                        ty,
                        attrs: None,
                    })
                }
                Ok(parsed)
            }
        };
        let args: Vec<IdlField> = parsed_idl_fields?;

        let accounts = accounts.into_iter().map(IdlAccountItem::from).collect();
        let default_optional_accounts = if strategies
            .contains(&InstructionStrategy::DefaultOptionalAccounts)
        {
            Some(true)
        } else {
            None
        };

        ensure!(
            discriminant < u8::MAX as usize,
            anyhow!(
                "Instruction variant discriminants have to be <= u8::MAX ({}), \
                    but the discriminant of variant '{}' is {}",
                u8::MAX,
                ident,
                discriminant
            )
        );

        Ok(Self {
            name,
            accounts,
            args,
            default_optional_accounts,
            discriminant: (discriminant as u8).into(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstructionDiscriminant {
    #[serde(rename = "type")]
    pub ty: IdlType,
    pub value: u8,
}

impl From<u8> for IdlInstructionDiscriminant {
    fn from(value: u8) -> Self {
        Self {
            ty: IdlType::U8,
            value,
        }
    }
}

// -----------------
// IdlAccounts
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccounts {
    pub name: String,
    pub accounts: Vec<IdlAccountItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IdlAccountItem {
    IdlAccount(IdlAccount),
    IdlAccounts(IdlAccounts),
}

impl From<InstructionAccount> for IdlAccountItem {
    fn from(account: InstructionAccount) -> Self {
        IdlAccountItem::IdlAccount(account.into())
    }
}

fn is_false(x: &bool) -> bool {
    return !x;
}
// -----------------
// IdlAccount
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub optional: bool,
}

impl From<InstructionAccount> for IdlAccount {
    fn from(acc: InstructionAccount) -> Self {
        let InstructionAccount {
            name,
            writable,
            signer,
            desc,
            optional,
            ..
        } = acc;
        Self {
            name: name.to_mixed_case(),
            is_mut: writable,
            is_signer: signer,
            desc,
            optional,
        }
    }
}
