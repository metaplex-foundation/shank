use std::collections::HashSet;

use syn::Attribute;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StructFieldAttr {
    Padding,
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
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}
