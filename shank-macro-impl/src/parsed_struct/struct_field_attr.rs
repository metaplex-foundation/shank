use std::convert::TryFrom;

use crate::types::RustType;
use syn::{
    Attribute, Error as ParseError, Lit, Meta, NestedMeta,
    Result as ParseResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructFieldAttr {
    Padding,
    IdlType(RustType),
    IdlName(String),
    Skip,
}

impl From<&StructFieldAttr> for String {
    fn from(attr: &StructFieldAttr) -> Self {
        match attr {
            StructFieldAttr::Padding => "padding".to_string(),
            StructFieldAttr::IdlType(_) => "idl-type".to_string(),
            StructFieldAttr::IdlName(_) => "idl-name".to_string(),
            StructFieldAttr::Skip => "skip".to_string(),
        }
    }
}

pub struct StructFieldAttrs(pub Vec<StructFieldAttr>);

impl TryFrom<&[Attribute]> for StructFieldAttrs {
    type Error = ParseError;

    fn try_from(attrs: &[Attribute]) -> ParseResult<Self> {
        let mut result = Vec::new();

        for attr in attrs {
            if attr.path.is_ident("padding") {
                result.push(StructFieldAttr::Padding);
            } else if attr.path.is_ident("skip") {
                result.push(StructFieldAttr::Skip);
            } else if attr.path.is_ident("idl_name") {
                match attr.parse_meta() {
                    Ok(Meta::List(meta_list)) => {
                        if meta_list.nested.len() != 1 {
                            return Err(ParseError::new_spanned(
                                attr,
                                "idl_name attribute must have exactly one argument"
                            ));
                        }

                        if let Some(NestedMeta::Lit(Lit::Str(lit_str))) =
                            meta_list.nested.first()
                        {
                            result.push(StructFieldAttr::IdlName(
                                lit_str.value(),
                            ));
                        } else {
                            return Err(ParseError::new_spanned(
                                attr,
                                "idl_name attribute must be a string literal, e.g., #[idl_name(\"fieldName\")]"
                            ));
                        }
                    }
                    Ok(_) => {
                        return Err(ParseError::new_spanned(
                            attr,
                            "idl_name attribute must be a list with a string literal, e.g., #[idl_name(\"fieldName\")]"
                        ));
                    }
                    Err(err) => {
                        return Err(ParseError::new_spanned(
                            attr,
                            format!(
                                "Failed to parse idl_name attribute: {}",
                                err
                            ),
                        ));
                    }
                }
            } else if attr.path.is_ident("idl_type") {
                match attr.parse_meta() {
                    Ok(Meta::List(meta_list)) => {
                        let mut found_valid_type = false;

                        for nested in meta_list.nested.iter() {
                            let type_str = match nested {
                                // Handle string literal format: #[idl_type("TypeName")]
                                NestedMeta::Lit(Lit::Str(lit_str)) => {
                                    Some(lit_str.value())
                                }

                                // Handle direct type format: #[idl_type(TypeName)]
                                NestedMeta::Meta(meta) => {
                                    if let Some(ident) = meta.path().get_ident()
                                    {
                                        Some(ident.to_string())
                                    } else {
                                        // Handle path with segments (like std::string::String)
                                        Some(
                                            meta.path()
                                                .segments
                                                .iter()
                                                .map(|seg| {
                                                    seg.ident.to_string()
                                                })
                                                .collect::<Vec<_>>()
                                                .join("::"),
                                        )
                                    }
                                }
                                _ => {
                                    return Err(ParseError::new_spanned(
                                        nested,
                                        "Invalid nested meta in idl_type attribute"
                                    ));
                                }
                            };

                            if let Some(type_str) = type_str {
                                match RustType::try_from(type_str.as_str()) {
                                    Ok(rust_type) => {
                                        result.push(StructFieldAttr::IdlType(
                                            rust_type,
                                        ));
                                        found_valid_type = true;
                                        break;
                                    }
                                    Err(err) => {
                                        return Err(ParseError::new_spanned(
                                            nested,
                                            format!("Invalid type override format in idl_type attribute: {}", err)
                                        ));
                                    }
                                }
                            }
                        }

                        if !found_valid_type {
                            return Err(ParseError::new_spanned(
                                &meta_list.nested,
                                "No valid type found in idl_type attribute",
                            ));
                        }
                    }
                    Ok(_) => {
                        return Err(ParseError::new_spanned(
                            attr,
                            "idl_type attribute must be a list, e.g., #[idl_type(TypeName)] or #[idl_type(\"TypeName\")]"
                        ));
                    }
                    Err(err) => {
                        return Err(ParseError::new_spanned(
                            attr,
                            format!(
                                "Failed to parse idl_type attribute: {}",
                                err
                            ),
                        ));
                    }
                }
            }
        }

        Ok(Self(result))
    }
}
