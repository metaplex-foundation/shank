use proc_macro2::TokenStream;
use shank_macro_impl::{
    account::extract_account_structs,
    parsed_struct::{ProcessedSeed, StructAttrs},
    syn::{self, Ident, ItemStruct, Result as ParseResult},
};
use shank_render::pda::try_process_seeds;

fn parse_struct(code: TokenStream) -> ItemStruct {
    syn::parse2::<ItemStruct>(code).expect("Should parse successfully")
}

pub fn parse_struct_attrs(code: TokenStream) -> (Ident, StructAttrs) {
    let account_struct = parse_struct(code);
    let all_structs = vec![&account_struct].into_iter();
    let parsed_structs = extract_account_structs(all_structs)
        .expect("Should parse struct without error");

    (
        account_struct.ident,
        parsed_structs.first().unwrap().struct_attrs.clone(),
    )
}

pub fn process_seeds(code: TokenStream) -> ParseResult<Vec<ProcessedSeed>> {
    let (_, struct_attrs) = parse_struct_attrs(code);
    try_process_seeds(&struct_attrs)
}

pub fn pretty_print(code: TokenStream) -> String {
    prettyplease::unparse(&syn::parse2(code).unwrap())
}
