use std::collections::HashSet;
use std::convert::TryFrom;

use crate::types::RustType;
use syn::{Attribute, Lit, Meta, NestedMeta};

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
                        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                            for nested in meta_list.nested.iter() {
                                match nested {
                                    // Handle string literal format: #[idl_type("TypeName")]
                                    NestedMeta::Lit(Lit::Str(lit_str)) => {
                                        let type_str = lit_str.value();
                                        if let Ok(rust_type) =
                                            RustType::try_from(
                                                type_str.as_str(),
                                            )
                                        {
                                            return Some(
                                                StructFieldAttr::IdlType(
                                                    rust_type,
                                                ),
                                            );
                                        }
                                    }
                                    // Handle direct type format: #[idl_type(TypeName)]
                                    NestedMeta::Meta(meta) => {
                                        if let Some(ident) =
                                            meta.path().get_ident()
                                        {
                                            let type_str = ident.to_string();
                                            if let Ok(rust_type) =
                                                RustType::try_from(
                                                    type_str.as_str(),
                                                )
                                            {
                                                return Some(
                                                    StructFieldAttr::IdlType(
                                                        rust_type,
                                                    ),
                                                );
                                            }
                                        } else {
                                            // Handle path with segments (like std::string::String)
                                            let path_str = meta
                                                .path()
                                                .segments
                                                .iter()
                                                .map(|seg| {
                                                    seg.ident.to_string()
                                                })
                                                .collect::<Vec<_>>()
                                                .join("::");

                                            if let Ok(rust_type) =
                                                RustType::try_from(
                                                    path_str.as_str(),
                                                )
                                            {
                                                return Some(
                                                    StructFieldAttr::IdlType(
                                                        rust_type,
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    _ => {}
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
