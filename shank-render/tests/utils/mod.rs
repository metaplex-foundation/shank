use proc_macro2::TokenStream;
use shank_macro_impl::{
    account::extract_account_structs,
    parsed_struct::ProcessedSeed,
    syn::{self, ItemStruct, Result as ParseResult},
};
use shank_render::try_process_seeds;

fn parse_struct(code: TokenStream) -> ItemStruct {
    syn::parse2::<ItemStruct>(code).expect("Should parse successfully")
}

pub fn process_seeds(code: TokenStream) -> ParseResult<Vec<ProcessedSeed>> {
    let account_struct = parse_struct(code);
    let all_structs = vec![&account_struct].into_iter();
    let parsed_structs = extract_account_structs(all_structs)
        .expect("Should parse struct without error");

    let struct_attrs = &parsed_structs.first().unwrap().struct_attrs;
    try_process_seeds(struct_attrs)
}

pub fn pretty_print(code: TokenStream) -> String {
    let syn_tree = syn::parse_file(code.to_string().as_str()).unwrap();
    prettyplease::unparse(&syn_tree)
}
