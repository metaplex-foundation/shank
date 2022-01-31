use syn::{Attribute, Meta, MetaList, NestedMeta};

pub fn attr_is_derive(attr: &&Attribute, derive: &str) -> bool {
    let meta = &attr.parse_meta();

    match meta {
        Ok(Meta::List(MetaList { path, nested, .. })) => {
            let derive_idx = path
                .segments
                .iter()
                .enumerate()
                .find(|(_, x)| x.ident == "derive")
                .map(|(idx, _)| idx);
            match derive_idx {
                Some(idx) => match &nested[idx] {
                    NestedMeta::Meta(Meta::Path(path)) => path
                        .segments
                        .iter()
                        .find(|x| x.ident == derive)
                        .is_some(),
                    NestedMeta::Lit(_) => {
                        todo!("Handle NestedMeta::Lit for derive nested")
                    }
                    _ => false,
                },
                None => false,
            }
        }
        Ok(_) => false,
        Err(_) => false,
    }
}

pub fn get_derive_attr<'a, 'b>(
    attrs: &'a [Attribute],
    derive: &'b str,
) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr_is_derive(attr, derive))
}
