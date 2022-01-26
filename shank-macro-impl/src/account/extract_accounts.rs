use syn::{Attribute, Meta, MetaList, NestedMeta};

use super::{parse_account_item_struct, AccountStruct};
use anyhow::{format_err, Result};

pub const DERIVE_ACCOUNT_ATTR: &str = "ShankAccount";

fn is_derive_account_attr(attr: &&Attribute) -> bool {
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
                        .find(|x| x.ident == DERIVE_ACCOUNT_ATTR)
                        .is_some(),
                    NestedMeta::Lit(_) => todo!("Handle NestedMeta::Lit for derive nested"),
                    _ => false,
                },
                None => false,
            }
        }
        Ok(_) => false,
        Err(_) => false,
    }
}

fn filter_account_structs<'a>(
    structs: impl Iterator<Item = &'a syn::ItemStruct>,
) -> Vec<&'a syn::ItemStruct> {
    structs
        .filter_map(|item_strct| {
            let attrs_count = item_strct
                .attrs
                .iter()
                .filter(is_derive_account_attr)
                .count();
            match attrs_count {
                0 => None,
                1 => Some(item_strct),
                _ => panic!("Invalid syntax: one event attribute allowed"),
            }
        })
        .collect()
}

pub fn extract_account_structs<'a>(
    structs: impl Iterator<Item = &'a syn::ItemStruct>,
) -> Result<Vec<AccountStruct>> {
    let mut account_structs = Vec::new();

    for x in filter_account_structs(structs) {
        let strct = parse_account_item_struct(x).map_err(|err| {
            format_err!("Encountered an error parsing {} Account.\n{}", x.ident, err)
        })?;
        account_structs.push(strct);
    }
    Ok(account_structs)
}

#[cfg(test)]
mod tests {

    use super::*;
    use assert_matches::assert_matches;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::ItemStruct;

    fn parse_struct(code: TokenStream) -> ItemStruct {
        syn::parse2::<ItemStruct>(code).expect("Should parse successfully")
    }

    fn other_struct() -> ItemStruct {
        parse_struct(quote! { struct OtherStruct {} })
    }

    fn account_struct() -> ItemStruct {
        parse_struct(quote! {
            #[derive(ShankAccount)]
            struct AccountStruct {}
        })
    }

    fn account_struct_with_fields() -> ItemStruct {
        parse_struct(quote! {
            #[derive(ShankAccount)]
            struct AccountStructWithFields {
                id: Pubkey,
                count: u8,
            }
        })
    }

    #[test]
    fn filter_account_structs_without_accounts() {
        let other_struct = other_struct();
        let all_structs = vec![&other_struct].into_iter();

        let account_structs = filter_account_structs(all_structs);
        assert_eq!(account_structs.len(), 0, "len");
    }

    #[test]
    fn filter_account_structs_with_accounts() {
        let other_struct = other_struct();
        let account_struct = account_struct();
        let all_structs = vec![&other_struct, &account_struct].into_iter();

        let account_structs = filter_account_structs(all_structs);
        assert_eq!(account_structs.len(), 1, "len");
        assert_eq!(account_structs[0].ident, "AccountStruct");
    }

    #[test]
    fn extract_accounts_from_structs_with_accounts() {
        let other_struct = other_struct();
        let account_struct = account_struct();
        let account_struct_with_fields = account_struct_with_fields();
        let all_structs =
            vec![&other_struct, &account_struct, &account_struct_with_fields].into_iter();

        let accounts = extract_account_structs(all_structs).expect("extracts accounts");
        assert_eq!(accounts.len(), 2, "two accounts");
        assert_matches!(&accounts[0], AccountStruct { ident, fields } => {
            assert_eq!(ident, "AccountStruct");
            assert_eq!(fields.len(), 0);
        });
        assert_matches!(&accounts[1], AccountStruct { ident, fields } => {
            assert_eq!(ident, "AccountStructWithFields");
            assert_eq!(fields.len(), 2);
        });
    }
}
