use proc_macro2::TokenStream;

mod parsed_struct;

pub use parsed_struct::*;

#[cfg(test)]
mod parsed_struct_test;

pub fn parse_struct(item: TokenStream) -> ParsedStruct {
    match syn::parse2::<ParsedStruct>(item) {
        Ok(account_struct) => account_struct,
        Err(err) => panic!("{}", err),
    }
}
