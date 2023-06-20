use proc_macro2::TokenStream;
use shank_macro_impl::{
    builder::Builder, parsers::get_derive_attr, DERIVE_BUILDER_ATTR,
};
use syn::{DeriveInput, Error as ParseError, Item, Result as ParseResult};

pub fn derive_builder(input: DeriveInput) -> ParseResult<TokenStream> {
    let attr = get_derive_attr(&input.attrs, DERIVE_BUILDER_ATTR).cloned();
    let item = Item::from(input);
    match item {
        Item::Enum(enum_item) => {
            if let Some(context) =
                Builder::try_from_item_enum(&enum_item, true)?
            {
                shank_render::builder::render_builders_impl(&context)
            } else {
                Err(ParseError::new_spanned(
                    &attr,
                    "ShankBuilder can only be derived for enums with variants that have a `#[ShankBuilder]` attribute",
                ))
            }
        }
        _ => Err(ParseError::new_spanned(
            &attr,
            "ShankBuilder can only be derived for enums",
        )),
    }
}
