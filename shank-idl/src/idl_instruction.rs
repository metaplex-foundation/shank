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
        let discriminator_size = ix.discriminator_size;
        let instructions = ix
            .variants
            .into_iter()
            .map(|variant| {
                let mut idl_ix = IdlInstruction::try_from(variant)?;
                if let Some(size) = discriminator_size {
                    idl_ix.discriminant.ty = match size {
                        1 => IdlType::U8,
                        2 => IdlType::U16,
                        4 => IdlType::U32,
                        8 => IdlType::U64,
                        _ => return Err(anyhow!("Invalid discriminator size: {}. Must be 1, 2, 4, or 8", size)),
                    };
                }
                Ok(idl_ix)
            })
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
    pub legacy_optional_accounts_strategy: Option<bool>,
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
            discriminant_size,
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
        let legacy_optional_accounts_strategy = if strategies
            .contains(&InstructionStrategy::LegacyOptionalAccounts)
        {
            Some(true)
        } else {
            None
        };

        // ensure!(
        //     discriminant < ((u64::MAX as u128) + 1),
        //     anyhow!(
        //         "Instruction variant discriminants have to be <= u64::MAX ({}), \
        //             but the discriminant of variant '{}' is {}",
        //         u8::MAX,
        //         ident,
        //         discriminant
        //     )
        // );

        Ok(Self {
            name,
            accounts,
            args,
            legacy_optional_accounts_strategy,
            discriminant: IdlInstructionDiscriminant {
                ty: match discriminant_size {
                    Some(1) => IdlType::U8,
                    Some(2) => IdlType::U16,
                    Some(4) => IdlType::U32,
                    Some(8) => IdlType::U64,
                    _ => IdlType::U8,
                },
                value: discriminant as u64,
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlInstructionDiscriminant {
    #[serde(rename = "type")]
    pub ty: IdlType,
    pub value: u64,
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
    !x
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
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_optional_signer: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_optional: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub docs: Option<Vec<String>>,
}

impl From<InstructionAccount> for IdlAccount {
    fn from(acc: InstructionAccount) -> Self {
        let InstructionAccount {
            name,
            writable,
            signer,
            desc,
            optional,
            optional_signer,
            ..
        } = acc;
        Self {
            name: name.to_mixed_case(),
            is_mut: writable,
            is_signer: signer,
            docs: desc.map(|desc| vec![desc]),
            is_optional: optional,
            is_optional_signer: optional_signer,
        }
    }
}
