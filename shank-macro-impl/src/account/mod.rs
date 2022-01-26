use proc_macro2::TokenStream;

mod account;
mod extract_accounts;

pub use account::*;
pub use extract_accounts::*;

#[cfg(test)]
mod account_test;

pub fn parse_account_struct(item: TokenStream) -> AccountStruct {
    match syn::parse2::<AccountStruct>(item) {
        Ok(account_struct) => account_struct,
        Err(err) => panic!("{}", err),
    }
}
