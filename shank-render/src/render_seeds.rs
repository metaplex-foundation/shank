use proc_macro2::TokenStream;
use shank_macro_impl::{
    parsed_struct::{Seed, Seeds, StructAttr, StructAttrs},
    types::{RustType, TypeKind, Value},
};

pub fn render_seeds(struct_attrs: &StructAttrs) -> TokenStream {
    let all_seeds = struct_attrs
        .items_ref()
        .iter()
        .filter_map(|attr| match attr {
            StructAttr::Seeds(seeds) => Some(seeds),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(
        all_seeds.len() <= 1,
        "Should only have one seed per account"
    );

    if all_seeds.is_empty() {
        return TokenStream::new();
    }

    let seeds = all_seeds.first().unwrap();
    let args = function_args(seeds);
}
