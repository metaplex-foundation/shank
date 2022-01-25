use proc_macro2::TokenStream;

mod account;

pub use account::*;

#[cfg(test)]
mod account_test;

pub fn parse_account_struct(item: TokenStream) -> AccountStruct {
    match syn::parse2::<AccountStruct>(item) {
        Ok(account_struct) => account_struct,
        Err(err) => panic!("{}", err),
    }
}
