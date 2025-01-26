use proc_macro2::TokenStream;
use quote::quote;

pub fn program_pubkey_type() -> TokenStream {
    if cfg!(feature = "pinocchio") {
        quote! { pinocchio::pubkey::Pubkey }
    } else {
        quote! { ::solana_program::pubkey::Pubkey }
    }
}
