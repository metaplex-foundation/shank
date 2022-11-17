use shank_macro_impl::{
    parsed_struct::{ProcessedSeed, StructAttr, StructAttrs},
    syn::Result as ParseResult,
};

pub fn try_process_seeds(
    struct_attrs: &StructAttrs,
) -> ParseResult<Vec<ProcessedSeed>> {
    let all_seeds = struct_attrs
        .items_ref()
        .iter()
        .map(|attr| match attr {
            StructAttr::Seeds(seeds) => seeds,
        })
        .collect::<Vec<_>>();

    assert!(
        all_seeds.len() <= 1,
        "Should only have one seed definition per account"
    );

    if all_seeds.is_empty() {
        Ok(vec![])
    } else {
        let seeds = all_seeds.first().unwrap();
        seeds.process()
    }
}
