use std::convert::TryFrom;

use anyhow::{Error, Result};
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
    pub discriminant: usize,
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
            let name = (&field_ty).ident.to_string();
            let ty = IdlType::try_from(field_ty)?;
            vec![IdlField { name, ty }]
        } else {
            vec![]
        };

        let accounts = accounts.into_iter().map(IdlAccountItem::from).collect();

        Ok(Self {
            name,
            accounts,
            args,
            discriminant,
        })
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
}

impl From<InstructionAccount> for IdlAccount {
    fn from(acc: InstructionAccount) -> Self {
        let InstructionAccount {
            name,
            writable,
            signer,
            desc,
            ..
        } = acc;
        Self {
            name,
            is_mut: writable,
            is_signer: signer,
            desc,
        }
    }
}
