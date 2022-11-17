use std::convert::TryFrom;

use proc_macro2::TokenStream;
use shank_macro_impl::{
    parsed_struct::ParsedStruct, parsers::get_derive_attr, DERIVE_ACCOUNT_ATTR,
};
use syn::{DeriveInput, Error as ParseError, Item, Result as ParseResult};

pub fn derive_account(input: DeriveInput) -> ParseResult<TokenStream> {
    let attr = get_derive_attr(&input.attrs, DERIVE_ACCOUNT_ATTR).cloned();
    let item = Item::from(input);
    match item {
        Item::Struct(struct_item) => {
            let parsed_struct = ParsedStruct::try_from(&struct_item)?;
            shank_render::pda::render_pda_and_seeds_impl(
                &parsed_struct.struct_attrs,
                &parsed_struct.ident,
            )
        }
        _ => Err(ParseError::new_spanned(
            &attr,
            "ShankAccount can only be derived for structs",
        )),
    }
}
