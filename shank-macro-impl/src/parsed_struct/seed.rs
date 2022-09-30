use std::convert::TryFrom;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error as ParseError, Ident, Result as ParseResult};

use crate::types::{Primitive, RustType, TypeKind, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Seed {
    Literal(String),
    ProgramId,
    Param(String, String, Option<String>),
}

impl Seed {
    pub fn get_literal(&self) -> Option<String> {
        match self {
            Seed::Literal(lit) => Some(lit.to_string()),
            _ => None,
        }
    }

    pub fn get_program_id(&self) -> Option<Seed> {
        match self {
            Seed::ProgramId => Some(Seed::ProgramId),
            _ => None,
        }
    }

    pub fn get_param(&self) -> Option<Seed> {
        match self {
            Seed::Param(name, desc, ty) => {
                Some(Seed::Param(name.to_owned(), desc.to_owned(), ty.clone()))
            }
            _ => None,
        }
    }
}

pub struct SeedArg {
    name: String,
    desc: String,
    ty: RustType,
}
impl SeedArg {
    fn new(name: String, desc: String, ty: RustType) -> Self {
        Self { name, desc, ty }
    }
}

pub struct ProcessedSeed {
    item: TokenStream,
    arg: Option<SeedArg>,
}

impl ProcessedSeed {
    fn new(item: TokenStream, arg: Option<SeedArg>) -> Self {
        Self { item, arg }
    }
}

impl TryFrom<&Seed> for ProcessedSeed {
    type Error = ParseError;
    fn try_from(seed: &Seed) -> ParseResult<Self> {
        match seed {
            Seed::Literal(lit) => {
                let ident = Ident::new(lit, Span::call_site());
                let item = quote! { b"#ident"  };
                Ok(ProcessedSeed::new(item, None))
            }
            Seed::ProgramId => {
                let name = "program_id".to_string();
                let desc = "The id of the program".to_string();
                let ty = RustType::reference(
                    "program_id",
                    TypeKind::Value(Value::Custom("Pubkey".to_string())),
                );
                let item = seed_item("program_id", &ty)?;
                Ok(ProcessedSeed::new(item, Some(SeedArg::new(name, desc, ty))))
            }
            Seed::Param(name, desc, maybe_kind) => {
                let kind = maybe_kind
                    .as_ref()
                    .map(|s| TypeKind::Value(Value::Custom(s.to_string())))
                    .unwrap_or_else(|| {
                        TypeKind::Value(Value::Custom("Pubkey".to_string()))
                    });
                let ty = RustType::reference(name.as_str(), kind.clone());
                let item = seed_item(name.as_str(), &ty)?;
                Ok(ProcessedSeed::new(
                    item,
                    Some(SeedArg::new(name.to_owned(), desc.to_owned(), ty)),
                ))
            }
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
