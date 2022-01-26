use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};
use shank_macro_impl::account::StructField;

use crate::idl_type::IdlType;
use anyhow::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdlField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: IdlType,
}

impl TryFrom<StructField> for IdlField {
    type Error = Error;
    fn try_from(field: StructField) -> Result<Self> {
        let ty: IdlType = field.rust_type.try_into()?;
        Ok(Self {
            name: field.ident.to_string(),
            ty,
        })
    }
}
