use quote::{quote, ToTokens};
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use shank_macro_impl::{
    parsed_struct::{
        ProcessedSeed, Seed, ACCOUNT_INFO_TY, FULL_ACCOUNT_INFO_TY,
        FULL_PUBKEY_TY, PUBKEY_TY,
    },
    syn::{Error as ParseError, Result as ParseResult},
    types::{Composite, ParsedReference, Primitive, RustType, TypeKind, Value},
};

use super::render_args_comments;

pub fn try_render_seeds_fn(
    processed_seeds: &[ProcessedSeed],
    seeds_fn_name: &Ident,
    seeds_fn_with_bump_name: &Ident,
    include_comments: bool,
) -> ParseResult<Option<TokenStream>> {
    let lifetime = "a";
    let RenderedSeedsParts {
        seed_array_items,
        seed_fn_args,
    } = try_render_seeds_parts(processed_seeds, lifetime)?;
    if seed_array_items.is_empty() {
        return Ok(None);
    }

    let len = seed_array_items.len();
    let lifetime_toks = format!("<'{}>", lifetime).parse::<TokenStream>()?;
    let len_with_bump = len + 1;
    let bump = if seed_fn_args.is_empty() {
        quote! { bump: &'a [u8; 1] }
    } else {
        quote! { , bump: &'a [u8; 1] }
    };

    let (seeds_comments, seeds_with_bump_comments) = if include_comments {
        let args_comments = render_args_comments(processed_seeds, false);
        (
            format!(
                r#"
                /// Derives the seeds for this account.
                ///
                {}"#,
                args_comments.join("\n")
            )
            .to_token_stream(),
            format!(
                r#"
                /// Derives the seeds for this account allowing to provide a bump seed.
                ///
                {}
                /// * **bump**: the bump seed to pass when deriving the PDA"#,
                args_comments.join("\n")
            )
            .to_token_stream(),
        )
    } else {
        (TokenStream::new(), TokenStream::new())
    };
    Ok(Some(quote! {
        #seeds_comments
        #[allow(unused, clippy::needless_lifetimes)]
        pub fn #seeds_fn_name#lifetime_toks(#(#seed_fn_args),*) -> [&'a [u8]; #len] {
            [#(#seed_array_items),*]
        }
        #seeds_with_bump_comments
        #[allow(unused, clippy::needless_lifetimes)]
        pub fn #seeds_fn_with_bump_name#lifetime_toks(#(#seed_fn_args),*#bump) -> [&'a [u8]; #len_with_bump] {
            [#(#seed_array_items),*, bump]
        }
    }))
}

#[derive(Debug)]
struct RenderedSeedsParts {
    seed_array_items: Vec<TokenStream>,
    seed_fn_args: Vec<TokenStream>,
}

fn try_render_seeds_parts(
    processed_seeds: &[ProcessedSeed],
    lifetime: &str,
) -> ParseResult<RenderedSeedsParts> {
    let seed_fn_args = processed_seeds
        .iter()
        .map(|x| render_seed_function_arg(x, lifetime))
        .collect::<ParseResult<Vec<Option<TokenStream>>>>()?
        .into_iter()
        .filter(Option::is_some)
        .flatten()
        .collect::<Vec<TokenStream>>();

    let seed_array_items = processed_seeds
        .iter()
        .map(render_seed_array_item)
        .collect::<ParseResult<Vec<TokenStream>>>()?
        .into_iter()
        .collect::<Vec<TokenStream>>();

    Ok(RenderedSeedsParts {
        seed_fn_args,
        seed_array_items,
    })
}

// -----------------
// Seed Function Args
// -----------------
fn render_seed_function_arg(
    seed: &ProcessedSeed,
    lifetime: &str,
) -> ParseResult<Option<TokenStream>> {
    match &seed.seed {
        Seed::Literal(_) => {
            // Literal items don't need to be passed to the function
            Ok(None)
        }
        Seed::ProgramId => {
            let arg = seed
                .arg
                .as_ref()
                // SAFETY: we can unwrap here since we control creation of this data
                // and know that for `Seed::ProgramId` the type arg is always set
                .unwrap()
                .ty
                .try_with_lifetime(lifetime)?
                .render_param("program_id");
            Ok(Some(arg))
        }
        Seed::Param(name, _, _) => {
            // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
            // that the arg is set
            let ty = seed.arg.as_ref().unwrap().ty.clone();
            let arg = adapt_seed_function_arg_type_kind(ty)
                .try_with_lifetime(lifetime)?
                .render_param(name);
            Ok(Some(arg))
        }
    }
}

fn adapt_seed_function_arg_type_kind(ty: RustType) -> RustType {
    match ty.kind {
        TypeKind::Primitive(Primitive::U8) => {
            let kind = TypeKind::Composite(
                Composite::Array(1),
                vec![ty.clone().as_owned()],
            );
            RustType {
                kind,
                reference: ParsedReference::Ref(None),
                ..ty
            }
        }
        // TODO(thlorenz): most of the below are not supported and some already error when seeds
        // are processed. We could add some more check here to detect invalid inputs.
        TypeKind::Primitive(_) => ty,
        TypeKind::Value(_) => ty,
        TypeKind::Composite(_, _) => ty,
        TypeKind::Unit => ty,
        TypeKind::Unknown => ty,
    }
}

// -----------------
// Seed Array Items
// -----------------
fn render_seed_array_item(seed: &ProcessedSeed) -> ParseResult<TokenStream> {
    match &seed.seed {
        Seed::Literal(lit) => {
            let item = TokenStream::from_str(&format!("b\"{}\"", lit))?;
            Ok(item)
        }
        Seed::ProgramId => {
            let item =
                seed_array_item("program_id", &seed.arg.as_ref().unwrap().ty)?;
            Ok(item)
        }
        Seed::Param(name, _, _) => {
            // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
            // that the arg is set
            let item =
                seed_array_item(name.as_str(), &seed.arg.as_ref().unwrap().ty)?;
            Ok(item)
        }
    }
}

fn seed_array_item(name: &str, ty: &RustType) -> ParseResult<TokenStream> {
    let ident = Ident::new(name, Span::call_site());
    match &ty.kind {
        TypeKind::Primitive(p) if p == &Primitive::Bool => {
            Ok(quote! { &[if #ident { 1 } else { 0 } ] })
        }
        TypeKind::Primitive(Primitive::U8) => Ok(quote! { #ident }),
        TypeKind::Primitive(prim) => Err(ParseError::new(
            Span::call_site(),
            format!(
                "Unsupported primitive type: {}, only u8 is supported. Consider using String or str instead.",
                prim
            ),
        )),
        TypeKind::Value(Value::String)
        | TypeKind::Value(Value::CString)
        | TypeKind::Value(Value::Str) => Ok(quote! { #ident.as_bytes() }),
        TypeKind::Value(Value::Custom(x))
            if x == PUBKEY_TY
                || x == FULL_PUBKEY_TY
                || x == ACCOUNT_INFO_TY
                || x == FULL_ACCOUNT_INFO_TY =>
        {
            Ok(quote! { #ident.as_ref() })
        }
        TypeKind::Value(Value::Custom(x)) => Err(ParseError::new(
            ty.ident.span(),
            format!("Custom seed type {} not supported yet", x),
        )),
        TypeKind::Composite(k1, k2) => Err(ParseError::new(
            ty.ident.span(),
            format!(
                "Composite seed types aren't supported yet ({:?}, {:?})",
                k1, k2
            ),
        )),
        TypeKind::Unit => {
            Err(ParseError::new(ident.span(), "Seeds cannot be unit type"))
        }
        TypeKind::Unknown => Err(ParseError::new(
            ident.span(),
            "Seeds cannot be of unknown type",
        )),
    }
}

#[cfg(test)]
mod tests {
    use shank_macro_impl::parsed_struct::Seeds;

    use super::*;

    fn render_seeds_parts(seeds: &[Seed]) -> RenderedSeedsParts {
        let processed_seeds = Seeds(seeds.to_vec())
            .process()
            .expect("should process seeds without error");
        try_render_seeds_parts(&processed_seeds, "a")
            .expect("Should render seeds without error")
    }

    fn assert_tokenstream_eq(actual: &TokenStream, expected: &str) {
        let expected_ts = expected.parse::<TokenStream>().unwrap().to_string();
        assert_eq!(actual.to_string(), expected_ts);
    }

    #[test]
    fn render_seed_literal() {
        let seed = Seed::Literal("uno".to_string());
        let RenderedSeedsParts {
            seed_array_items,
            seed_fn_args,
        } = render_seeds_parts(&[seed]);

        assert_eq!(seed_array_items.len(), 1);
        assert_tokenstream_eq(&seed_array_items[0], "b\"uno\"");
        assert_eq!(seed_fn_args.len(), 0);
    }

    #[test]
    fn process_seed_program_id() {
        let seed = Seed::ProgramId;
        let RenderedSeedsParts {
            seed_array_items,
            seed_fn_args,
        } = render_seeds_parts(&[seed]);

        let expected_item = quote! { program_id.as_ref() }.to_string();

        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_fn_args.len(), 1);
        assert_eq!(seed_array_items[0].to_string(), expected_item);
        assert_tokenstream_eq(
            &seed_fn_args[0],
            "program_id : &'a ::solana_program::pubkey::Pubkey",
        );
    }

    #[test]
    fn process_seed_custom_pubkey() {
        let seed =
            Seed::Param("owner".to_string(), "The owner".to_string(), None);
        let RenderedSeedsParts {
            seed_array_items,
            seed_fn_args,
        } = render_seeds_parts(&[seed]);

        let expected_item = quote! { owner.as_ref() }.to_string();
        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_fn_args.len(), 1);
        assert_eq!(seed_array_items[0].to_string(), expected_item);
        assert_tokenstream_eq(
            &seed_fn_args[0],
            "owner : &'a ::solana_program::pubkey::Pubkey",
        );
    }

    #[test]
    fn process_seed_explicit_custom_pubkey() {
        let seed = Seed::Param(
            "owner".to_string(),
            "The owner".to_string(),
            Some("Pubkey".to_string()),
        );
        let RenderedSeedsParts {
            seed_array_items,
            seed_fn_args,
        } = render_seeds_parts(&[seed]);

        let expected_item = quote! { owner.as_ref() }.to_string();
        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_fn_args.len(), 1);
        assert_eq!(seed_array_items[0].to_string(), expected_item);
        assert_tokenstream_eq(
            &seed_fn_args[0],
            "owner : &'a ::solana_program::pubkey::Pubkey",
        );
    }
}
