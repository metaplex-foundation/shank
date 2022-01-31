use assert_matches::assert_matches;
use std::convert::TryInto;

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

use syn::{Attribute, Result as ParseResult};

use crate::instruction::account_attrs::InstructionAccount;

use super::account_attrs::InstructionAccounts;

fn parse_first_enum_variant_attrs(
    code: TokenStream,
) -> ParseResult<InstructionAccounts> {
    let parsed =
        syn::parse2::<ItemEnum>(code).expect("Should parse successfully");
    let attrs: &[Attribute] = parsed.variants.first().unwrap().attrs.as_ref();
    attrs.try_into()
}

#[test]
fn account_readonly() {
    let accounts_indexed = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(0, name="authority")]
            Indexed
        }
    })
    .expect("Should parse fine");
    assert_eq!(
        accounts_indexed.0[0],
        InstructionAccount {
            index: Some(0,),
            name: "authority".to_string(),
            writable: false,
            signer: false,
            desc: None,
        }
    );

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(name="authority")]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(
        accounts.0[0],
        InstructionAccount {
            index: None,
            name: "authority".to_string(),
            writable: false,
            signer: false,
            desc: None,
        }
    );
}

#[test]
fn account_signer() {
    let accounts_indexed = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
            pub enum Instructions {
                #[account(0, signer, name = "authority")]
                Indexed
            }
    })
    .expect("Should parse fine");
    assert_eq!(
        accounts_indexed.0[0],
        InstructionAccount {
            index: Some(0,),
            name: "authority".to_string(),
            writable: false,
            signer: true,
            desc: None,
        }
    );

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(name="authority", sign)]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(
        accounts.0[0],
        InstructionAccount {
            index: None,
            name: "authority".to_string(),
            writable: false,
            signer: true,
            desc: None,
        }
    );
}

#[test]
fn account_writable() {
    let accounts_indexed = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(0, name="authority", writable)]
            Indexed
        }
    })
    .expect("Should parse fine");
    assert_eq!(
        accounts_indexed.0[0],
        InstructionAccount {
            index: Some(0,),
            name: "authority".to_string(),
            writable: true,
            signer: false,
            desc: None,
        }
    );

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(w, name="authority")]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(
        accounts.0[0],
        InstructionAccount {
            index: None,
            name: "authority".to_string(),
            writable: true,
            signer: false,
            desc: None,
        }
    );
}

#[test]
fn account_desc() {
    let accounts_indexed = parse_first_enum_variant_attrs(quote! {
            #[derive(ShankInstruction)]
            pub enum Instructions {
                #[account(0, name ="funnel", desc = "Readonly indexed account description")]
                Indexed
            }
        })
        .expect("Should parse fine");

    assert_eq!(
        accounts_indexed.0[0],
        InstructionAccount {
            index: Some(0,),
            name: "funnel".to_string(),
            writable: false,
            signer: false,
            desc: Some("Readonly indexed account description".to_string()),
        }
    );
}

#[test]
fn account_multiple_attrs() {
    let expected_indexed = InstructionAccounts(vec![
        InstructionAccount {
            index: Some(0),
            name: "authority".to_string(),
            writable: false,
            signer: true,
            desc: Some("Signer account".to_string()),
        },
        InstructionAccount {
            index: Some(1),
            name: "storage".to_string(),
            writable: true,
            signer: false,
            desc: Some("Writable account".to_string()),
        },
        InstructionAccount {
            index: Some(2),
            name: "funnel".to_string(),
            writable: false,
            signer: false,
            desc: Some("Readonly account".to_string()),
        },
    ]);

    let expected_non_indexed = InstructionAccounts(
        expected_indexed
            .0
            .iter()
            .map(
                |InstructionAccount {
                     name,
                     writable,
                     signer,
                     desc,
                     ..
                 }| {
                    InstructionAccount {
                        index: None,
                        name: name.clone(),
                        writable: writable.clone(),
                        signer: signer.clone(),
                        desc: desc.clone(),
                    }
                },
            )
            .collect(),
    );

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(name ="authority", sig, desc = "Signer account")]
            #[account(name ="storage", mut, desc = "Writable account")]
            #[account(name ="funnel", desc = "Readonly account")]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(&accounts, &expected_non_indexed);

    let indexed_accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(0, name ="authority", sig, desc = "Signer account")]
            #[account(1, name ="storage", mut, desc = "Writable account")]
            #[account(2, name ="funnel", desc = "Readonly account")]
            Indexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(&indexed_accounts, &expected_indexed);
}

#[test]
fn account_invalid_indexes() {
    assert_matches!(parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(0, name ="authority", sig, desc = "Signer account")]
            #[account(1, name ="storage", mut, desc = "Writable account")]
            #[account(3, name ="funnel", desc = "Readonly account")]
            Indexed
        }
    }) ,
        Err(err) if err.to_string().contains("index 3 does not match"));

    assert_matches!(parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(1, name ="authority", sig, desc = "Signer account")]
            Indexed
        }
    }) ,
        Err(err) if err.to_string().contains("index 1 does not match"));
    assert_matches!(parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(0, name ="authority", sig, desc = "Signer account")]
            #[account(2, name ="storage", mut, desc = "Writable account")]
            #[account(2, name ="funnel", desc = "Readonly account")]
            Indexed
        }
    }) ,
        Err(err) if err.to_string().contains("index 2 does not match"));
}
