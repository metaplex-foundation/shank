use quote::quote;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use shank_macro_impl::{
    parsed_struct::{
        ProcessedSeed, Seed, ACCOUNT_INFO_TY, FULL_ACCOUNT_INFO_TY,
        FULL_PUBKEY_TY, PUBKEY_TY,
    },
    syn::{Error as ParseError, Result as ParseResult},
    types::{Composite, Primitive, RustType, TypeKind, Value},
};

pub fn try_render_seeds_fn(
    processed_seeds: &[ProcessedSeed],
    pub_modifier: Option<TokenStream>,
) -> ParseResult<Option<TokenStream>> {
    let lifetime = "a";
    let RenderedSeedsParts {
        seed_array_items,
        seed_fn_args,
    } = try_render_seeds_parts(processed_seeds, lifetime)?;
    if seed_array_items.is_empty() {
        return Ok(None);
    }

    let pub_modifier = pub_modifier.unwrap_or_else(|| quote! { pub });
    let len = seed_array_items.len();
    let lifetime_toks = format!("<'{}>", lifetime).parse::<TokenStream>()?;
    // The seed function will be part of an impl block for the Account for which we're
    // deriving the seeds, thus we can re-use the same name without clashes
    Ok(Some(quote! {
        #pub_modifier fn account_seeds#lifetime_toks(#(#seed_fn_args),*) -> [&'a [u8]; #len] {
            [#(#seed_array_items),*]
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
                .unwrap()
                .ty
                .with_lifetime(lifetime)?
                .render_param("program_id");
            Ok(Some(arg))
        }
        Seed::Param(name, _, _) => {
            // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
            // that the arg is set
            let ty = seed.arg.as_ref().unwrap().ty.clone();
            let arg = adapt_seed_function_arg_type_kind(ty)
                .with_lifetime("a")?
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
            RustType { kind, ..ty }
        }
        // TODO(thlorenz): technically most of the below are not supported so we should ideally add
        // some check here to detect invalid inputs
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
