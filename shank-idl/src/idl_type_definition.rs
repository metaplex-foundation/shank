use std::convert::{TryFrom, TryInto};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use shank_macro_impl::account::AccountStruct;

use crate::{idl_field::IdlField, idl_type::IdlType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnumFields {
    Named(Vec<IdlField>),
    Tuple(Vec<IdlType>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlEnumVariant {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fields: Option<EnumFields>,
}

// -----------------
// IdlTypeDefinitionTy
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum IdlTypeDefinitionTy {
    Struct { fields: Vec<IdlField> },
    Enum { variants: Vec<IdlEnumVariant> },
}

impl TryFrom<AccountStruct> for IdlTypeDefinitionTy {
    type Error = Error;

    fn try_from(strct: AccountStruct) -> Result<Self> {
        let mut fields = Vec::new();
        for f in strct.fields {
            let idl_field: IdlField = f.try_into()?;
            fields.push(idl_field);
        }
        Ok(Self::Struct { fields })
    }
}

// -----------------
// IdlTypeDefinition
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlTypeDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlTypeDefinitionTy,
}

impl TryFrom<AccountStruct> for IdlTypeDefinition {
    type Error = Error;

    fn try_from(strct: AccountStruct) -> Result<Self> {
        let name = strct.ident.to_string();
        let ty: IdlTypeDefinitionTy = strct.try_into()?;
        Ok(Self { ty, name })
    }
}
