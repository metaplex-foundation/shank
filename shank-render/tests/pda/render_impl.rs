use proc_macro2::{Span, TokenStream};
use quote::quote;
use shank_macro_impl::syn::Ident;
use shank_render::pda::render_pda_and_seeds_impl;

use crate::utils;

// -----------------
// Integration Tests and Real World Examples
// -----------------

fn render_impl(code: TokenStream) -> TokenStream {
    let struct_attrs = utils::parse_struct_attrs(code);
    render_pda_and_seeds_impl(
        &struct_attrs,
        &Ident::new("MyAccount", Span::call_site()),
    )
    .unwrap()
}

#[allow(unused)]
fn render_and_dump(code: &TokenStream) {
    let rendered = render_impl(code.clone());
    eprintln!("{}", utils::pretty_print(rendered));
}

fn assert_rendered_impl_fn(code: TokenStream, expected: TokenStream) {
    let rendered = render_impl(code);
    assert_eq!(utils::pretty_print(rendered), utils::pretty_print(expected));
}

// NOTE: the below tests use the same seeds as the ./render_seeds_fn.rs tests
#[test]
fn literal_pubkeys_and_u8_byte_impl() {
    let code = quote! {
        #[derive(ShankAccount)]
        #[seeds(
            /* literal    */ "lit:prefix",
            /* program_id */ program_id,
            /* pubkey     */ some_pubkey("description of some pubkey"),
            /* byte       */ some_byte("description of byte", u8),
        )]
        struct AccountStructWithSeed {
            count: u8,
        }
    };
    assert_rendered_impl_fn(
        code,
        quote! {
            impl MyAccount {
                pub fn account_seeds<'a>(
                    program_id: &'a ::solana_program::pubkey::Pubkey,
                    some_pubkey: &'a ::solana_program::pubkey::Pubkey,
                    some_byte: &'a [u8; 1usize],
                ) -> [&'a [u8]; 4usize] {
                    [b"lit:prefix", program_id.as_ref(), some_pubkey.as_ref(), some_byte]
                }
                pub fn account_pda(
                    program_id: &::solana_program::pubkey::Pubkey,
                    some_pubkey: &::solana_program::pubkey::Pubkey,
                    some_byte: u8,
                ) -> (::solana_program::pubkey::Pubkey, u8) {
                    let some_byte_arg = &[some_byte];
                    let seeds = Self::account_seeds(program_id, some_pubkey, some_byte_arg);
                    ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
                }
            }
        },
    )
}

#[test]
fn candy_guard_mint_limit_impl() {
    let code = quote! {
        #[derive(ShankAccount)]
        #[seeds(
            id("Guard Id", u8),
            user("The User Pubkey"),
            candy_guard_key("Candy Guard Key", Pubkey),
            candy_machine_key("Candy Machine Key"),
        )]
        struct CandyGuardMintLimitSeeds {
            count: u8,
        }
    };
    render_and_dump(&code);
    assert_rendered_impl_fn(
        code,
        quote! {
            impl MyAccount {
                pub fn account_seeds<'a>(
                    id: &'a [u8; 1usize],
                    user: &'a ::solana_program::pubkey::Pubkey,
                    candy_guard_key: &'a ::solana_program::pubkey::Pubkey,
                    candy_machine_key: &'a ::solana_program::pubkey::Pubkey,
                ) -> [&'a [u8]; 4usize] {
                    [id, user.as_ref(), candy_guard_key.as_ref(), candy_machine_key.as_ref()]
                }
                pub fn account_pda(
                    program_id: &::solana_program::pubkey::Pubkey,
                    id: u8,
                    user: &::solana_program::pubkey::Pubkey,
                    candy_guard_key: &::solana_program::pubkey::Pubkey,
                    candy_machine_key: &::solana_program::pubkey::Pubkey,
                ) -> (::solana_program::pubkey::Pubkey, u8) {
                    let id_arg = &[id];
                    let seeds = Self::account_seeds(
                        id_arg,
                        user,
                        candy_guard_key,
                        candy_machine_key,
                    );
                    ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
                }
            }
        },
    )
}
