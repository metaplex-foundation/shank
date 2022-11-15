use proc_macro2::TokenStream;
use quote::quote;

use shank_macro_impl::{
    account::extract_account_structs,
    syn::{self, ItemStruct},
};
use shank_render::{try_process_seeds, try_render_seeds_fn};

// -----------------
// Integration Tests and Real World Examples
// -----------------

fn parse_struct(code: TokenStream) -> ItemStruct {
    syn::parse2::<ItemStruct>(code).expect("Should parse successfully")
}

fn render_seeds(code: TokenStream) -> TokenStream {
    let account_struct = parse_struct(code);
    let all_structs = vec![&account_struct].into_iter();
    let parsed_structs = extract_account_structs(all_structs)
        .expect("Should parse struct without error");

    let struct_attrs = &parsed_structs.first().unwrap().struct_attrs;
    let processed_seeds = try_process_seeds(struct_attrs)
        .expect("Should process seeds without error");
    try_render_seeds_fn(&processed_seeds, None)
        .expect("Should render seeds")
        .unwrap()
}

fn pretty_print(code: TokenStream) -> String {
    let syn_tree = syn::parse_file(code.to_string().as_str()).unwrap();
    prettyplease::unparse(&syn_tree)
}

#[allow(dead_code)]
fn render_and_dump(code: &TokenStream) {
    let rendered = render_seeds(code.clone());
    eprintln!("{}", pretty_print(rendered));
}

fn assert_rendered_seeds_fn(code: TokenStream, expected: TokenStream) {
    let rendered = render_seeds(code);
    assert_eq!(pretty_print(rendered), pretty_print(expected));
}

#[test]
fn literal_pubkeys_and_u8_byte() {
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
    assert_rendered_seeds_fn(
        code,
        quote! {
            pub fn account_seeds<'a>(
                program_id: &'a ::solana_program::pubkey::Pubkey,
                some_pubkey: &'a ::solana_program::pubkey::Pubkey,
                some_byte: &'a [u8; 1usize]
            ) -> [&'a [u8]; 4usize] {
                [
                    b"lit:prefix",
                    program_id.as_ref(),
                    some_pubkey.as_ref(),
                    some_byte
                ]
            }
        },
    );
}

#[test]
fn candy_guard_edition_marker_seeds() {
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

    assert_rendered_seeds_fn(
        code,
        quote! {
            pub fn account_seeds<'a>(
                prefix: &'a str,
                program_id: &'a ::solana_program::pubkey::Pubkey,
                master_edition_mint_info: &'a ::solana_program::account_info::AccountInfo,
                edition: &'a str,
                edition_marker_number: &'a String,
            ) -> [&'a [u8]; 5usize] {
                [
                    prefix.as_bytes(),
                    program_id.as_ref(),
                    master_edition_mint_info.as_ref(),
                    edition.as_bytes(),
                    edition_marker_number.as_bytes(),
                ]
            }
        },
    );
}

#[test]
fn candy_candy_guard_mint_limit_seeds() {
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

    assert_rendered_seeds_fn(
        code,
        quote! {
            pub fn account_seeds<'a>(
                id: &'a [u8; 1usize],
                user: &'a ::solana_program::pubkey::Pubkey,
                candy_guard_key: &'a ::solana_program::pubkey::Pubkey,
                candy_machine_key: &'a ::solana_program::pubkey::Pubkey,
            ) -> [&'a [u8]; 4usize] {
                [
                    id,
                    user.as_ref(),
                    candy_guard_key.as_ref(),
                    candy_machine_key.as_ref(),
                ]
            }
        },
    );
}
