use proc_macro2::TokenStream;
use quote::quote;

pub fn solana_program_pubkey() -> TokenStream {
    quote! { ::solana_program::pubkey::Pubkey }
}
