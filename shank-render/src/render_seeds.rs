use quote::quote;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use shank_macro_impl::{
    parsed_struct::{ProcessedSeed, Seed, StructAttr, StructAttrs},
    syn::{Error as ParseError, Result as ParseResult},
    types::{Primitive, RustType, TypeKind, Value},
};

#[derive(Debug)]
pub struct RenderedSeeds {
    seed_array_items: Vec<TokenStream>,
    seed_fn_args: Vec<TokenStream>,
}

pub fn try_render_seeds(
    struct_attrs: &StructAttrs,
) -> ParseResult<RenderedSeeds> {
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
        return Ok(RenderedSeeds {
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

    Ok(RenderedSeeds {
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
            let arg = seed.arg.as_ref().unwrap().ty.render_param();
            Ok(Some(arg))
        }
        Seed::Param(name, _, _) => {
            // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
            // that the arg is set
            let item =
                seed_array_item(name.as_str(), &seed.arg.as_ref().unwrap().ty)?;
            Ok(Some(item))
        }
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

    fn render_seeds(seeds: &[Seed]) -> RenderedSeeds {
        let attrs = struct_attrs_with_seeds(seeds);
        try_render_seeds(&attrs).expect("Should render seeds fine")
    }

    #[test]
    fn render_seed_literal() {
        let seed = Seed::Literal("uno".to_string());
        let RenderedSeeds {
            seed_array_items,
            seed_fn_args,
        } = render_seeds(&[seed]);

        let expected_item = quote! { b"uno" }.to_string();
        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_array_items[0].to_string(), expected_item);
        assert_eq!(seed_fn_args.len(), 0);
    }

    #[test]
    fn process_seed_program_id() {
        let seed = Seed::ProgramId;
        let RenderedSeeds {
            seed_array_items,
            seed_fn_args,
        } = render_seeds(&[seed]);

        let expected_item = quote! { program_id.as_ref() }.to_string();
        let expected_arg = quote! { program_id: &Pubkey };
        assert_eq!(seed_array_items.len(), 1);
        assert_eq!(seed_fn_args.len(), 1);
        eprintln!("{}", seed_array_items[0]);
        eprintln!("{}", seed_fn_args[0]);
        // assert_eq!(item.to_string(), expected_item);
        // assert_eq!(arg.unwrap().to_string(), expected_arg.to_string());
    }
}
