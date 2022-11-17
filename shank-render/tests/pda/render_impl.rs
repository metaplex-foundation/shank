use proc_macro2::TokenStream;
use quote::quote;
use shank_render::pda::render_pda_and_seeds_impl;

use crate::utils;

// -----------------
// Integration Tests and Real World Examples
// -----------------

fn render_impl(code: TokenStream, include_comments: bool) -> TokenStream {
    let (struct_ident, struct_attrs) = utils::parse_struct_attrs(code);
    render_pda_and_seeds_impl(&struct_attrs, &struct_ident, include_comments)
        .unwrap()
}

#[allow(unused)]
fn render_and_dump(code: &TokenStream) {
    let rendered = render_impl(code.clone(), false);
    eprintln!("{}", utils::pretty_print(rendered));
}

#[allow(unused)]
fn render_and_dump_commented(code: &TokenStream) {
    let rendered = render_impl(code.clone(), true);
    eprintln!("{}", utils::pretty_print(rendered));
}

fn assert_rendered_impl_fn(code: TokenStream, expected: TokenStream) {
    let rendered = render_impl(code, false);
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
            impl AccountStructWithSeed {
                #[allow(unused, clippy::needless_lifetimes)]
                pub fn shank_seeds<'a>(
                    program_id: &'a ::solana_program::pubkey::Pubkey,
                    some_pubkey: &'a ::solana_program::pubkey::Pubkey,
                    some_byte: &'a [u8; 1usize],
                ) -> [&'a [u8]; 4usize] {
                    [b"lit:prefix", program_id.as_ref(), some_pubkey.as_ref(), some_byte]
                }
                #[allow(unused, clippy::needless_lifetimes)]
                pub fn shank_seeds_with_bump<'a>(
                    program_id: &'a ::solana_program::pubkey::Pubkey,
                    some_pubkey: &'a ::solana_program::pubkey::Pubkey,
                    some_byte: &'a [u8; 1usize],
                    bump: &'a [u8; 1],
                ) -> [&'a [u8]; 5usize] {
                    [b"lit:prefix", program_id.as_ref(), some_pubkey.as_ref(), some_byte, bump]
                }
                #[allow(unused)]
                pub fn shank_pda(
                    program_id: &::solana_program::pubkey::Pubkey,
                    some_pubkey: &::solana_program::pubkey::Pubkey,
                    some_byte: u8,
                ) -> (::solana_program::pubkey::Pubkey, u8) {
                    let some_byte_arg = &[some_byte];
                    let seeds = Self::shank_seeds(program_id, some_pubkey, some_byte_arg);
                    ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
                }
                #[allow(unused)]
                pub fn shank_pda_with_bump(
                    program_id: &::solana_program::pubkey::Pubkey,
                    some_pubkey: &::solana_program::pubkey::Pubkey,
                    some_byte: u8,
                    bump: u8,
                ) -> (::solana_program::pubkey::Pubkey, u8) {
                    let some_byte_arg = &[some_byte];
                    let bump_arg = &[bump];
                    let seeds = Self::shank_seeds_with_bump(
                        program_id,
                        some_pubkey,
                        some_byte_arg,
                        bump_arg,
                    );
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
        struct CandyGuardMintLimit {
            count: u8,
        }
    };
    assert_rendered_impl_fn(
        code,
        quote! {
            impl CandyGuardMintLimit {
                #[allow(unused, clippy::needless_lifetimes)]
                pub fn shank_seeds<'a>(
                    id: &'a [u8; 1usize],
                    user: &'a ::solana_program::pubkey::Pubkey,
                    candy_guard_key: &'a ::solana_program::pubkey::Pubkey,
                    candy_machine_key: &'a ::solana_program::pubkey::Pubkey,
                ) -> [&'a [u8]; 4usize] {
                    [id, user.as_ref(), candy_guard_key.as_ref(), candy_machine_key.as_ref()]
                }
                #[allow(unused, clippy::needless_lifetimes)]
                pub fn shank_seeds_with_bump<'a>(
                    id: &'a [u8; 1usize],
                    user: &'a ::solana_program::pubkey::Pubkey,
                    candy_guard_key: &'a ::solana_program::pubkey::Pubkey,
                    candy_machine_key: &'a ::solana_program::pubkey::Pubkey,
                    bump: &'a [u8; 1],
                ) -> [&'a [u8]; 5usize] {
                    [id, user.as_ref(), candy_guard_key.as_ref(), candy_machine_key.as_ref(), bump]
                }
                #[allow(unused)]
                pub fn shank_pda(
                    program_id: &::solana_program::pubkey::Pubkey,
                    id: u8,
                    user: &::solana_program::pubkey::Pubkey,
                    candy_guard_key: &::solana_program::pubkey::Pubkey,
                    candy_machine_key: &::solana_program::pubkey::Pubkey,
                ) -> (::solana_program::pubkey::Pubkey, u8) {
                    let id_arg = &[id];
                    let seeds = Self::shank_seeds(id_arg, user, candy_guard_key, candy_machine_key);
                    ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
                }
                #[allow(unused)]
                pub fn shank_pda_with_bump(
                    program_id: &::solana_program::pubkey::Pubkey,
                    id: u8,
                    user: &::solana_program::pubkey::Pubkey,
                    candy_guard_key: &::solana_program::pubkey::Pubkey,
                    candy_machine_key: &::solana_program::pubkey::Pubkey,
                    bump: u8,
                ) -> (::solana_program::pubkey::Pubkey, u8) {
                    let id_arg = &[id];
                    let bump_arg = &[bump];
                    let seeds = Self::shank_seeds_with_bump(
                        id_arg,
                        user,
                        candy_guard_key,
                        candy_machine_key,
                        bump_arg,
                    );
                    ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
                }
            }
        },
    )
}

