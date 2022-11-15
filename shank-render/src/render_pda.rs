#![allow(unused)]
use proc_macro2::TokenStream;
use quote::quote;
use shank_macro_impl::{
    parsed_struct::{ProcessedSeed, Seed, StructAttrs},
    syn::{parse2, Result as ParseResult},
};

use crate::consts::solana_program_pubkey;

#[derive(Debug)]
struct RenderedPdaParts {
    // seed_param_assigns: Vec<TokenStream>,
    pda_fn_args: Vec<TokenStream>,
}

fn try_render_pda_parts(
    processed_seeds: &[ProcessedSeed],
) -> ParseResult<RenderedPdaParts> {
    let mut pda_fn_args = processed_seeds
        .iter()
        .map(render_pda_function_arg)
        .collect::<ParseResult<Vec<Option<TokenStream>>>>()?
        .into_iter()
        .filter(Option::is_some)
        .flatten()
        .collect::<Vec<TokenStream>>();

    let pubkey_ty = solana_program_pubkey();
    let program_id_arg = quote! { program_id : &#pubkey_ty };
    pda_fn_args.insert(0, program_id_arg);

    Ok(RenderedPdaParts { pda_fn_args })
}

fn render_pda_function_arg(
    seed: &ProcessedSeed,
) -> ParseResult<Option<TokenStream>> {
    match &seed.seed {
        Seed::Literal(_) => {
            // Literal items don't need to be passed to the function
            Ok(None)
        }
        Seed::ProgramId => {
            // Since `Pubkey::find_program_address` depends on program_id, we always
            // render as the first argument of the function
            Ok(None)
        }
        Seed::Param(name, _, _) => {
            // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
            // that the arg is set
            let ty = seed.arg.as_ref().unwrap().ty.clone();
            let arg = ty.render_param(name);
            Ok(Some(arg))
        }
    }
}
#[cfg(test)]
mod tests {
    use shank_macro_impl::parsed_struct::Seeds;

    use super::*;

    fn render_pda_parts(seeds: &[Seed]) -> RenderedPdaParts {
        let processed_seeds = Seeds(seeds.to_vec())
            .process()
            .expect("should process seeds without error");
        try_render_pda_parts(&processed_seeds)
            .expect("Should render pda without error")
    }

    fn assert_tokenstream_eq(actual: &TokenStream, expected: &str) {
        let expected_ts = expected.parse::<TokenStream>().unwrap().to_string();
        assert_eq!(actual.to_string(), expected_ts);
    }

    #[test]
    fn render_pda_literal() {
        let seed = Seed::Literal("uno".to_string());
        let RenderedPdaParts { pda_fn_args } = render_pda_parts(&[seed]);

        assert_eq!(pda_fn_args.len(), 1);
        assert_tokenstream_eq(
            &pda_fn_args[0],
            "program_id : & ::solana_program::pubkey::Pubkey",
        );
    }
}
