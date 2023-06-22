use proc_macro2::TokenStream;
use shank_macro_impl::{
    instruction::Instruction, parsers::get_derive_attr, DERIVE_CONTEXT_ATTR,
};
use syn::{DeriveInput, Error as ParseError, Item, Result as ParseResult};

pub fn derive_context(input: DeriveInput) -> ParseResult<TokenStream> {
    let attr = get_derive_attr(&input.attrs, DERIVE_CONTEXT_ATTR).cloned();
    let item = Item::from(input);
    match item {
        Item::Enum(enum_item) => {
            if let Some(instruction) =
                Instruction::try_from_item_enum(&enum_item, true)?
            {
                shank_render::context::render_contexts_impl(&instruction)
            } else {
                Err(ParseError::new_spanned(
                    &attr,
                    "ShankContext can only be derived for enums with variants that have a `#[ShankContext]` attribute",
                ))
            }
        }
        _ => Err(ParseError::new_spanned(
            &attr,
            "ShankContext can only be derived for enums",
        )),
    }
}
