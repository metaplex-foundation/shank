use proc_macro2::TokenStream;
use quote::quote;
use shank_macro_impl::instruction::Instruction;
use shank_macro_impl::syn::Result as ParseResult;

mod render_context;
use self::render_context::generate_context;

pub fn render_contexts_impl(
    instruction: &Instruction,
) -> ParseResult<TokenStream> {
    let contexts = instruction
        .variants
        .iter()
        .map(generate_context)
        .collect::<Vec<TokenStream>>();

    Ok(quote! {
        pub mod accounts {
            use super::*;

            pub struct Context<'a, T> {
                pub accounts: T,
                pub remaining_accounts: &'a [solana_program::account_info::AccountInfo<'a>],
            }

            #(#contexts)*
        }
    })
}
