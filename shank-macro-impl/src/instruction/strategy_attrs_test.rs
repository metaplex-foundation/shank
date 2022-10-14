use std::convert::TryInto;

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

use syn::{Attribute, Result as ParseResult};

use crate::instruction::{
    account_attrs_test::{
        assert_instruction_account_matches, InstructionAccountWithoutIdent,
    },
    InstructionStrategy,
};

use super::{account_attrs::InstructionAccounts, InstructionStrategies};

fn parse_first_enum_variant_attrs(
    code: TokenStream,
) -> ParseResult<(InstructionAccounts, InstructionStrategies)> {
    let parsed =
        syn::parse2::<ItemEnum>(code).expect("Should parse successfully");
    let attrs: &[Attribute] = parsed.variants.first().unwrap().attrs.as_ref();
    let instruction_accounts = attrs.try_into()?;
    let instruction_strategies = attrs.into();
    Ok((instruction_accounts, instruction_strategies))
}

#[test]
fn instruction_with_default_optional_accounts() {
    let (accounts, strategies) = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[default_optional_accounts]
            #[account(name="authority")]
            NonIndexed
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

    assert_eq!(strategies.0.len(), 1, "includes one instruction strategy");
    assert!(
        strategies
            .0
            .contains(&InstructionStrategy::DefaultOptionalAccounts),
        "to default optional accounts"
    );
}

#[test]
fn instruction_without_default_optional_accounts() {
    let (accounts, strategies) = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankInstruction)]
        pub enum Instructions {
            #[account(name="authority")]
            NonIndexed
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

    assert_eq!(strategies.0.len(), 0, "includes no instruction strategy");
}
