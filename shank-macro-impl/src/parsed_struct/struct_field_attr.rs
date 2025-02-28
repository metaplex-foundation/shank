use std::collections::HashSet;
use std::convert::TryFrom;

use crate::types::RustType;
use syn::{Attribute, Meta, NestedMeta, Lit};

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
                        // Parse the attribute to check for the "as" parameter
                        match attr.parse_meta() {
                            Ok(Meta::List(meta_list)) => {
                                for nested in meta_list.nested {
                                    if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                                        if name_value.path.is_ident("as") {
                                            if let Lit::Str(lit_str) = name_value.lit {
                                                let type_str = lit_str.value();
                                                
                                                // Parse the type string into a RustType
                                                match RustType::try_from(type_str.as_str()) {
                                                    Ok(rust_type) => {
                                                        return Some(StructFieldAttr::ShankAs(rust_type));
                                                    },
                                                    Err(_) => {},
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Err(_) => {},
                            _ => {}
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
