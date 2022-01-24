/// NOTE: from https://github.com/project-serum/anchor/blob/1a2fd38451b36a569287eb9794ec10e51675789e/lang/syn/src/lib.rs
/// without the contstraints fields
mod account_field;
mod account_field_ty;
mod accounts;
mod common;
mod parsed_accounts;

pub use account_field::*;
pub use account_field_ty::*;
pub use accounts::*;
pub use parsed_accounts::*;
use proc_macro2::TokenStream;

#[cfg(test)]
mod accounts_test;

pub fn parse_accounts_struct(item: TokenStream) -> AccountsStruct {
    match syn::parse2::<AccountsStruct>(item) {
        Ok(syntax_tree) => syntax_tree,
        Err(_err) => todo!(), // TokenStream::from(err.to_compile_error()),
    }
}
