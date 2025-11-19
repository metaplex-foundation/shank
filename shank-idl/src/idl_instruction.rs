use std::{collections::HashMap, convert::TryFrom};

use anyhow::{anyhow, ensure, Error, Result};
use heck::MixedCase;
use serde::{Deserialize, Serialize};
use shank_macro_impl::instruction::{
    AccountsSource, Instruction, InstructionAccount, InstructionStrategy,
    InstructionVariant, InstructionVariantFields,
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

impl IdlInstructions {
    pub fn try_into_with_accounts(
        ix: Instruction,
        accounts_map: &HashMap<String, Vec<InstructionAccount>>,
    ) -> Result<Self> {
        let instructions = ix
            .variants
            .into_iter()
            .map(|variant| {
                IdlInstruction::try_from_with_accounts(variant, accounts_map)
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
            accounts_source,
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

        // For the basic try_from, we still create placeholders for struct references
        let accounts = match accounts_source {
            AccountsSource::Inline(_) => {
                // Use the accounts directly from the instruction attributes
                accounts.into_iter().map(IdlAccountItem::from).collect()
            }
            AccountsSource::Struct(struct_path) => {
                // Create placeholder - this will be resolved in try_from_with_accounts
                let struct_name = struct_path
                    .segments
                    .last()
                    .map(|seg| seg.ident.to_string())
                    .unwrap_or_else(|| "UnknownStruct".to_string());

                vec![IdlAccountItem::IdlAccount(IdlAccount {
                    name: format!("accountsStruct{}", struct_name),
                    is_mut: false,
                    is_signer: false,
                    is_optional: false,
                    is_optional_signer: false,
                    docs: Some(vec![format!(
                        "Accounts defined by struct: {}",
                        struct_name
                    )]),
                })]
            }
        };
        let legacy_optional_accounts_strategy = if strategies
            .contains(&InstructionStrategy::LegacyOptionalAccounts)
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
            legacy_optional_accounts_strategy,
            discriminant: (discriminant as u8).into(),
        })
    }
}

impl IdlInstruction {
    pub fn try_from_with_accounts(
        variant: InstructionVariant,
        accounts_map: &HashMap<String, Vec<InstructionAccount>>,
    ) -> Result<Self> {
        let InstructionVariant {
            ident,
            field_tys,
            accounts,
            accounts_source,
            strategies,
            discriminant,
        } = variant;

        let name = ident.to_string();

        // Parse instruction arguments (same as regular try_from)
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

        // Handle different account sources - THIS IS THE KEY IMPROVEMENT
        let accounts = match accounts_source {
            AccountsSource::Inline(_) => {
                // Use the accounts directly from the instruction attributes
                accounts.into_iter().map(IdlAccountItem::from).collect()
            }
            AccountsSource::Struct(struct_path) => {
                // Resolve the struct reference to individual accounts
                let struct_name = struct_path
                    .segments
                    .last()
                    .map(|seg| seg.ident.to_string())
                    .unwrap_or_else(|| "UnknownStruct".to_string());

                if let Some(struct_accounts) = accounts_map.get(&struct_name) {
                    // Found the struct - expand its accounts
                    struct_accounts
                        .iter()
                        .map(|acc| IdlAccountItem::from(acc.clone()))
                        .collect()
                } else {
                    // Struct not found - fall back to placeholder
                    vec![IdlAccountItem::IdlAccount(IdlAccount {
                        name: format!("accountsStruct{}", struct_name),
                        is_mut: false,
                        is_signer: false,
                        is_optional: false,
                        is_optional_signer: false,
                        docs: Some(vec![format!(
                            "Accounts defined by struct: {} (not resolved)",
                            struct_name
                        )]),
                    })]
                }
            }
        };

        let legacy_optional_accounts_strategy = if strategies
            .contains(&InstructionStrategy::LegacyOptionalAccounts)
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
            legacy_optional_accounts_strategy,
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
