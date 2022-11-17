use proc_macro2::{Span, TokenStream};
use quote::quote;
use shank_macro_impl::syn::Ident;
use shank_render::pda::render_pda_fn;

use crate::utils;

// -----------------
// Integration Tests and Real World Examples
// -----------------

fn render_pda(code: TokenStream) -> TokenStream {
    let processed_seeds =
        utils::process_seeds(code).expect("Should process seeds without error");
    render_pda_fn(
        &processed_seeds,
        &Ident::new("account_seeds", Span::call_site()),
        &Ident::new("account_pda", Span::call_site()),
    )
    .unwrap()
}

#[allow(unused)]
fn render_and_dump(code: &TokenStream) {
    let rendered = render_pda(code.clone());
    eprintln!("{}", utils::pretty_print(rendered));
}

fn assert_rendered_pda_fn(code: TokenStream, expected: TokenStream) {
    let rendered = render_pda(code);
    assert_eq!(utils::pretty_print(rendered), utils::pretty_print(expected));
}

// NOTE: the below tests use the same seeds as the ./render_seeds_fn.rs tests
#[test]
fn literal_pubkeys_and_u8_byte_pda() {
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
    render_and_dump(&code);
    assert_rendered_pda_fn(
        code,
        quote! {
            pub fn account_pda(
                program_id: &::solana_program::pubkey::Pubkey,
                some_pubkey: &::solana_program::pubkey::Pubkey,
                some_byte: u8,
            ) -> (::solana_program::pubkey::Pubkey, u8) {
                let some_byte_arg = &[some_byte];
                let seeds = Self::account_seeds(program_id, some_pubkey, some_byte_arg);
                ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
            }
        },
    );
}

#[test]
fn candy_guard_edition_marker_pda() {
    let code = quote! {
        #[derive(ShankAccount)]
        #[seeds(
            prefix("Prefix", str),
            program_id,
            master_edition_mint_info("Master Edition Mint Info", AccountInfo),
            edition("Edition", str),
            edition_marker_number("Edition Marker Number", String),
        )]
        struct CandyGuardEditionMarker {
            count: u8,
        }
    };
    assert_rendered_pda_fn(
        code,
        quote! {
            pub fn account_pda(
                program_id: &::solana_program::pubkey::Pubkey,
                prefix: &str,
                master_edition_mint_info: &::solana_program::account_info::AccountInfo,
                edition: &str,
                edition_marker_number: &String,
            ) -> (::solana_program::pubkey::Pubkey, u8) {
                let seeds = Self::account_seeds(
                    prefix,
                    program_id,
                    master_edition_mint_info,
                    edition,
                    edition_marker_number,
                );
                ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
            }
        },
    );
}

#[test]
fn candy_candy_guard_mint_limit_pda() {
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
    assert_rendered_pda_fn(
        code,
        quote! {
            pub fn account_pda(
                program_id: &::solana_program::pubkey::Pubkey,
                id: u8,
                user: &::solana_program::pubkey::Pubkey,
                candy_guard_key: &::solana_program::pubkey::Pubkey,
                candy_machine_key: &::solana_program::pubkey::Pubkey,
            ) -> (::solana_program::pubkey::Pubkey, u8) {
                let id_arg = &[id];
                let seeds = Self::account_seeds(id_arg, user, candy_guard_key, candy_machine_key);
                ::solana_program::pubkey::Pubkey::find_program_address(&seeds, program_id)
            }
        },
    );
}