// -----------------
// Including Comments
// -----------------

// NOTE: once comments are involved it is very brittle to compare rendered code
//       thus this test only exists to allow uncommenting dumping the rendered code
// #[test]
#[allow(unused)]
fn literal_pubkeys_and_u8_byte_impl_commented() {
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
    render_and_dump_commented(&code);
}

// -----------------
// Edge Cases
// -----------------
#[test]
fn struct_without_seeds_impl() {
    let code = quote! {
        #[derive(ShankAccount)]
        struct SomeAccount {
            count: u8,
        }
    };
    assert_rendered_impl_fn(code, TokenStream::new());
}

#[test]
fn struct_with_empty_seeds_impl() {
    let code = quote! {
        #[derive(ShankAccount)]
        #[seeds()]
        struct SomeAccount {
            count: u8,
        }
    };
    assert_rendered_impl_fn(code, TokenStream::new());
}

#[test]
fn struct_with_one_seed_impl() {
    let code = quote! {
        #[derive(ShankAccount)]
        #[seeds("lit:prefix")]
        struct SomeAccount {
            count: u8,
        }
    };
    assert_rendered_impl_fn(
        code,
        quote! {
            impl SomeAccount {
                #[allow(unused, clippy::needless_lifetimes)]
                pub fn shank_seeds<'a>() -> [&'a [u8]; 1usize] {
                    [b"lit:prefix"]
                }
                #[allow(unused, clippy::needless_lifetimes)]
                pub fn shank_seeds_with_bump<'a>(bump: &'a [u8; 1]) -> [&'a [u8]; 2usize] {
                    [b"lit:prefix", bump]
                }
                #[allow(unused)]
                pub fn shank_pda(
                    program_id: &::solana_program::pubkey::Pubkey,
                ) -> (::solana_program::pubkey::Pubkey, u8) {
                    let seeds = Self::shank_seeds();
                    ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
                }
                #[allow(unused)]
                pub fn shank_pda_with_bump(
                    program_id: &::solana_program::pubkey::Pubkey,
                    bump: u8,
                ) -> (::solana_program::pubkey::Pubkey, u8) {
                    let bump_arg = &[bump];
                    let seeds = Self::shank_seeds_with_bump(bump_arg);
                    ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
                }
            }
        },
    );
}
