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

    use crate::parsed_struct::{Seed, StructAttr, StructFieldAttr};

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

    // -----------------
    // Seeds
    // -----------------

    /*
    fn token_metadata_seeds() {
        // https://github.com/metaplex-foundation/metaplex-program-library/blob/master/token-metadata/program/src/utils.rs#L411
        let edition_seeds: &[&[u8]; 4] = &[
            PREFIX.as_bytes(),
            program_id.as_ref(),
            mint.as_ref(),
            EDITION.as_bytes(),
        ];

        // https://github.com/metaplex-foundation/metaplex-program-library/blob/master/token-metadata/program/src/processor.rs#L1959
        let edition_marker_number = print_edition
            .edition
            .checked_div(EDITION_MARKER_BIT_SIZE)
            .ok_or(MetadataError::NumericalOverflowError)?;
        let edition_marker_number_str = edition_marker_number.to_string();

        // Ensure we were passed the correct edition marker PDA.
        let edition_marker_info_path = Vec::from([
            PREFIX.as_bytes(),
            program_id.as_ref(),
            // AccountInfo
            master_edition_mint_info.key.as_ref(),
            EDITION.as_bytes(),
            edition_marker_number_str.as_bytes(),
        ]);
    }
    */

    #[test]
    fn extract_account_from_account_struct_experiment() {
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds(
                /* literal    */ "lit:prefix",
                /* program_id */ program_id,
                /* pubkey     */ some_pubkey("description of some pubkey"),
                /* byte       */ some_byte("description of byte", u8),
            )]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });
        let all_structs = vec![&account_struct].into_iter();

        let res = extract_account_structs(all_structs);
        eprintln!("{:#?}", res);
    }

    fn extract_seeds_attr(account_struct: &ItemStruct) -> StructAttr {
        let all_structs = vec![account_struct].into_iter();
        let res = extract_account_structs(all_structs)
            .expect("Should parse struct without error");

        let struct_attrs = res.into_iter().nth(0).unwrap().struct_attrs;
        assert_eq!(struct_attrs.len(), 1, "Extracts one attr");

        struct_attrs
            .items()
            .into_iter()
            .nth(0)
            .expect("Should extract one struct attr")
    }

    #[test]
    fn account_with_literal_seed() {
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds("lit:prefix")]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });

        let attr = extract_seeds_attr(&account_struct);
        assert_matches!(attr,
         StructAttr::Seeds(seeds) => {
            assert_eq!(seeds.get_literals(), vec!["lit:prefix".to_string()]);
        });
    }

    #[test]
    fn account_with_program_id_seed() {
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds(program_id)]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });

        let attr = extract_seeds_attr(&account_struct);
        assert_matches!(attr,
         StructAttr::Seeds(seeds) => {
            assert_eq!(seeds.get_program_ids(), vec![Seed::ProgramId]);
        });
    }

    #[test]
    fn account_with_pubkey_seed() {
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds(mypubkey("desc of my pubkey"))]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });

        let attr = extract_seeds_attr(&account_struct);
        assert_matches!(attr,
         StructAttr::Seeds(seeds) => {
            assert_eq!(
                seeds.get_params(),
                vec![Seed::Param("mypubkey".to_string(), "desc of my pubkey".to_string(), None)]
            );
        });
    }

    #[test]
    fn account_with_byte_seed() {
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds(mybyte("desc of my byte", u8))]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });

        let attr = extract_seeds_attr(&account_struct);
        assert_matches!(attr,
         StructAttr::Seeds(seeds) => {
            assert_eq!(
                seeds.get_params(),
                vec![Seed::Param(
                    "mybyte".to_string(),
                    "desc of my byte".to_string(),
                    Some("u8".to_string())
                )]
            );
        });
    }

    #[test]
    fn account_with_u32_seed() {
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds(myu32("desc of my u32", u32))]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });

        let attr = extract_seeds_attr(&account_struct);
        assert_matches!(attr,
         StructAttr::Seeds(seeds) => {
            assert_eq!(
                seeds.get_params(),
                vec![Seed::Param(
                    "myu32".to_string(),
                    "desc of my u32".to_string(),
                    Some("u32".to_string())
                )]
            );
        });
    }

    #[test]
    fn candy_guard_seeds_mint_limit() {
        // https://github.com/metaplex-foundation/candy-guard/blob/30481839256f192840da609d0d2c26c28a1051f4/program/src/guards/mint_limit.rs#L51
        /*
        let seeds = [
            &[self.id],                 // self.id: u8
            user.as_ref(),              // Pubkey
            candy_guard_key.as_ref(),   // &Pubkey
            candy_machine_key.as_ref(), // &Pubkey
        ];
        */
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds(
                id("MintLimit id", u8),
                user("User key"),
                candy_guard_key("Candy Guard key"),
                candy_machine_key("Candy Machine key"),
            )]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });

        let attr = extract_seeds_attr(&account_struct);
        assert_matches!(attr,
          StructAttr::Seeds(seeds) => {
            assert_eq!(
                seeds.get_params(),
                vec![
                    Seed::Param(
                        "id".to_string(),
                        "MintLimit id".to_string(),
                        Some("u8".to_string())
                    ),
                    Seed::Param(
                        "user".to_string(),
                        "User key".to_string(),
                        None,
                    ),
                    Seed::Param(
                        "candy_guard_key".to_string(),
                        "Candy Guard key".to_string(),
                        None,
                    ),
                    Seed::Param(
                        "candy_machine_key".to_string(),
                        "Candy Machine key".to_string(),
                        None,
                    ),
                ]
            );
        });
    }

    #[test]
    fn candy_guard_seeds_wrap() {
        // https://github.com/metaplex-foundation/candy-guard/blob/abdde4308b44857576154d6930a04c13e9c8cfda/program/src/instructions/wrap.rs#L12
        // pub const SEED: &[u8] = b"candy_guard";
        // let seeds = [
        //   SEED,                          // &[u8] (passing as literal)
        //   &candy_guard.base.to_bytes(),  // candy_guard: Account .base: Pubkey
        //   &[candy_guard.bump]            // candy_guard.bump: u8
        // ];
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds(
                "candy_guard",
                user("User key"),
                candy_guard_base("Candy Guard base"),
                candy_guard_bump("Determined bump", u8),
            )]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });
        let attr = extract_seeds_attr(&account_struct);
        assert_matches!(attr,
          StructAttr::Seeds(seeds) => {
            assert_eq!(seeds.get_literals(), vec!["candy_guard".to_string()]);
            assert_eq!(
                seeds.get_params(),
                vec![
                    Seed::Param(
                        "user".to_string(),
                        "User key".to_string(),
                        None,
                    ),
                    Seed::Param(
                        "candy_guard_base".to_string(),
                        "Candy Guard base".to_string(),
                        None,
                    ),
                    Seed::Param(
                        "candy_guard_bump".to_string(),
                        "Determined bump".to_string(),
                        Some(
                            "u8".to_string(),
                        ),
                    ),
                ],
            );
        });
    }

    #[test]
    fn candy_guard_seeds_update() {
        // https://github.com/metaplex-foundation/candy-guard/blob/abdde4308b44857576154d6930a04c13e9c8cfda/program/src/instructions/update.rs#L83
        // #[account(
        //   seeds = [
        //       SEED,                           // same as candy_guard_seeds_wrap
        //       candy_guard.base.key().as_ref() // candy_guard: Account, base: Pubkey
        //   ];
        //   bump = candy_guard.bump             // u8
        // )]
        let account_struct = parse_struct(quote! {
            #[derive(ShankAccount)]
            #[seeds(
                "candy_guard",
                candy_guard_base("Candy Guard base"),
                candy_guard_bump("Determined bump", u8),
            )]
            struct AccountStructWithLiteralSeed {
                count: u8,
            }
        });
        let attr = extract_seeds_attr(&account_struct);
        assert_matches!(attr,
          StructAttr::Seeds(seeds) => {
            assert_eq!(seeds.get_literals(), vec!["candy_guard".to_string()]);
            assert_eq!(
                seeds.get_params(),
                vec![
                    Seed::Param(
                        "candy_guard_base".to_string(),
                        "Candy Guard base".to_string(),
                        None,
                    ),
                    Seed::Param(
                        "candy_guard_bump".to_string(),
                        "Determined bump".to_string(),
                        Some(
                            "u8".to_string(),
                        ),
                    ),
                ],
            );
        });
    }
}
