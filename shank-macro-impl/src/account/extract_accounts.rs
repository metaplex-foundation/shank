use std::{collections::HashSet, convert::TryFrom};

use crate::{
    parsed_struct::{ParsedStruct, StructFieldAttr},
    parsers::get_derive_attr,
    DERIVE_ACCOUNT_ATTR,
};

use anyhow::{format_err, Result};

fn filter_account_structs<'a>(
    structs: impl Iterator<Item = &'a syn::ItemStruct>,
) -> Vec<&'a syn::ItemStruct> {
    structs
        .filter_map(|item_strct| {
            get_derive_attr(&item_strct.attrs, DERIVE_ACCOUNT_ATTR)
                .map(|_| item_strct)
        })
        .collect()
}

pub fn extract_account_structs<'a>(
    structs: impl Iterator<Item = &'a syn::ItemStruct>,
) -> Result<Vec<ParsedStruct>> {
    let mut account_structs = Vec::new();

    for x in filter_account_structs(structs) {
        let strct = ParsedStruct::try_from(x).map_err(|err| {
            format_err!(
                "Encountered an error parsing {} Account.\n{}",
                x.ident,
                err
            )
        })?;
        verify_account_struct(&strct)?;
        account_structs.push(strct);
    }
    Ok(account_structs)
}

fn verify_account_struct(strct: &ParsedStruct) -> Result<()> {
    if strct.fields.is_empty() {
        return Err(format_err!(
            "Account struct {} has no fields",
            strct.ident
        ));
    }
    // TODO(thlorenz): Don't allow more than one padding field
    let mut padded_fields = HashSet::new();
    for f in &strct.fields {
        if f.attrs.get(&StructFieldAttr::Padding).is_some() {
            if f.rust_type.ident != "Array" {
                return Err(format_err!(
                    "Account struct {} field {} has padding attribute, but is not an Array, i.e. [u8; 36]",
                    strct.ident,
                    f.ident
                ));
            } else {
                padded_fields.insert(f.ident.to_string());
            }
        }
    }

    if padded_fields.len() > 1 {
        return Err(format_err!(
            "Account struct {} has more than one padded field: [ {} ]",
            strct.ident,
            padded_fields.iter().cloned().collect::<Vec<_>>().join(", ")
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::parsed_struct::StructFieldAttr;

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

    fn account_struct_with_one_field() -> ItemStruct {
        parse_struct(quote! {
            #[derive(ShankAccount)]
            struct AccountStruct {
                id: Pubkey,
            }
        })
    }

    fn account_struct_with_two_fields() -> ItemStruct {
        parse_struct(quote! {
            #[derive(ShankAccount)]
            struct AccountStructWithTwoFields {
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
        let account_struct = account_struct_with_one_field();
        let all_structs = vec![&other_struct, &account_struct].into_iter();

        let account_structs = filter_account_structs(all_structs);
        assert_eq!(account_structs.len(), 1, "len");
        assert_eq!(account_structs[0].ident, "AccountStruct");
    }

    #[test]
    fn extract_accounts_from_structs_with_accounts() {
        let other_struct = other_struct();
        let account_struct = account_struct_with_one_field();
        let account_struct_with_fields = account_struct_with_two_fields();
        let all_structs =
            vec![&other_struct, &account_struct, &account_struct_with_fields]
                .into_iter();

        let accounts =
            extract_account_structs(all_structs).expect("extracts accounts");
        assert_eq!(accounts.len(), 2, "two accounts");
        assert_matches!(&accounts[0], ParsedStruct { ident, fields, .. } => {
            assert_eq!(ident, "AccountStruct");
            assert_eq!(fields.len(), 1);
        });
        assert_matches!(&accounts[1], ParsedStruct { ident, fields, .. } => {
            assert_eq!(ident, "AccountStructWithTwoFields");
            assert_eq!(fields.len(), 2);
        });
    }
    // -----------------
    // Padding
    // -----------------

    fn account_struct_with_valid_padding() -> ItemStruct {
        parse_struct(quote! {
            #[derive(ShankAccount)]
            struct AccountStruct {
                count: u8,
                #[padding]
                _padding: [u8; 3],
            }
        })
    }

    fn account_struct_with_invalid_padding() -> ItemStruct {
        // _padding needs to be array
        parse_struct(quote! {
            #[derive(ShankAccount)]
            struct AccountStruct {
                count: u8,
                #[padding]
                _padding: u8,
            }
        })
    }

    fn account_struct_with_two_padded_fields() -> ItemStruct {
        parse_struct(quote! {
            #[derive(ShankAccount)]
            struct AccountStructWithTwoPaddedFields {
                count: u8,
                #[padding]
                _padding: [u8; 3],
                #[padding]
                _other_padding: [u8; 3],
            }
        })
    }

    #[test]
    fn extract_account_from_account_struct_with_valid_padding() {
        let account_struct = account_struct_with_valid_padding();
        let all_structs = vec![&account_struct].into_iter();

        let accounts =
            extract_account_structs(all_structs).expect("extracts accounts");

        assert_eq!(accounts.len(), 1, "one account");
        assert_matches!(&accounts[0], ParsedStruct { ident, fields, .. } => {
            assert_eq!(ident, "AccountStruct");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].attrs.len(), 0, "first field not padded");
            assert_eq!(fields[1].attrs.len(), 1, "second field has one attribute");
            assert_eq!(fields[1].attrs.get(&StructFieldAttr::Padding), Some(&StructFieldAttr::Padding), "second field has padding attribute");
        });
    }

    #[test]
    fn extract_account_from_account_struct_with_invalid_padding() {
        let account_struct = account_struct_with_invalid_padding();
        let all_structs = vec![&account_struct].into_iter();

        let res = extract_account_structs(all_structs);
        assert_matches!(res, Err(err) => {
            assert!(err.to_string().contains("Account struct AccountStruct field _padding has padding attribute, but is not an Array"));
        });
    }

    #[test]
    fn extract_account_from_account_struct_with_two_padded_fields() {
        let account_struct = account_struct_with_two_padded_fields();
        let all_structs = vec![&account_struct].into_iter();

        let res = extract_account_structs(all_structs);
        assert_matches!(res, Err(err) => {
            assert!(err.to_string().contains("AccountStructWithTwoPaddedFields has more than one padded field"));
        });
    }
}
