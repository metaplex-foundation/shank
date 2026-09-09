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
///
/// The builtin `derive` macro may be path qualified, e.g.
/// `#[::core::prelude::v1::derive(..)]`, so only the last path segment is
/// compared.
fn derive_names_of_attr(attr: &Attribute) -> Vec<Ident> {
    match &attr.meta {
        Meta::List(list) if is_derive_path(&list.path) => {
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

fn is_derive_path(path: &syn::Path) -> bool {
    path.segments.last().is_some_and(|x| x.ident == "derive")
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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::ItemStruct;

    fn derive_names(code: proc_macro2::TokenStream) -> Vec<String> {
        let item = syn::parse2::<ItemStruct>(code).expect("parses");
        get_derive_names(&item.attrs)
    }

    #[test]
    fn derive_names_of_plain_derive() {
        assert_eq!(
            derive_names(quote! {
                #[derive(Debug, ShankAccount)]
                #[repr(C)]
                struct Foo;
            }),
            vec!["Debug", "ShankAccount"]
        );
    }

    #[test]
    fn derive_names_of_path_qualified_derive() {
        assert_eq!(
            derive_names(quote! {
                #[::core::prelude::v1::derive(ShankType)]
                struct Foo;
            }),
            vec!["ShankType"]
        );
    }

    #[test]
    fn derive_names_ignore_other_attrs_mentioning_derive() {
        assert!(derive_names(quote! {
            #[derive::something(ShankType)]
            #[derivex(ShankType)]
            #[not_derive = "ShankType"]
            struct Foo;
        })
        .is_empty());
    }
}
