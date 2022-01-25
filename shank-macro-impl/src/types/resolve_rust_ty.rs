use std::{convert::TryFrom, ops::Deref};

use quote::format_ident;
use syn::{
    spanned::Spanned, AngleBracketedGenericArguments, Expr, ExprLit, GenericArgument, Ident, Lit,
    Path, PathArguments, PathSegment, Type, TypeArray, TypePath,
};

use super::{Composite, ParsedReference, Primitive, TypeKind, Value};
use syn::{Error as ParseError, Result as ParseResult};

#[derive(Debug, Clone, PartialEq)]
pub struct RustType {
    pub ident: Ident,

    pub kind: TypeKind,
    pub reference: ParsedReference,

    /// The context of the type, i.e. is it an inner type of `Vec<ty>` and thus a
    /// CollectionInnerType
    pub context: RustTypeContext,
}

impl TryFrom<&Type> for RustType {
    type Error = ParseError;

    fn try_from(ty: &Type) -> ParseResult<Self> {
        resolve_rust_ty(ty, super::RustTypeContext::Default)
    }
}

pub struct IdentWrap(Ident);
impl From<&str> for IdentWrap {
    fn from(s: &str) -> Self {
        let inner = format_ident!("{}", s);
        Self(inner)
    }
}

// -----------------
// RustType creation helper methods
// -----------------
impl RustType {
    pub fn owned<T: Into<IdentWrap>>(ident: T, kind: TypeKind) -> Self {
        let ident_wrap: IdentWrap = ident.into();
        RustType {
            ident: ident_wrap.0,
            kind,
            reference: ParsedReference::Owned,
            context: RustTypeContext::Default,
        }
    }
    pub fn owned_primitive<T: Into<IdentWrap>>(ident: T, primitive: Primitive) -> Self {
        RustType::owned(ident, TypeKind::Primitive(primitive))
    }
    pub fn owned_string<T: Into<IdentWrap>>(ident: T) -> Self {
        RustType::owned(ident, TypeKind::Value(Value::String))
    }
    pub fn owned_custom_value<T: Into<IdentWrap>>(ident: T, value: &str) -> Self {
        RustType::owned(ident, TypeKind::Value(Value::Custom(value.to_string())))
    }
    pub fn owned_vec_primitive<T: Into<IdentWrap>>(ident: T, primitive: Primitive) -> Self {
        RustType::owned(
            ident,
            TypeKind::Composite(
                Composite::Vec,
                Some(Box::new(RustType::owned_primitive("inner", primitive))),
                None,
            ),
        )
    }

    pub fn owned_array_primitive<T: Into<IdentWrap>>(
        ident: T,
        primitive: Primitive,
        size: usize,
    ) -> Self {
        RustType::owned(
            ident,
            TypeKind::Composite(
                Composite::Array(size),
                Some(Box::new(RustType::owned_primitive("inner", primitive))),
                None,
            ),
        )
    }

    pub fn owned_option_primitive<T: Into<IdentWrap>>(ident: T, primitive: Primitive) -> Self {
        RustType::owned(
            ident,
            TypeKind::Composite(
                Composite::Option,
                Some(Box::new(RustType::owned_primitive("inner", primitive))),
                None,
            ),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustTypeContext {
    Default,
    CollectionItem,
    OptionItem,
    CustomItem,
}

fn ident_and_kind_from_path(path: &Path) -> (Ident, TypeKind) {
    let PathSegment {
        ident, arguments, ..
    } = path.segments.first().unwrap();
    (ident.clone(), ident_to_kind(&ident, &arguments))
}

fn len_from_expr(expr: &Expr) -> ParseResult<usize> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(val), ..
        }) => {
            let size = match val.base10_parse::<usize>() {
                Ok(size) => size,
                Err(err) => {
                    eprintln!("'{:?}' -> {}", val, err);
                    return Err(ParseError::new(val.span(), "Failed to parse into usize"));
                }
            };
            Ok(size)
        }
        _ => {
            eprintln!("{:#?}", expr);
            Err(ParseError::new(
                expr.span(),
                "Expected a Lit(ExprLit(Int)) expression when extracting length",
            ))
        }
    }
}

