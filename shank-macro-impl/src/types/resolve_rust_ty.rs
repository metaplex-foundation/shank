use std::{convert::TryFrom, ops::Deref};

use quote::format_ident;
use syn::{
    spanned::Spanned, AngleBracketedGenericArguments, Expr, ExprLit,
    GenericArgument, Ident, Lit, Path, PathArguments, PathSegment, Type,
    TypeArray, TypePath, TypeTuple,
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
    pub fn owned_primitive<T: Into<IdentWrap>>(
        ident: T,
        primitive: Primitive,
    ) -> Self {
        RustType::owned(ident, TypeKind::Primitive(primitive))
    }
    pub fn owned_string<T: Into<IdentWrap>>(ident: T) -> Self {
        RustType::owned(ident, TypeKind::Value(Value::String))
    }
    pub fn owned_custom_value<T: Into<IdentWrap>>(
        ident: T,
        value: &str,
    ) -> Self {
        RustType::owned(
            ident,
            TypeKind::Value(Value::Custom(value.to_string())),
        )
    }
    pub fn owned_vec_primitive<T: Into<IdentWrap>>(
        ident: T,
        primitive: Primitive,
    ) -> Self {
        RustType::owned(
            ident,
            TypeKind::Composite(
                Composite::Vec,
                vec![RustType::owned_primitive("inner", primitive)],
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
                vec![RustType::owned_primitive("inner", primitive)],
            ),
        )
    }

    pub fn owned_option_primitive<T: Into<IdentWrap>>(
        ident: T,
        primitive: Primitive,
    ) -> Self {
        RustType::owned(
            ident,
            TypeKind::Composite(
                Composite::Option,
                vec![RustType::owned_primitive("inner", primitive)],
            ),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustTypeContext {
    Default,
    CollectionItem,
    TupleItem,
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
                    return Err(ParseError::new(
                        val.span(),
                        format!("Failed to parse into usize: {}", err),
                    ));
                }
            };
            Ok(size)
        }
        _ => Err(ParseError::new(
            expr.span(),
            "Expected a Lit(ExprLit(Int)) expression when extracting length",
        )),
    }
}

pub fn resolve_rust_ty(
    ty: &Type,
    context: RustTypeContext,
) -> ParseResult<RustType> {
    let (ty, reference) = match ty {
        Type::Reference(r) => {
            let pr = ParsedReference::from(r);
            (r.elem.as_ref(), pr)
        }
        Type::Array(_) | Type::Path(_) | Type::Tuple(_) => {
            (ty, ParsedReference::Owned)
        }
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
                Type::Path(TypePath { path, .. }) => {
                    ident_and_kind_from_path(path)
                }
                _ => {
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
            let kind =
                TypeKind::Composite(Composite::Array(len), vec![inner_ty]);
            (format_ident!("Array"), kind)
        }
        Type::Tuple(TypeTuple { elems, .. }) => {
            if elems.len() < 2 {
                return Err(ParseError::new(
                    ty.span(),
                    "A Tuple should have at least 2 type parameters",
                ));
            }

            let mut types: Vec<RustType> = vec![];
            for elem in elems {
                match elem {
                    Type::Path(TypePath { path, .. }) => {
                        let (ident, kind) = ident_and_kind_from_path(path);
                        let ty = RustType {
                            kind: kind.clone(),
                            ident: ident.clone(),
                            reference: ParsedReference::Owned,
                            context: RustTypeContext::TupleItem,
                        };
                        types.push(ty);
                    }
                    _ => {
                        return Err(ParseError::new(
                            ty.span(),
                            "Only owned or reference Path/Array types supported",
                        ));
                    }
                }
            }
            let kind = TypeKind::Composite(Composite::Tuple, types);
            (format_ident!("Tuple"), kind)
        }
        _ => {
            return Err(ParseError::new(
                ty.span(),
                "Only Path, Tuple or Array types supported",
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
        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            args,
            ..
        }) => {
            let pat = ident_str.as_str();
            match args.len() {
                // -----------------
                // Single Type Parameter
                // -----------------
                1 => match &args[0] {
                    GenericArgument::Type(ty) => match pat {
                        "Vec" | "Option" | "HashSet" | "BTreeSet" => {
                            let composite = match pat {
                                "Vec" => Composite::Vec,
                                "Option" => Composite::Option,
                                "HashSet" => Composite::HashSet,
                                "BTreeSet" => Composite::BTreeSet,
                                _ => {
                                    panic!("Rust you are drunk (pat only matches on those specific strings)")
                                }
                            };
                            match resolve_rust_ty(
                                ty,
                                RustTypeContext::CollectionItem,
                            ) {
                                Ok(inner) => {
                                    TypeKind::Composite(composite, vec![inner])
                                }
                                Err(_) => {
                                    TypeKind::Composite(composite, vec![])
                                }
                            }
                        }
                        _ => match resolve_rust_ty(
                            ty,
                            RustTypeContext::CustomItem,
                        ) {
                            Ok(inner) => TypeKind::Composite(
                                Composite::Custom(ident_str.clone()),
                                vec![inner],
                            ),
                            Err(_) => TypeKind::Composite(
                                Composite::Custom(ident_str.clone()),
                                vec![],
                            ),
                        },
                    },
                    _ => TypeKind::Unknown,
                },
                // -----------------
                // Two Type Parameters
                // -----------------
                2 => match (&args[0], &args[1]) {
                    (
                        GenericArgument::Type(ty1),
                        GenericArgument::Type(ty2),
                    ) => match ident_str.as_str() {
                        ident if ident == "HashMap" || ident == "BTreeMap" => {
                            let inners = match (
                                resolve_rust_ty(
                                    ty1,
                                    RustTypeContext::CollectionItem,
                                ),
                                resolve_rust_ty(
                                    ty2,
                                    RustTypeContext::CollectionItem,
                                ),
                            ) {
                                (Ok(inner1), Ok(inner2)) => {
                                    vec![inner1, inner2]
                                }
                                (Ok(inner1), Err(_)) => vec![inner1],
                                (Err(_), Ok(inner2)) => vec![inner2],
                                (Err(_), Err(_)) => vec![],
                            };

                            let composite = if ident == "HashMap" {
                                Composite::HashMap
                            } else {
                                Composite::BTreeMap
                            };
                            TypeKind::Composite(composite, inners)
                        }
                        _ => {
                            eprintln!("ident: {:#?}, args: {:#?}", ident, args);
                            todo!(
                                "Not yet handling custom angle bracketed types with {} type parameters",
                                args.len()
                            )
                        }
                    },
                    _ => TypeKind::Unknown,
                },
                _ => {
                    eprintln!("ident: {:#?}, args: {:#?}", ident, args);
                    todo!(
                        "Not yet handling angle bracketed types with more {} type parameters",
                        args.len()
                    )
                }
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
