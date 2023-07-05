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
        /// Convenience function for accessing the next item in an [`AccountInfo`]
        /// iterator and validating whether the account is present or not.
        ///
        /// This relies on the client setting the `crate::ID` as the pubkey for
        /// accounts that are not set, which effectively allows us to use positional
        /// optional accounts.
        fn next_optional_account_info<'b, 'c, I: Iterator<Item = &'b solana_program::account_info::AccountInfo<'c>>>(
            iter: &mut I,
        ) -> Result<Option<I::Item>, solana_program::program_error::ProgramError> {
            let account_info = iter.next().ok_or(solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;

            Ok(if *account_info.key == crate::ID {
                None
            } else {
                Some(account_info)
            })
        }

        pub struct Context<'a, T> {
            pub accounts: T,
            pub remaining_accounts: Vec<&'a solana_program::account_info::AccountInfo<'a>>,
        }

        #(#contexts)*
    })
}
