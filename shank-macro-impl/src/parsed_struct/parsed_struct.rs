use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

use syn::{
    parse::{Parse, ParseStream},
    Attribute, Error as ParseError, Field, Ident, ItemStruct,
    Result as ParseResult,
};

use crate::{parsed_struct::struct_attr::StructAttrs, types::RustType};

use super::struct_field_attr::{StructFieldAttr, StructFieldAttrs};

#[derive(Debug, Clone)]
pub struct StructField {
    pub ident: syn::Ident,
    pub rust_type: RustType,
    pub attrs: Vec<StructFieldAttr>,
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

impl StructField {
    /// Get the overridden type from the IdlType attribute if present
    pub fn type_override(&self) -> Option<&RustType> {
        self.attrs.iter().find_map(|attr| {
            if let StructFieldAttr::IdlType(rust_type) = attr {
                Some(rust_type)
            } else {
                None
            }
        })
    }

    /// Get the overridden name from the IdlName attribute if present
    pub fn name_override(&self) -> Option<&String> {
        self.attrs.iter().find_map(|attr| {
            if let StructFieldAttr::IdlName(name) = attr {
                Some(name)
            } else {
                None
            }
        })
    }
}

impl TryFrom<&Field> for StructField {
    type Error = ParseError;

    fn try_from(f: &Field) -> ParseResult<Self> {
        let ident = f.ident.as_ref().unwrap().clone();
        let attrs = match StructFieldAttrs::try_from(f.attrs.as_ref()) {
            Ok(field_attrs) => field_attrs.0,
            Err(err) => {
                return Err(ParseError::new_spanned(
                    &f.ident,
                    format!("Failed to parse field attributes: {}", err),
                ));
            }
        };
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
    pub struct_attrs: StructAttrs,
}

impl Parse for ParsedStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let strct = <ItemStruct as Parse>::parse(input)?;
        ParsedStruct::try_from(&strct)
    }
}

/// Helper function to check if a field has the skip attribute
fn field_has_skip_attr(field: &Field) -> bool {
    field.attrs.iter().any(|attr| attr.path.is_ident("skip"))
}

impl TryFrom<&ItemStruct> for ParsedStruct {
    type Error = ParseError;

    fn try_from(item: &ItemStruct) -> ParseResult<Self> {
        let fields = match &item.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                // Filter out fields with #[skip] attribute before trying to parse them
                .filter(|f| !field_has_skip_attr(f))
                .map(StructField::try_from)
                .collect::<ParseResult<Vec<StructField>>>()?,
            _ => {
                return Err(ParseError::new_spanned(
                    &item.fields,
                    "failed to parse fields make sure they are all named",
                ))
            }
        };
        let struct_attrs = StructAttrs::try_from(item.attrs.as_slice())?;
        Ok(ParsedStruct {
            ident: item.ident.clone(),
            fields,
            attrs: item.attrs.clone(),
            struct_attrs,
        })
    }
}
