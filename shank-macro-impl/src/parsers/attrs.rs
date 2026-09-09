use proc_macro2::Ident;
use syn::{Attribute, Meta};

use super::{parse_nested_metas_of_list, NestedMeta, NestedMetas};

fn flattened_idents_from_nested_meta(nested: &NestedMetas) -> Vec<Ident> {
    nested
        .iter()
        .flat_map(|nested| match nested {
            NestedMeta::Meta(Meta::Path(path)) => {
                path.segments.iter().map(|x| x.ident.clone()).collect()
            }
            NestedMeta::Lit(_) => {
                todo!("Handle NestedMeta::Lit for derive nested")
            }
            _ => vec![],
        })
        .collect()
}

/// Returns the names listed in `attr` if it is a `#[derive(..)]` attribute.
fn derive_names_of_attr(attr: &Attribute) -> Vec<Ident> {
    match &attr.meta {
        Meta::List(list)
            if list.path.segments.iter().any(|x| x.ident == "derive") =>
        {
            match parse_nested_metas_of_list(list) {
                Ok(nested) => flattened_idents_from_nested_meta(&nested),
                Err(err) => {
                    eprintln!("{:#?}", err);
                    vec![]
                }
            }
        }
        _ => vec![],
    }
}

pub fn get_derive_names(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .flat_map(|attr| {
            derive_names_of_attr(attr)
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

pub fn attr_is_derive(attr: &&Attribute, derive: &str) -> bool {
    derive_names_of_attr(attr)
        .into_iter()
        .any(|ident| ident == derive)
}

pub fn get_derive_attr<'a>(
    attrs: &'a [Attribute],
    derive: &str,
) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr_is_derive(attr, derive))
}
