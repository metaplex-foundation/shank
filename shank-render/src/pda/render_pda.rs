use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use shank_macro_impl::{
    parsed_struct::{ProcessedSeed, Seed},
    syn::Ident,
};

use crate::consts::solana_program_pubkey;

use super::render_args_comments;

pub fn render_pda_fn(
    processed_seeds: &[ProcessedSeed],
    seeds_fn_name: &Ident,
    seeds_fn_with_bump_name: &Ident,
    pda_fn_name: &Ident,
    pda_fn_with_bump_name: &Ident,
    include_comments: bool,
) -> Option<TokenStream> {
    let RenderedPdaParts {
        seed_param_assigns,
        seed_fn_args,
        pda_fn_args,
    } = render_pda_parts(processed_seeds);
    if pda_fn_args.is_empty() {
        return None;
    }

    let pubkey = solana_program_pubkey();
    let seed_bump_arg = if seed_fn_args.is_empty() {
        quote! { bump_arg }
    } else {
        quote! { , bump_arg }
    };
    let (pda_comments, pda_with_bump_comments) = if include_comments {
        let args_comments = render_args_comments(processed_seeds, true);
        (
            format!(
                r#"
                /// Derives the PDA for this account.
                ///
                /// * **program_id**: The id of the program
                {}"#,
                args_comments.join("\n")
            )
            .to_token_stream(),
            format!(
                r#"
                /// Derives the PDA for this account allowing to provide a bump seed.
                ///
                /// * **program_id**: The id of the program
                {}
                /// * **bump**: the bump seed to pass when deriving the PDA"#,
                args_comments.join("\n")
            )
            .to_token_stream(),
        )
    } else {
        (TokenStream::new(), TokenStream::new())
    };

    Some(quote! {
        #pda_comments
        #[allow(unused)]
        pub fn #pda_fn_name(#(#pda_fn_args),*) -> (#pubkey, u8)  {
            #(#seed_param_assigns)*
            let seeds = Self::#seeds_fn_name(#(#seed_fn_args),*);
            #pubkey::find_program_address(&seeds, program_id)
        }
        #pda_with_bump_comments
        #[allow(unused)]
        pub fn #pda_fn_with_bump_name(#(#pda_fn_args),*, bump: u8) -> (#pubkey, u8)  {
            #(#seed_param_assigns)*
            let bump_arg = &[bump];
            let seeds = Self::#seeds_fn_with_bump_name(#(#seed_fn_args),* #seed_bump_arg);
            #pubkey::find_program_address(&seeds, program_id)
        }
    })
}

#[derive(Debug)]
struct RenderedPdaParts {
    seed_param_assigns: Vec<TokenStream>,
    seed_fn_args: Vec<Ident>,
    pda_fn_args: Vec<TokenStream>,
}

