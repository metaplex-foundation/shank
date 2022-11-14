use std::{collections::HashSet, convert::TryFrom, slice::Iter};

use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Error as ParseError, Lit,
    Meta, MetaList, NestedMeta, Path, Result as ParseResult,
};

use super::{ProcessedSeed, Seed};

const SUPPORTED_FORMATS: &str = r##"Examples of supported seeds:
#[seeds("literal", program_id, pubkey("description"), byte("desc", u8), other_type("desc", u32))]"##;

// -----------------
// StructAttr
// -----------------
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StructAttr {
    Seeds(Seeds),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seeds(pub Vec<Seed>);

impl Seeds {
    pub fn get_literals(&self) -> Vec<String> {
        self.0.iter().filter_map(|x| x.get_literal()).collect()
    }

    pub fn get_program_ids(&self) -> Vec<Seed> {
        self.0.iter().filter_map(|x| x.get_program_id()).collect()
    }

    pub fn get_params(&self) -> Vec<Seed> {
        self.0.iter().filter_map(|x| x.get_param()).collect()
    }

    pub fn iter(&self) -> Iter<Seed> {
        self.0.iter()
    }

    pub fn process(&self) -> ParseResult<Vec<ProcessedSeed>> {
        self.iter()
            .map(ProcessedSeed::try_from)
            .collect::<ParseResult<Vec<ProcessedSeed>>>()
    }
}

impl From<&StructAttr> for String {
    fn from(attr: &StructAttr) -> Self {
        match attr {
            StructAttr::Seeds(_seeds) => "seeds".to_string(),
        }
    }
}

impl StructAttr {
    pub fn into_seeds(self) -> Option<Vec<Seed>> {
        match self {
            StructAttr::Seeds(seeds) => Some(seeds.0),
        }
    }
}

// -----------------
// StructAttrs
// -----------------
#[derive(Debug)]
pub struct StructAttrs(pub HashSet<StructAttr>);
impl StructAttrs {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
    pub fn items_ref(&self) -> Vec<&StructAttr> {
        self.0.iter().collect::<Vec<&StructAttr>>()
    }
    pub fn items(self) -> Vec<StructAttr> {
        self.0.into_iter().collect::<Vec<StructAttr>>()
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn insert(&mut self, attr: StructAttr) -> bool {
        self.0.insert(attr)
    }
}

impl Default for StructAttrs {
    fn default() -> Self {
        Self::new()
    }
}

// TODO(thlorenz): Include the stringified representation of invalid seeds when possible in order
// to improve error messages during IDL generation via shank-cli
impl TryFrom<&[Attribute]> for StructAttrs {
    type Error = ParseError;
    fn try_from(attrs: &[Attribute]) -> ParseResult<Self> {
        let seed_attrs: Vec<&Attribute> = attrs
            .iter()
            .filter(|attr| attr.path.is_ident("seeds"))
            .collect();

        if seed_attrs.len() > 1 {
            return Err(ParseError::new(
                Span::call_site(),
                format!(
                    "Only one #[seed(..)] allowed per account\n{}",
                    SUPPORTED_FORMATS
                ),
            ));
        }

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
                        Meta::Path(path) => {
                            let Path { segments, .. } = path;
                            // Should be exactly one segment
                            if segments.len() != 1 {
                                Err(ParseError::new(
                                    path.get_ident().unwrap().span(),
                                    format!(
                                        "This seed definition is invalid.\n{}",
                                        SUPPORTED_FORMATS
                                    ),
                                ))
                            } else {
                                let ident = &segments.first().unwrap().ident;

                                match ident.to_string().as_str() {
                                    "program_id" => Ok(Seed::ProgramId),
                                    _ => Err(ParseError::new(
                                        ident.span(),
                                        format!(
                                        "This seed definition is invalid.\n{}",
                                        SUPPORTED_FORMATS
                                    ),
                                    )),
                                }
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
                        Meta::NameValue(val) => Err(ParseError::new(
                            val.path.get_ident().unwrap().span(),
                            format!(
                                "This seed definition is invalid.\n{}",
                                SUPPORTED_FORMATS
                            ),
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
    let desc_meta = iter.next().ok_or_else(|| {
        ParseError::new(
            *span,
            format!("Failed to read Param description which should be the first argument.\n{}", SUPPORTED_FORMATS)
        )
    })?;
    let ty_meta = iter.next();

    let desc = match desc_meta {
        NestedMeta::Meta(_) => Err(ParseError::new(
            *span,
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
                    format!("Second arg to Param needs to be an exactly one Rust type, tuples or collections are not supported.\n{}", SUPPORTED_FORMATS),
                )),
                NestedMeta::Meta(Meta::NameValue(val),) => Err(ParseError::new(
                    val.path.get_ident().unwrap().span(),
                    format!("Second arg to Param needs to be an exactly one Rust type, assignments are not supported.\n{}", SUPPORTED_FORMATS),
                )),
                NestedMeta::Lit(lit) => Err(ParseError::new(
                    lit.span(),
                    format!("Second arg to Param needs to be an unquoted Rust type.\n{}", SUPPORTED_FORMATS),
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
