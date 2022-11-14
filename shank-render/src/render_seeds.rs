use quote::quote;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use shank_macro_impl::{
    parsed_struct::{ProcessedSeed, Seed, StructAttr, StructAttrs},
    syn::{Error as ParseError, Result as ParseResult},
    types::{Composite, Primitive, RustType, TypeKind, Value},
};

pub fn try_render_seeds_fn(
    struct_attrs: &StructAttrs,
) -> ParseResult<Option<TokenStream>> {
    let RenderedSeedsParts {
        seed_array_items,
        seed_fn_args,
    } = try_render_seeds_parts(struct_attrs)?;
    if seed_array_items.is_empty() {
        return Ok(None);
    }

    let len = seed_array_items.len();
    // The seed function will be part of an impl block for the Account for which we're
    // deriving the seeds, thus we can re-use the same name without clashes
    Ok(Some(quote! {
        pub fn account_seeds(#(#seed_fn_args),*) -> [&'a [u8]; #len] {
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
    struct_attrs: &StructAttrs,
) -> ParseResult<RenderedSeedsParts> {
    let all_seeds = struct_attrs
        .items_ref()
        .iter()
        .filter_map(|attr| match attr {
            StructAttr::Seeds(seeds) => Some(seeds),
        })
        .collect::<Vec<_>>();

    assert!(
        all_seeds.len() <= 1,
        "Should only have one seed definition per account"
    );

    if all_seeds.is_empty() {
        return Ok(RenderedSeedsParts {
            seed_array_items: vec![],
            seed_fn_args: vec![],
        });
    }

    let seeds = all_seeds.first().unwrap();
    let processed = seeds.process()?;

    let seed_fn_args = processed
        .iter()
        .map(render_seed_function_arg)
        .collect::<ParseResult<Vec<Option<TokenStream>>>>()?
        .into_iter()
        .filter(Option::is_some)
        .flatten()
        .collect::<Vec<TokenStream>>();

    let seed_array_items = processed
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
) -> ParseResult<Option<TokenStream>> {
    // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs
    //       always ensures that the arg is set
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
                .with_lifetime("a")?
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
            let kind =
                TypeKind::Composite(Composite::Array(1), vec![ty.clone()]);
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
        TypeKind::Primitive(_p) => Ok(quote! { &[#ident] }),
        TypeKind::Value(Value::String)
        | TypeKind::Value(Value::CString)
        | TypeKind::Value(Value::Str) => Ok(quote! { #ident.as_bytes() }),
        TypeKind::Value(Value::Custom(x)) if x == "Pubkey" => {
            Ok(quote! { #ident.as_ref() })
        }
        TypeKind::Value(Value::Custom(x)) if x == "AccountInfo" => {
            Ok(quote! { #ident.key.as_ref() })
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

    fn struct_attrs_with_seeds(seeds: &[Seed]) -> StructAttrs {
        let struct_attr = StructAttr::Seeds(Seeds(seeds.to_vec()));
        let mut attrs = StructAttrs::new();
        attrs.insert(struct_attr);
        attrs
    }

    fn render_seeds_parts(seeds: &[Seed]) -> RenderedSeedsParts {
        let attrs = struct_attrs_with_seeds(seeds);
        try_render_seeds_parts(&attrs).expect("Should render seeds fine")
    }

    #[test]
    fn render_seed_literal() {
        let seed = Seed::Literal("uno".to_string());
        let RenderedSeedsParts {
            seed_array_items,
            seed_fn_args,
        } = render_seeds_parts(&[seed]);

        let expected_item = quote! { b"uno" }.to_string();
        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_array_items[0].to_string(), expected_item);
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
        let expected_arg = "program_id : &'a Pubkey".to_string();
        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_fn_args.len(), 1);
        assert_eq!(seed_array_items[0].to_string(), expected_item);
        assert_eq!(seed_fn_args[0].to_string(), expected_arg);
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
        let expected_arg = "owner : &'a Pubkey".to_string();
        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_fn_args.len(), 1);
        assert_eq!(seed_array_items[0].to_string(), expected_item);
        assert_eq!(seed_fn_args[0].to_string(), expected_arg);
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
        let expected_arg = "owner : &'a Pubkey".to_string();
        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_fn_args.len(), 1);
        assert_eq!(seed_array_items[0].to_string(), expected_item);
        assert_eq!(seed_fn_args[0].to_string(), expected_arg);
    }
}

// -----------------
// Integration Tests based on Real World Examples
// -----------------
#[cfg(test)]
mod seed_integration {
    use shank_macro_impl::{
        account::extract_account_structs,
        syn::{self, ItemStruct},
    };

    use super::*;

    fn parse_struct(code: TokenStream) -> ItemStruct {
        syn::parse2::<ItemStruct>(code).expect("Should parse successfully")
    }

    fn render_seeds(code: TokenStream) -> TokenStream {
        let account_struct = parse_struct(code);
        let all_structs = vec![&account_struct].into_iter();
        let parsed_structs = extract_account_structs(all_structs)
            .expect("Should parse struct without error");

        let struct_attrs = &parsed_structs.first().unwrap().struct_attrs;
        try_render_seeds_fn(&struct_attrs)
            .expect("Should render seeds")
            .unwrap()
    }

    #[test]
    fn literal_and_pubkeys() {
        let code = quote! {
            #[derive(ShankAccount)]
            #[seeds(
                /* literal    */ "lit:prefix",
                /* program_id */ program_id,
                /* pubkey     */ some_pubkey("description of some pubkey"),
                /* byte       */ some_byte("description of byte", u8),
            )]
            struct AccountStructWithSeed {
                count: u8,
            }
        };
        // TODO(thlorenz): need to map `u8` to `&[u8]` for the seed fn
        let expected = quote! {
            pub fn account_seeds<'a>(
                program_id: &'a Pubkey,
                some_pubkey: &'a Pubkey,
                some_byte: &'a [u8],
            ) -> [&'a [u8]; 4usize] {
                [
                    b"lit:prefix",
                    program_id.as_ref(),
                    some_pubkey.as_ref(),
                    some_byte,
                ]
            }
        };
        let rendered = render_seeds(code);
        eprintln!("{}", rendered.to_string());
    }
}