fn render_pda_parts(processed_seeds: &[ProcessedSeed]) -> RenderedPdaParts {
    // -----------------
    // Incoming Args
    // -----------------
    let mut pda_fn_args = processed_seeds
        .iter()
        .map(render_pda_function_arg)
        .collect::<Vec<Option<TokenStream>>>()
        .into_iter()
        .filter(Option::is_some)
        .flatten()
        .collect::<Vec<TokenStream>>();

    let pubkey_ty = solana_program_pubkey();
    let program_id_arg = quote! { program_id : &#pubkey_ty };
    pda_fn_args.insert(0, program_id_arg);

    // -----------------
    // Args to get seeds array
    // -----------------
    let mut seed_param_assigns = Vec::new();
    let mut seed_fn_args = Vec::new();

    for seed in processed_seeds {
        let (reassign, arg) = render_seed_param(seed);
        if let Some(reassign) = reassign {
            seed_param_assigns.push(reassign);
        }
        if let Some(arg) = arg {
            seed_fn_args.push(arg);
        }
    }

    RenderedPdaParts {
        pda_fn_args,
        seed_param_assigns,
        seed_fn_args,
    }
}

fn render_pda_function_arg(seed: &ProcessedSeed) -> Option<TokenStream> {
    match &seed.seed {
        Seed::Literal(_) => {
            // Literal items don't need to be passed to the function
            None
        }
        Seed::ProgramId => {
            // Since `Pubkey::find_program_address` depends on program_id, we always
            // render as the first argument of the function
            None
        }
        Seed::Param(name, _, _) => {
            // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
            // that the arg is set
            let ty = seed.arg.as_ref().unwrap().ty.clone();
            let arg = ty.render_param(name);
            Some(arg)
        }
    }
}

/// Determines if the arg to the PDA fn needs to be reassigned or if it can be passed directly to
/// the seed fn when getting the seeds array.
/// Returns a tuple of optional reassignment and the ident of the arg we need to pass.
/// Specifically:
///   - it filters out literals which don't need to be passed
///   - it wraps u8s in a &[u8] and passes that reference to the seed fn
fn render_seed_param(
    seed: &ProcessedSeed,
) -> (Option<TokenStream>, Option<Ident>) {
    match &seed.seed {
        Seed::Literal(_) => (None, None),
        Seed::ProgramId => {
            (None, Some(Ident::new("program_id", Span::call_site())))
        }
        Seed::Param(name, _, seed_ty) => {
            let ident =
                Ident::new(name.as_str(), proc_macro2::Span::call_site());
            match &seed_ty {
                Some(ty) if ty == "u8" => {
                    // We pass a byte array ref (&[u8]) to the seed function and need to assign it so
                    // it lives long enough to be included in the seeds array used to calculate the PDA
                    let ident_arg = Ident::new(
                        format!("{}_arg", name).as_str(),
                        Span::call_site(),
                    );
                    (
                        Some(quote! { let #ident_arg = &[#ident]; }),
                        Some(ident_arg),
                    )
                }
                _ => (None, Some(ident)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use shank_macro_impl::parsed_struct::Seeds;

    use super::*;

    fn process_and_render_pda_parts(seeds: &[Seed]) -> RenderedPdaParts {
        let processed_seeds = Seeds(seeds.to_vec())
            .process()
            .expect("should process seeds without error");
        render_pda_parts(&processed_seeds)
    }

    fn assert_tokenstream_eq(actual: &TokenStream, expected: &str) {
        let expected_ts = expected.parse::<TokenStream>().unwrap().to_string();
        assert_eq!(actual.to_string(), expected_ts);
    }

    fn assert_program_id_arg(tokens: &TokenStream) {
        assert_tokenstream_eq(
            tokens,
            "program_id : & ::solana_program::pubkey::Pubkey",
        );
    }

    fn assert_ident(ident: &Ident, expected: &str) {
        assert_eq!(ident.to_string(), expected);
    }

    #[test]
    fn process_pda_literal() {
        let seed = Seed::Literal("uno".to_string());
        let RenderedPdaParts {
            pda_fn_args,
            seed_param_assigns,
            seed_fn_args,
        } = process_and_render_pda_parts(&[seed]);

        // Takes program id arg
        assert_eq!(pda_fn_args.len(), 1);
        assert_program_id_arg(&pda_fn_args[0]);

        // Does not pass along program id
        assert_eq!(seed_param_assigns.len(), 0);
        assert_eq!(seed_fn_args.len(), 0);
    }

    #[test]
    fn process_pda_program_id() {
        let seed = Seed::ProgramId;
        let RenderedPdaParts {
            pda_fn_args,
            seed_param_assigns,
            seed_fn_args,
        } = process_and_render_pda_parts(&[seed]);

        // Takes program id arg
        assert_eq!(pda_fn_args.len(), 1);
        assert_program_id_arg(&pda_fn_args[0]);

        // Passes along program id
        assert_eq!(seed_param_assigns.len(), 0);
        assert_eq!(seed_fn_args.len(), 1);
        assert_ident(&seed_fn_args[0], "program_id");
    }

    #[test]
    fn process_pda_custom_pubkey() {
        let seed =
            Seed::Param("owner".to_string(), "The owner".to_string(), None);
        let RenderedPdaParts {
            pda_fn_args,
            seed_param_assigns,
            seed_fn_args,
        } = process_and_render_pda_parts(&[seed]);

        // Takes program id arg and owner
        assert_eq!(pda_fn_args.len(), 2);
        assert_program_id_arg(&pda_fn_args[0]);
        assert_tokenstream_eq(
            &pda_fn_args[1],
            "owner : & ::solana_program::pubkey::Pubkey",
        );

        // Passes along owner only
        assert_eq!(seed_param_assigns.len(), 0);
        assert_eq!(seed_fn_args.len(), 1);
        assert_ident(&seed_fn_args[0], "owner");
    }

    #[test]
    fn process_pda_u8() {
        let seed = Seed::Param(
            "count".to_string(),
            "The count".to_string(),
            Some("u8".to_string()),
        );
        let RenderedPdaParts {
            pda_fn_args,
            seed_param_assigns,
            seed_fn_args,
        } = process_and_render_pda_parts(&[seed]);

        // Takes program id arg and count (NOTE it doesn't take it as a reference, i.e. not &u8)
        // See: ProccessedSeed::TryFrom<Seed>
        assert_eq!(pda_fn_args.len(), 2);
        assert_program_id_arg(&pda_fn_args[0]);
        assert_tokenstream_eq(&pda_fn_args[1], "count : u8");

        // Wraps count in a byte array ref and passes that along
        assert_eq!(seed_param_assigns.len(), 1);
        assert_tokenstream_eq(
            &seed_param_assigns[0],
            "let count_arg = &[count];",
        );
        assert_eq!(seed_fn_args.len(), 1);
        assert_ident(&seed_fn_args[0], "count_arg");
    }
}
