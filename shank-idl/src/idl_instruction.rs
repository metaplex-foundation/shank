use std::convert::TryFrom;

use anyhow::{anyhow, ensure, Error, Result};
use heck::MixedCase;
use serde::{Deserialize, Serialize};
use shank_macro_impl::instruction::{
    Instruction, InstructionAccount, InstructionVariant,
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
pub struct IdlInstruction {
    pub name: String,
    pub accounts: Vec<IdlAccountItem>,
    pub args: Vec<IdlField>,
    pub discriminant: IdlInstructionDiscriminant,
}

impl TryFrom<InstructionVariant> for IdlInstruction {
    type Error = Error;

    fn try_from(variant: InstructionVariant) -> Result<Self> {
        let InstructionVariant {
            ident,
            field_ty,
            accounts,
            discriminant,
        } = variant;

        let name = ident.to_string();
        let args: Vec<IdlField> = if let Some(field_ty) = field_ty {
            let name = if field_ty.kind.is_custom() {
                field_ty.ident.to_string().to_mixed_case()
            } else {
                "instructionArgs".to_string()
            };
            let ty = IdlType::try_from(field_ty)?;
            vec![IdlField {
                name,
                ty,
                attrs: None,
            }]
        } else {
            vec![]
        };

        let accounts = accounts.into_iter().map(IdlAccountItem::from).collect();
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
