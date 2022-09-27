use std::{collections::HashSet, convert::TryFrom};

use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Error as ParseError, Lit,
    Meta, MetaList, NestedMeta, Path, Result as ParseResult,
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
    ProgramId,
    Param(String, String, Option<String>),
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
        let mut seeds = vec![];
        for arg in nested_args.iter() {
            let seed = match arg {
                NestedMeta::Meta(meta) => {
                    match meta {
                        // #[seeds(program_id)]
                        Meta::Path(Path { segments, .. }) => {
                            // @@@ propagate as proper error
                            assert_eq!(
                                segments.len(),
                                1,
                                "Should be exactly one segment"
                            );
                            let ident = &segments.first().unwrap().ident;

                            match ident.to_string().as_str()
                            {
                                "program_id" => Ok(Seed::ProgramId),
                                _ => Err(ParseError::new(ident.span(), "Either put program_id here or a literal or @@@ TODO unified error message")),
                            }
                        }
                        // #[seeds(some_pubkey("description of some pubkey", type?))]
                        Meta::List(MetaList { path, nested, .. }) => {
                            let ident = path.get_ident().unwrap();
                            let (desc, ty_str) =
                                param_args(nested, &ident.span())?;
                            let seed =
                                Seed::Param(ident.to_string(), desc, ty_str);
                            Ok(seed)
                        }
                        // TODO(thlorenz): @@@ error here to warn about invalid case
                        Meta::NameValue(val) => Err(ParseError::new(
                            val.path.get_ident().unwrap().span(),
                            "Expected args of the form @@@ TODO here",
                        )),
                    }
                }
                // #[seeds("some:literal:string")]
                NestedMeta::Lit(lit) => {
                    let seed = Seed::Literal(extract_lit_str(lit)?);
                    Ok(seed)
                }
            }?;
            seeds.push(seed);
        }

        let seeds_struct_attr = StructAttr::Seeds(Seeds(seeds));
        let struct_attrs = {
            let mut set = HashSet::new();
            set.insert(seeds_struct_attr);
            StructAttrs(set)
        };

        Ok(struct_attrs)
    }
}

fn param_args(
    meta: &Punctuated<NestedMeta, Comma>,
    span: &Span,
) -> ParseResult<(String, Option<String>)> {
    let mut iter = meta.iter();
    let desc_meta = iter.next().ok_or_else(|| ParseError::new(span.clone(), "Failed to read Param description which should be the first argument"))?;
    let ty_meta = iter.next();

    let desc = match desc_meta {
        NestedMeta::Meta(_) => Err(ParseError::new(
            span.clone(),
            "Expected a literal string for the param description",
        )),
        NestedMeta::Lit(lit) => extract_lit_str(lit),
    }?;

    let ty: Option<String> = match ty_meta {
        Some(ty_meta) => {
            match ty_meta {
                NestedMeta::Meta(Meta::Path(path)) => {
                    Ok(Some(path.get_ident().unwrap().to_string()))
                }
                NestedMeta::Meta(Meta::List(list)) => Err(ParseError::new(
                    list.path.get_ident().unwrap().span(),
                    "Second arg to Param needs to be an exactly one Rust type, tuples or collections are not supported",
                )),
                NestedMeta::Meta(Meta::NameValue(val),) => Err(ParseError::new(
                    val.path.get_ident().unwrap().span(),
                    "Second arg to Param needs to be an exactly one Rust type, assignments are not supported",
                )),
                NestedMeta::Lit(lit) => Err(ParseError::new(
                    lit.span(),
                    "Second arg to Param needs to be an unquoted Rust type",
                )),
            }?
        }
        None => None,
    };
    Ok((desc, ty))
}

fn extract_lit_str(lit: &Lit) -> ParseResult<String> {
    match lit {
        Lit::Str(str) => Ok(str.value()),
        Lit::ByteStr(_)
        | Lit::Byte(_)
        | Lit::Char(_)
        | Lit::Int(_)
        | Lit::Float(_)
        | Lit::Bool(_)
        | Lit::Verbatim(_) => {
            Err(ParseError::new(lit.span(), "Expected a literal string"))
        }
    }
}
