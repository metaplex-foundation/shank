use std::collections::HashSet;
use std::convert::TryFrom;

use crate::types::RustType;
use syn::Attribute;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StructFieldAttr {
    Padding,
    IdlType(RustType),
}

impl From<&StructFieldAttr> for String {
    fn from(attr: &StructFieldAttr) -> Self {
        match attr {
            StructFieldAttr::Padding => "padding".to_string(),
            StructFieldAttr::IdlType(_) => "idl-type".to_string(),
        }
    }
}

pub struct StructFieldAttrs(pub HashSet<StructFieldAttr>);

impl From<&[Attribute]> for StructFieldAttrs {
    fn from(attrs: &[Attribute]) -> Self {
        Self(
            attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path.is_ident("padding") {
                        Some(StructFieldAttr::Padding)
                    } else if attr.path.is_ident("idl_type") {
                        // Try to parse the content inside the parentheses
                        let tokens_str = attr.tokens.to_string();

                        // Extract the type from the tokens
                        // Parse "(TypeName)" format
                        let type_part = tokens_str
                            .trim_start_matches('(')
                            .trim_end_matches(')');

                        if let Ok(rust_type) = RustType::try_from(type_part) {
                            return Some(StructFieldAttr::IdlType(rust_type));
                        }

                        None
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}
