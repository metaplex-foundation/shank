use std::convert::TryInto;

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

use syn::{Attribute, Result as ParseResult};

use crate::instruction::InstructionAccounts;

use super::BuilderArguments;

fn parse_first_enum_variant_attrs(
    code: TokenStream,
) -> ParseResult<(InstructionAccounts, BuilderArguments)> {
    let parsed =
        syn::parse2::<ItemEnum>(code).expect("Should parse successfully");
    let attrs: &[Attribute] = parsed.variants.first().unwrap().attrs.as_ref();
    let instruction_accounts = attrs.try_into()?;
    let instruction_arguments = attrs.try_into()?;
    Ok((instruction_accounts, instruction_arguments))
}

#[test]
fn instruction_with_args() {
    let (_, arguments) = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankBuilder)]
        pub enum Instructions {
            #[account(name="authority")]
            #[args(authority_id: u64)]
            NonIndexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(arguments.0.len(), 1, "includes one instruction argument");
    assert!(
        arguments.0.first().unwrap().name == "authority_id",
        "to instruction argument"
    );
}

#[test]
fn instruction_with_multiple_args() {
    let (_, arguments) = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankBuilder)]
        pub enum Instructions {
            #[account(name="authority")]
            #[args(id_1: u64)]
            #[args(id_2: u64)]
            #[args(id_3: u64)]
            NonIndexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(
        arguments.0.len(),
        3,
        "includes multuple instruction arguments"
    );
}

#[test]
fn instruction_without_args() {
    let (_, arguments) = parse_first_enum_variant_attrs(quote! {
        #[derive(ShankBuilder)]
        pub enum Instructions {
            #[account(name="authority")]
            NonIndexed
        }
    })
    .expect("Should parse fine");

    assert_eq!(arguments.0.len(), 0, "includes no instruction argument");
}
