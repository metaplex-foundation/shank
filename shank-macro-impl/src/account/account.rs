use std::convert::{TryFrom, TryInto};

use anyhow::Result;
use syn::{Field, Ident};

use crate::types::RustType;

#[derive(Debug)]
pub struct UnprocessedAccountStruct {
    pub ident: Ident,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct StructField {
    pub ident: syn::Ident,
    pub rust_type: RustType,
}

impl TryFrom<&Field> for StructField {
    type Error = anyhow::Error;

    fn try_from(f: &Field) -> Result<Self, Self::Error> {
        let ident = f.ident.as_ref().unwrap().clone();
        let rust_type: RustType = (&f.ty).try_into()?;
        Ok(Self { ident, rust_type })
    }
}

#[derive(Debug)]
pub struct AccountStruct {
    pub ident: Ident,
    pub fields: Vec<StructField>,
}

impl TryFrom<UnprocessedAccountStruct> for AccountStruct {
    type Error = anyhow::Error;

    fn try_from(strct: UnprocessedAccountStruct) -> Result<Self, Self::Error> {
        let fields: Vec<StructField> = strct
            .fields
            .into_iter()
            .flat_map(|f| StructField::try_from(&f))
            .collect();

        Ok(AccountStruct {
            ident: strct.ident,
            fields,
        })
    }
}
