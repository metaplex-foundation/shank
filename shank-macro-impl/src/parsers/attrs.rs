use proc_macro2::Ident;
use syn::{
    punctuated::Punctuated, Attribute, Meta, MetaList, NestedMeta, Token,
};

fn flattened_idents_from_nested_meta(
    nested: &Punctuated<NestedMeta, Token![,]>,
) -> Vec<Ident> {
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

pub fn get_derive_names(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .flat_map(|attr| {
            let meta = &attr.parse_meta();
            match meta {
                Ok(Meta::List(MetaList { path, nested, .. })) => {
                    let derive = path
                        .segments
                        .iter()
                        .enumerate()
                        .find(|(_, x)| x.ident == "derive");

                    match derive {
                        Some(_) => flattened_idents_from_nested_meta(nested)
                            .into_iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>(),
                        None => vec![],
                    }
                }
                Ok(_) => vec![],
                Err(_) => vec![],
            }
        })
        .collect()
}

pub fn attr_is_derive(attr: &&Attribute, derive: &str) -> bool {
    let meta = &attr.parse_meta();

    match meta {
        Ok(Meta::List(MetaList { path, nested, .. })) => {
            let found_derive =
                path.segments.iter().find(|x| x.ident == "derive");

            match found_derive {
                Some(_) => flattened_idents_from_nested_meta(nested)
                    .into_iter()
                    .any(|ident| ident == derive),
                None => false,
            }
        }
        Ok(_) => false,
        Err(err) => {
            eprintln!("{:#?}", err);
            false
        }
    }
}

pub fn get_derive_attr<'a>(
    attrs: &'a [Attribute],
    derive: &str,
) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr_is_derive(attr, derive))
}
