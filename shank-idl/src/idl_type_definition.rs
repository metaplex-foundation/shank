use std::convert::{TryFrom, TryInto};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use shank_macro_impl::{
    custom_type::{CustomEnum, CustomStruct},
    parsed_enum::ParsedEnum,
    parsed_struct::{ParsedStruct, StructAttr, StructAttrs},
};

use crate::{idl_field::IdlField, idl_variant::IdlEnumVariant};

fn extract_pod_sentinel(struct_attrs: &StructAttrs) -> Option<Vec<u8>> {
    struct_attrs.items_ref().iter().find_map(|attr| match attr {
        StructAttr::PodSentinel(sentinel) => Some(sentinel.clone()),
        _ => None,
    })
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
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "podSentinel"
    )]
    pub pod_sentinel: Option<Vec<u8>>,
}

impl TryFrom<ParsedStruct> for IdlTypeDefinition {
    type Error = Error;

    fn try_from(strct: ParsedStruct) -> Result<Self> {
        let name = strct.ident.to_string();
        let pod_sentinel = extract_pod_sentinel(&strct.struct_attrs);

        let ty: IdlTypeDefinitionTy = strct.try_into()?;
        Ok(Self {
            ty,
            name,
            pod_sentinel,
        })
    }
}

impl TryFrom<CustomStruct> for IdlTypeDefinition {
    type Error = Error;

    fn try_from(strct: CustomStruct) -> Result<Self> {
        let name = strct.ident.to_string();
        let pod_sentinel = extract_pod_sentinel(&strct.0.struct_attrs);

        let ty: IdlTypeDefinitionTy = strct.0.try_into()?;
        Ok(Self {
            ty,
            name,
            pod_sentinel,
        })
    }
}

impl TryFrom<CustomEnum> for IdlTypeDefinition {
    type Error = Error;

    fn try_from(enm: CustomEnum) -> Result<Self> {
        let name = enm.ident.to_string();
        let pod_sentinel = extract_pod_sentinel(&enm.0.struct_attrs);

        let ty: IdlTypeDefinitionTy = enm.0.try_into()?;
        Ok(Self {
            ty,
            name,
            pod_sentinel,
        })
    }
}
