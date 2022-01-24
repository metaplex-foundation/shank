use proc_macro2::TokenStream;

mod account;

pub use account::*;

#[cfg(test)]
mod account_test;

pub fn parse_account_struct(_item: TokenStream) -> AccountStruct {
    todo!()
    // match syn::parse2::<AccountStruct>(item) {
    //     Ok(syntax_tree) => syntax_tree,
    //     Err(_err) => todo!(),
    // }
}
