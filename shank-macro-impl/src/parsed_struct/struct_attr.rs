use std::{collections::HashSet, convert::TryFrom};

use syn::{
    parse::Parse, punctuated::Punctuated, token::Comma, Attribute,
    Error as ParseError, Lit, Meta, MetaList, NestedMeta,
    Result as ParseResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seeds(Vec<Seed>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StructAttr {
    Seeds(Seeds),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Seed {
    Literal(String),
}

impl From<&StructAttr> for String {
    fn from(attr: &StructAttr) -> Self {
        match attr {
            // TODO(thlorenz): log seeds
            StructAttr::Seeds(_seeds) => "seeds".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct StructAttrs(pub HashSet<StructAttr>);
impl TryFrom<&[Attribute]> for StructAttrs {
    type Error = ParseError;
    fn try_from(attrs: &[Attribute]) -> ParseResult<Self> {
        let seed_attrs = attrs
            .iter()
            .filter_map(|attr| {
                if attr.path.is_ident("seeds") {
                    Some(attr)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert!(
            seed_attrs.len() <= 1,
            "only one #[seed(..)] allowed per account"
        );

        // For now we only handle seeds as attributes on the `struct` itself
        if seed_attrs.first().is_none() {
            return Ok(StructAttrs(HashSet::new()));
        }

        let seed_attrs_meta = seed_attrs.first().unwrap().parse_meta()?;
        let nested_args: Punctuated<NestedMeta, Comma> = {
            use syn::Meta::*;
            match seed_attrs_meta {
                List(MetaList { nested, .. }) => nested,
                Path(_) | NameValue(_) => {
                    return Ok(StructAttrs(HashSet::new()))
                }
            }
        };
        let set = HashSet::new();
        for arg in nested_args.iter() {
            let seed = match arg {
                // @@@ implement this once done with lit
                NestedMeta::Meta(_) => None::<Seed>,
                NestedMeta::Lit(lit) => match lit {
                    Lit::Str(str) => { 
                        let seed = Seed::Literal(str.value());
                        Some(seed)
                    },
                    Lit::ByteStr(_) | 
                    Lit::Byte(_)    | 
                    Lit::Char(_)    | 
                    Lit::Int(_)     | 
                    Lit::Float(_)   | 
                    Lit::Bool(_)    | 
                    // TODO(thlorenz): ideally we'd error here to warn about an unhandled case
                    Lit::Verbatim(_) => None,
                },
            };
            eprintln!("{:#?}", seed);
        }

        Ok(StructAttrs(set))
    }
}

impl Parse for Seeds {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let seeds = Vec::<Seed>::new();

        Ok(Seeds(seeds))
    }
}

impl Parse for Seed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // if input.peek(syn::Path::parse_mod_style) {}

        Ok(Seed::Literal("".to_string()))
    }
}
