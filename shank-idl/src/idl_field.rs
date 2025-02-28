use heck::MixedCase;
use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};
use shank_macro_impl::parsed_struct::StructField;

use crate::idl_type::IdlType;
use anyhow::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Vec<String>>,
}

impl TryFrom<StructField> for IdlField {
    type Error = Error;
    fn try_from(field: StructField) -> Result<Self> {
        let ty: IdlType = if let Some(override_type) = field.type_override() {
            override_type.clone().try_into()?
        } else {
            field.rust_type.try_into()?
        };

        let attrs = field
            .attrs
            .iter()
            .map(Into::<String>::into)
            .collect::<Vec<String>>();

        let attrs = if attrs.is_empty() { None } else { Some(attrs) };
        Ok(Self {
            name: field.ident.to_string().to_mixed_case(),
            ty,
            attrs,
        })
    }
}
