use proc_macro2::TokenStream;
use quote::quote;

use shank_macro_impl::{
    account::extract_account_structs,
    syn::{self, ItemStruct},
};
use shank_render::try_render_seeds_fn;

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
    try_render_seeds_fn(struct_attrs)
        .expect("Should render seeds")
        .unwrap()
}

fn assert_rendered_seeds_fn(code: TokenStream, expected: TokenStream) {
    let rendered = render_seeds(code);
    assert_eq!(
        rendered.to_string().replace(" 'a", "'a"),
        expected.to_string().replace(" 'a", "'a")
    );
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
