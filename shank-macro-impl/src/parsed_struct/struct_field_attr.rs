use std::collections::HashSet;
use std::convert::TryFrom;

use crate::types::RustType;
use syn::{Attribute, Lit, Meta, NestedMeta};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StructFieldAttr {
    Padding,
    ShankAs(RustType),
}

impl From<&StructFieldAttr> for String {
    fn from(attr: &StructFieldAttr) -> Self {
        match attr {
            StructFieldAttr::Padding => "padding".to_string(),
            StructFieldAttr::ShankAs(_) => "shank-as".to_string(),
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
                    } else if attr.path.is_ident("shank") {
                        // Try to parse as Meta first for string literals
                        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                            for nested in meta_list.nested.iter() {
                                if let NestedMeta::Meta(Meta::NameValue(
                                    name_value,
                                )) = nested
                                {
                                    if name_value.path.is_ident("as") {
                                        if let Lit::Str(lit_str) =
                                            &name_value.lit
                                        {
                                            let type_str = lit_str.value();
                                            if let Ok(rust_type) =
                                                RustType::try_from(
                                                    type_str.as_str(),
                                                )
                                            {
                                                return Some(
                                                    StructFieldAttr::ShankAs(
                                                        rust_type,
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Try to handle direct type format
                        let tokens_str = attr.tokens.to_string();

                        // If we have a direct type reference
                        if tokens_str.contains("as = ") {
                            // Extract the type from the tokens
                            // Parse "(as = TypeName)" format
                            let parts: Vec<&str> = tokens_str
                                .trim_start_matches('(')
                                .trim_end_matches(')')
                                .split('=')
                                .collect();
                            if parts.len() == 2 {
                                let type_part = parts[1].trim();

                                if let Ok(rust_type) =
                                    RustType::try_from(type_part)
                                {
                                    return Some(StructFieldAttr::ShankAs(
                                        rust_type,
                                    ));
                                }
                            }
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
