mod custom_struct;
pub use custom_struct::*;
use proc_macro2::TokenStream;

pub fn parse_custom_struct(item: TokenStream) -> CustomStruct {
    match syn::parse2::<CustomStruct>(item) {
        Ok(account_struct) => account_struct,
        Err(err) => panic!("{}", err),
    }
}
