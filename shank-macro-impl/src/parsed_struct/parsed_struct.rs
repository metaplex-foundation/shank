use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    fmt::Display,
};

use syn::{
    parse::{Parse, ParseStream},
    Attribute, Error as ParseError, Field, Ident, ItemStruct,
    Result as ParseResult,
};

use crate::types::RustType;

use super::struct_field_attr::{StructFieldAttr, StructFieldAttrs};

#[derive(Debug, Clone)]
pub struct StructField {
    pub ident: syn::Ident,
    pub rust_type: RustType,
    pub attrs: HashSet<StructFieldAttr>,
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
        let attrs = StructFieldAttrs::from(f.attrs.as_ref()).0;
        let rust_type: RustType = match (&f.ty).try_into() {
            Ok(ty) => ty,
            Err(err) => {
                return Err(ParseError::new_spanned(ident, err.to_string()))
            }
        };

        Ok(Self {
            ident,
            rust_type,
            attrs,
        })
    }
}

#[derive(Debug)]
pub struct ParsedStruct {
    pub ident: Ident,
    pub fields: Vec<StructField>,
    pub attrs: Vec<Attribute>,
}

impl Parse for ParsedStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let strct = <ItemStruct as Parse>::parse(input)?;
        ParsedStruct::try_from(&strct)
    }
}

impl TryFrom<&ItemStruct> for ParsedStruct {
    type Error = ParseError;

    fn try_from(item: &ItemStruct) -> ParseResult<Self> {
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
        Ok(ParsedStruct {
            ident: item.ident.clone(),
            fields,
            attrs: item.attrs.clone(),
        })
    }
}
