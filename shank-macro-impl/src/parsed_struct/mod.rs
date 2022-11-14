use proc_macro2::TokenStream;

#[allow(clippy::module_inception)]
mod parsed_struct;
mod seed;
mod struct_attr;
mod struct_field_attr;

pub use parsed_struct::*;
pub use seed::*;
pub use struct_attr::*;
pub use struct_field_attr::StructFieldAttr;

#[cfg(test)]
mod parsed_struct_test;

pub fn parse_struct(item: TokenStream) -> ParsedStruct {
    match syn::parse2::<ParsedStruct>(item) {
        Ok(account_struct) => account_struct,
        Err(err) => panic!("{}", err),
    }
}
