use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

use syn::{
    parse::{Parse, ParseStream},
    Error as ParseError, Field, Ident, ItemStruct, Result as ParseResult,
};

use crate::types::RustType;

#[derive(Debug, Clone)]
pub struct StructField {
    pub ident: syn::Ident,
    pub rust_type: RustType,
}

impl Display for StructField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{key}: {ty} ({kind:?})",
            key = self.ident,
            ty = self.rust_type.ident,
            kind = self.rust_type.kind
        )
    }
}

impl TryFrom<&Field> for StructField {
    type Error = ParseError;

    fn try_from(f: &Field) -> ParseResult<Self> {
        let ident = f.ident.as_ref().unwrap().clone();
        let rust_type: RustType = match (&f.ty).try_into() {
            Ok(ty) => ty,
            Err(err) => {
                return Err(ParseError::new_spanned(ident, err.to_string()))
            }
        };
        Ok(Self { ident, rust_type })
    }
}

#[derive(Debug)]
pub struct AccountStruct {
    pub ident: Ident,
    pub fields: Vec<StructField>,
}

impl Parse for AccountStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let strct = <ItemStruct as Parse>::parse(input)?;
        parse_account_item_struct(&strct)
    }
}

pub fn parse_account_item_struct(
    item: &ItemStruct,
) -> ParseResult<AccountStruct> {
    let fields = match &item.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(StructField::try_from)
            .collect::<ParseResult<Vec<StructField>>>()?,
        _ => {
            return Err(ParseError::new_spanned(
                &item.fields,
                "failed to parse fields make sure they are all named",
            ))
        }
    };
    Ok(AccountStruct {
        ident: item.ident.clone(),
        fields,
    })
}
