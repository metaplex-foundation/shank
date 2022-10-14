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

#[derive(Debug, PartialEq)]
pub struct InstructionAccountWithoutIdent {
    pub index: Option<u32>,
    pub name: String,
    pub writable: bool,
    pub signer: bool,
    pub desc: Option<String>,
    pub optional: bool,
}

impl From<&InstructionAccount> for InstructionAccountWithoutIdent {
    fn from(acc: &InstructionAccount) -> Self {
        let InstructionAccount {
            index,
            name,
            writable,
            signer,
            desc,
            optional,
            ..
        } = acc;
        Self {
            index: *index,
            name: name.clone(),
            writable: *writable,
            signer: *signer,
            desc: desc.clone(),
            optional: optional.clone(),
        }
    }
}
impl From<&InstructionAccounts> for Vec<InstructionAccountWithoutIdent> {
    fn from(accs: &InstructionAccounts) -> Self {
        accs.0
            .iter()
            .map(InstructionAccountWithoutIdent::from)
            .collect()
    }
}

pub fn assert_instruction_account_matches(
    acc_actual: &InstructionAccount,
    acc_expected: InstructionAccountWithoutIdent,
) {
    let acc_actual = InstructionAccountWithoutIdent::from(acc_actual);
    assert_eq!(acc_actual, acc_expected, "account matches");
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
    assert_instruction_account_matches(
        &accounts_indexed.0[0],
        InstructionAccountWithoutIdent {
            index: Some(0),
            name: "authority".to_string(),
            writable: false,
            signer: false,
            desc: None,
            optional: false,
        },
    );

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(name="authority")]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_instruction_account_matches(
        &accounts.0[0],
        InstructionAccountWithoutIdent {
            index: None,
            name: "authority".to_string(),
            writable: false,
            signer: false,
            desc: None,
            optional: false,
        },
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
    assert_instruction_account_matches(
        &accounts_indexed.0[0],
        InstructionAccountWithoutIdent {
            index: Some(0),
            name: "authority".to_string(),
            writable: false,
            signer: true,
            desc: None,
            optional: false,
        },
    );

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(name="authority", sign)]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_instruction_account_matches(
        &accounts.0[0],
        InstructionAccountWithoutIdent {
            index: None,
            name: "authority".to_string(),
            writable: false,
            signer: true,
            desc: None,
            optional: false,
        },
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
    assert_instruction_account_matches(
        &accounts_indexed.0[0],
        InstructionAccountWithoutIdent {
            index: Some(0),
            name: "authority".to_string(),
            writable: true,
            signer: false,
            desc: None,
            optional: false,
        },
    );

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(w, name="authority")]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_instruction_account_matches(
        &accounts.0[0],
        InstructionAccountWithoutIdent {
            index: None,
            name: "authority".to_string(),
            writable: true,
            signer: false,
            desc: None,
            optional: false,
        },
    );
}

#[test]
fn account_optional() {
    let accounts_indexed = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(0, name="authority", writable, optional)]
            Indexed
        }
    })
    .expect("Should parse fine");
    assert_instruction_account_matches(
        &accounts_indexed.0[0],
        InstructionAccountWithoutIdent {
            index: Some(0),
            name: "authority".to_string(),
            writable: true,
            signer: false,
            desc: None,
            optional: true,
        },
    );

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(w, name="authority", optional)]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_instruction_account_matches(
        &accounts.0[0],
        InstructionAccountWithoutIdent {
            index: None,
            name: "authority".to_string(),
            writable: true,
            signer: false,
            desc: None,
            optional: true,
        },
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

    assert_instruction_account_matches(
        &accounts_indexed.0[0],
        InstructionAccountWithoutIdent {
            index: Some(0),
            name: "funnel".to_string(),
            writable: false,
            signer: false,
            desc: Some("Readonly indexed account description".to_string()),
            optional: false,
        },
    );
}

#[test]
fn account_multiple_attrs() {
    let expected_indexed = vec![
        InstructionAccountWithoutIdent {
            index: Some(0),
            name: "authority".to_string(),
            writable: false,
            signer: true,
            desc: Some("Signer account".to_string()),
            optional: false,
        },
        InstructionAccountWithoutIdent {
            index: Some(1),
            name: "storage".to_string(),
            writable: true,
            signer: false,
            desc: Some("Writable account".to_string()),
            optional: false,
        },
        InstructionAccountWithoutIdent {
            index: Some(2),
            name: "funnel".to_string(),
            writable: false,
            signer: false,
            desc: Some("Readonly account".to_string()),
            optional: false,
        },
        InstructionAccountWithoutIdent {
            index: Some(3),
            name: "optional_account".to_string(),
            writable: false,
            signer: false,
            desc: Some("Readonly optional account".to_string()),
            optional: true,
        },
    ];

    let expected_non_indexed: Vec<InstructionAccountWithoutIdent> =
        expected_indexed
            .iter()
            .map(
                |InstructionAccountWithoutIdent {
                     name,
                     writable,
                     signer,
                     desc,
                     optional,
                     ..
                 }| {
                    InstructionAccountWithoutIdent {
                        index: None,
                        name: name.clone(),
                        writable: writable.clone(),
                        signer: signer.clone(),
                        desc: desc.clone(),
                        optional: optional.clone(),
                    }
                },
            )
            .collect();

    let accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(name ="authority", sig, desc = "Signer account")]
            #[account(name ="storage", mut, desc = "Writable account")]
            #[account(name ="funnel", desc = "Readonly account")]
            #[account(name ="optional_account", desc = "Readonly optional account", optional)]
            NotIndexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(
        <Vec<InstructionAccountWithoutIdent>>::from(&accounts),
        expected_non_indexed
    );

    let indexed_accounts = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(0, name ="authority", sig, desc = "Signer account")]
            #[account(1, name ="storage", mut, desc = "Writable account")]
            #[account(2, name ="funnel", desc = "Readonly account")]
            #[account(3, name ="optional_account", desc = "Readonly optional account", optional)]
            Indexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(
        <Vec<InstructionAccountWithoutIdent>>::from(&indexed_accounts),
        expected_indexed
    );
}

#[test]
fn account_invalid_empty_name() {
    assert_matches!(
    parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(name ="", sig, desc = "Signer account")]
            NotIndexed
        }
    }),
        Err(err) if err.to_string().contains("account name cannot be empty"));
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
