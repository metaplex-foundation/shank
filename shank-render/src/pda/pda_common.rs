use shank_macro_impl::{
    parsed_struct::{ProcessedSeed, SeedArg, StructAttr, StructAttrs},
    syn::Result as ParseResult,
};

pub fn try_process_seeds(
    struct_attrs: &StructAttrs,
) -> ParseResult<Vec<ProcessedSeed>> {
    let all_seeds = struct_attrs
        .items_ref()
        .iter()
        .filter_map(|attr| match attr {
            StructAttr::Seeds(seeds) => Some(seeds),
            StructAttr::PodSentinel(_) => None,
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

pub fn render_args_comments(
    processed_seeds: &[ProcessedSeed],
    exclude_program_id: bool,
) -> Vec<String> {
    processed_seeds
        .iter()
        .map(|x| x.arg.as_ref())
        .filter(Option::is_some)
        .flatten()
        .filter(|x| !exclude_program_id || x.name != "program_id")
        .map(|SeedArg { name, desc, ty }| {
            format!("/// * **{}**: {} | [{}] ", name, desc, ty.ident)
        })
        .collect()
}
