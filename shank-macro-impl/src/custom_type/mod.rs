use proc_macro2::TokenStream;

mod custom_enum;
mod custom_struct;
mod custom_type_config;
pub use custom_enum::*;
pub use custom_struct::*;
pub use custom_type_config::*;

pub fn parse_custom_struct(item: TokenStream) -> CustomStruct {
    match syn::parse2::<CustomStruct>(item) {
        Ok(custom_struct) => custom_struct,
        Err(err) => panic!("{}", err),
    }
}

pub fn parse_custom_enum(item: TokenStream) -> CustomEnum {
    match syn::parse2::<CustomEnum>(item) {
        Ok(custom_enum) => custom_enum,
        Err(err) => panic!("{}", err),
    }
}