pub fn resolve_rust_ty(ty: &Type, context: RustTypeContext) -> ParseResult<RustType> {
    let (ty, reference) = match ty {
        Type::Reference(r) => {
            let pr = ParsedReference::from(r);
            (r.elem.as_ref(), pr)
        }
        Type::Array(_) | Type::Path(_) => (ty, ParsedReference::Owned),
        ty => {
            eprintln!("{:#?}", ty);
            return Err(ParseError::new(
                ty.span(),
                "Only owned or reference Path/Array types supported",
            ));
        }
    };

    let (ident, kind): (Ident, TypeKind) = match ty {
        Type::Path(TypePath { path, .. }) => {
            let (ident, kind) = ident_and_kind_from_path(path);
            (ident, kind)
        }
        Type::Array(TypeArray { elem, len, .. }) => {
            let (inner_ident, inner_kind) = match elem.deref() {
                Type::Path(TypePath { path, .. }) => ident_and_kind_from_path(path),
                _ => {
                    eprintln!("{:#?}", ty);
                    return Err(ParseError::new(
                        ty.span(),
                        "Only owned or reference Path/Array types supported",
                    ));
                }
            };
            let len = len_from_expr(len)?;
            let inner_ty = RustType {
                kind: inner_kind.clone(),
                ident: inner_ident.clone(),
                reference: ParsedReference::Owned,
                context: RustTypeContext::CollectionItem,
            };
            let kind = TypeKind::Composite(Composite::Array(len), Some(Box::new(inner_ty)), None);
            (format_ident!("Array"), kind)
        }
        _ => {
            return Err(ParseError::new(
                ty.span(),
                "Only Path or Array types supported",
            ));
        }
    };

    Ok(RustType {
        ident: ident.clone(),
        kind,
        reference,
        context,
    })
}

fn ident_to_kind(ident: &Ident, arguments: &PathArguments) -> TypeKind {
    let ident_str = ident.to_string();

    match arguments {
        // Non Composite Types
        PathArguments::None => {
            // primitives
            match ident_str.as_str() {
                "u8" => return TypeKind::Primitive(Primitive::U8),
                "i8" => return TypeKind::Primitive(Primitive::I8),
                "u16" => return TypeKind::Primitive(Primitive::U16),
                "i16" => return TypeKind::Primitive(Primitive::I16),
                "u32" => return TypeKind::Primitive(Primitive::U32),
                "i32" => return TypeKind::Primitive(Primitive::I32),
                "u64" => return TypeKind::Primitive(Primitive::U64),
                "i64" => return TypeKind::Primitive(Primitive::I64),
                "u128" => return TypeKind::Primitive(Primitive::U128),
                "i128" => return TypeKind::Primitive(Primitive::I128),
                "usize" => return TypeKind::Primitive(Primitive::USize),
                "bool" => return TypeKind::Primitive(Primitive::Bool),
                _ => {}
            };

            // known value types
            match ident_str.as_str() {
                "String" => return TypeKind::Value(Value::String),
                "CString" => return TypeKind::Value(Value::CString),
                "str" => return TypeKind::Value(Value::Str),
                _ => {}
            }

            return TypeKind::Value(Value::Custom(ident_str.clone()));
        }

        // Composite Types
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            match args.len() {
                // -----------------
                // Single Type Parameter
                // -----------------
                1 => match &args[0] {
                    GenericArgument::Type(ty) => match ident_str.as_str() {
                        "Vec" => {
                            let inner = resolve_rust_ty(ty, RustTypeContext::CollectionItem)
                                .ok()
                                .map(|x| Box::new(x));
                            TypeKind::Composite(Composite::Vec, inner, None)
                        }
                        "Option" => {
                            let inner = resolve_rust_ty(ty, RustTypeContext::OptionItem)
                                .ok()
                                .map(|x| Box::new(x));
                            TypeKind::Composite(Composite::Option, inner, None)
                        }
                        _ => {
                            let inner = resolve_rust_ty(ty, RustTypeContext::CustomItem)
                                .ok()
                                .map(|x| Box::new(x));

                            TypeKind::Composite(Composite::Custom(ident_str.clone()), inner, None)
                        }
                    },
                    _ => TypeKind::Unknown,
                },
                // -----------------
                // Two Type Parameters
                // -----------------
                2 => match (&args[0], &args[1]) {
                    (GenericArgument::Type(ty1), GenericArgument::Type(ty2)) => {
                        match ident_str.as_str() {
                            "HashMap" => {
                                let inner1 = resolve_rust_ty(ty1, RustTypeContext::CollectionItem)
                                    .ok()
                                    .map(|x| Box::new(x));
                                let inner2 = resolve_rust_ty(ty2, RustTypeContext::CollectionItem)
                                    .ok()
                                    .map(|x| Box::new(x));
                                TypeKind::Composite(Composite::HashMap, inner1, inner2)
                            }
                            _ => todo!(
                            "Not yet handling custom angle bracketed types with {} type parameters",
                            args.len()
                        ),
                        }
                    }
                    _ => TypeKind::Unknown,
                },
                _ => todo!(
                    "Not yet handling angle bracketed types with more {} type parameters",
                    args.len()
                ),
            }
        }
        PathArguments::Parenthesized(args) => {
            todo!(
                "rust_type::ident_to_kind PathArguments::Parenthesized {:#?}",
                args
            )
        }
    }
}
