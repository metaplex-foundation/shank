use proc_macro2::TokenStream;
use shank_macro_impl::{
    instruction::Instruction, parsers::get_derive_attr, DERIVE_INSTRUCTION_ATTR,
};
use syn::{DeriveInput, Error as ParseError, Item, Result as ParseResult};

pub fn derive_instruction(input: DeriveInput) -> ParseResult<TokenStream> {
    let attr = get_derive_attr(&input.attrs, DERIVE_INSTRUCTION_ATTR).cloned();
    let item = Item::from(input);
    match item {
        Item::Enum(enum_item) => {
            Instruction::try_from_item_enum(&enum_item, true)
                .map(|_| TokenStream::new())
        }
        _ => Err(ParseError::new_spanned(
            &attr,
            "ShankInstruction can only be derived for enums",
        )),
    }
}
