use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};
use shank_macro_impl::parsed_enum::{
    ParsedEnumVariant, ParsedEnumVariantField,
};

use crate::{idl_field::IdlField, idl_type::IdlType};
use anyhow::{Error, Result};

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

impl TryFrom<ParsedEnumVariantField> for IdlType {
    type Error = Error;
    fn try_from(field: ParsedEnumVariantField) -> Result<Self> {
        field.rust_type.try_into()
    }
}

impl TryFrom<ParsedEnumVariant> for IdlEnumVariant {
    type Error = Error;

    fn try_from(variant: ParsedEnumVariant) -> Result<Self> {
        let types = variant
            .fields
            .into_iter()
            .map(IdlType::try_from)
            .collect::<Result<Vec<IdlType>>>()?;

        let fields = if types.is_empty() {
            None
        } else {
            Some(EnumFields::Tuple(types))
        };

        Ok(Self {
            name: variant.ident.to_string(),
            fields,
        })
    }
}
