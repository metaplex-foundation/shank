use std::convert::{TryFrom, TryInto};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use shank_macro_impl::{
    custom_type::{CustomEnum, CustomStruct},
    parsed_enum::ParsedEnum,
    parsed_struct::ParsedStruct,
};

use crate::{idl_field::IdlField, idl_variant::IdlEnumVariant};

// -----------------
// IdlTypeDefinitionTy
// -----------------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum IdlTypeDefinitionTy {
    Struct { fields: Vec<IdlField> },
    Enum { variants: Vec<IdlEnumVariant> },
}

impl TryFrom<ParsedStruct> for IdlTypeDefinitionTy {
    type Error = Error;

    fn try_from(strct: ParsedStruct) -> Result<Self> {
        let fields = strct
            .fields
            .into_iter()
            .map(IdlField::try_from)
            .collect::<Result<Vec<IdlField>>>()?;

        Ok(Self::Struct { fields })
    }
}

impl TryFrom<ParsedEnum> for IdlTypeDefinitionTy {
    type Error = Error;

    fn try_from(enm: ParsedEnum) -> Result<Self> {
        let variants = enm
            .variants
            .into_iter()
            .map(IdlEnumVariant::try_from)
            .collect::<Result<Vec<IdlEnumVariant>>>()?;

        Ok(Self::Enum { variants })
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

impl TryFrom<ParsedStruct> for IdlTypeDefinition {
    type Error = Error;

    fn try_from(strct: ParsedStruct) -> Result<Self> {
        let name = strct.ident.to_string();
        let ty: IdlTypeDefinitionTy = strct.try_into()?;
        Ok(Self { ty, name })
    }
}

impl TryFrom<CustomStruct> for IdlTypeDefinition {
    type Error = Error;

    fn try_from(strct: CustomStruct) -> Result<Self> {
        let name = strct.ident.to_string();
        let ty: IdlTypeDefinitionTy = strct.0.try_into()?;
        Ok(Self { ty, name })
    }
}

impl TryFrom<CustomEnum> for IdlTypeDefinition {
    type Error = Error;

    fn try_from(enm: CustomEnum) -> Result<Self> {
        let name = enm.ident.to_string();
        let ty: IdlTypeDefinitionTy = enm.0.try_into()?;
        Ok(Self { ty, name })
    }
}
