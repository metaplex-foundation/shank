use quote::quote;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use shank_macro_impl::{
    parsed_struct::{ProcessedSeed, Seed, StructAttr, StructAttrs},
    syn::{Error as ParseError, Result as ParseResult},
    types::{Primitive, RustType, TypeKind, Value},
};

pub fn try_render_seeds(
    struct_attrs: &StructAttrs,
) -> ParseResult<TokenStream> {
    let all_seeds = struct_attrs
        .items_ref()
        .iter()
        .filter_map(|attr| match attr {
            StructAttr::Seeds(seeds) => Some(seeds),
        })
        .collect::<Vec<_>>();

    assert!(
        all_seeds.len() <= 1,
        "Should only have one seed per account"
    );

    if all_seeds.is_empty() {
        return Ok(TokenStream::new());
    }

    let seeds = all_seeds.first().unwrap();
    let processed = seeds.process()?;
    let items = processed
        .iter()
        .map(render_seed_item)
        .collect::<ParseResult<TokenStream>>()?;
    Ok(items)
}

// -----------------
// Function Args
// -----------------
fn render_function_arg(
    seed: &ProcessedSeed,
) -> ParseResult<Option<TokenStream>> {
    // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
    // that the arg is set
    match &seed.seed {
        Seed::Literal(_) => {
            // Literal items don't need to be passed to the function
            Ok(None)
        }
        Seed::ProgramId => {
            // @@@: RustType should know how to render itself and we should just invoke that here

            let item = seed_item("program_id", &seed.arg.as_ref().unwrap().ty)?;
            Ok(Some(item))
        }
        Seed::Param(name, _, _) => {
            // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
            // that the arg is set
            let item =
                seed_item(name.as_str(), &seed.arg.as_ref().unwrap().ty)?;
            Ok(Some(item))
        }
    }
}

// -----------------
// Seed Items
// -----------------
fn render_seed_item(seed: &ProcessedSeed) -> ParseResult<TokenStream> {
    match &seed.seed {
        Seed::Literal(lit) => {
            let item = TokenStream::from_str(&format!("b\"{}\"", lit))?;
            Ok(item)
        }
        Seed::ProgramId => {
            let item = seed_item("program_id", &seed.arg.as_ref().unwrap().ty)?;
            Ok(item)
        }
        Seed::Param(name, _, _) => {
            // NOTE: for a param seed shank-macro-impl:src/parsed_struct/seeds.rs always ensures
            // that the arg is set
            let item =
                seed_item(name.as_str(), &seed.arg.as_ref().unwrap().ty)?;
            Ok(item)
        }
    }
}

fn seed_item(name: &str, ty: &RustType) -> ParseResult<TokenStream> {
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

    fn render_seeds(seeds: &[Seed]) -> TokenStream {
        let attrs = struct_attrs_with_seeds(seeds);
        try_render_seeds(&attrs).expect("Should render seeds fine")
    }

    #[test]
    fn render_seed_literal() {
        let seed = Seed::Literal("uno".to_string());
        let toks = render_seeds(&[seed]);

        let expected = quote! { b"uno" }.to_string();
        assert_eq!(toks.to_string(), expected);
        // assert!(arg.is_none());
    }

    /*
    #[test]
    fn process_seed_program_id() {
        let seed = Seed::ProgramId;
        let ProcessedSeed { item, arg } = ProcessedSeed::try_from(&seed)
            .expect("should process seed without error");

        let expected_item = quote! { program_id.as_ref() }.to_string();
        let expected_arg = quote! { program_id: &Pubkey };
        assert_eq!(item.to_string(), expected_item);
        // assert_eq!(arg.unwrap().to_string(), expected_arg.to_string());
    }
    */
}
