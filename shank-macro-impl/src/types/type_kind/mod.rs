mod composite;
mod primitive;
mod value;

pub use composite::*;
pub use primitive::*;
pub use value::*;

use std::fmt::Debug;

use super::RustType;

// -----------------
// TypeKind
// -----------------
#[derive(Clone)]
pub enum TypeKind {
    Primitive(Primitive),
    Value(Value),
    Composite(Composite, Option<Box<RustType>>, Option<Box<RustType>>),
    Unit,
    Unknown,
}

impl PartialEq for TypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeKind::Primitive(prim1), TypeKind::Primitive(prim2)) => {
                prim1 == prim2
            }
            (TypeKind::Value(val1), TypeKind::Value(val2)) => val1 == val2,
            (
                TypeKind::Composite(com1, first_ty1, second_ty1),
                TypeKind::Composite(com2, first_ty2, second_ty2),
            ) => {
                com1 == com2
                    && first_ty1 == first_ty2
                    && second_ty1 == second_ty2
            }
            (TypeKind::Unit, TypeKind::Unit) => true,
            (TypeKind::Unknown, TypeKind::Unknown) => true,
            _ => false,
        }
    }
}

impl Debug for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            TypeKind::Primitive(p) => format!("TypeKind::Primitive({:?})", p),
            TypeKind::Value(val) => format!("TypeKind::Value({:?})", val),
            TypeKind::Composite(com, fst_inner, snd_inner) => {
                format!(
                    "TypeKind::Composite({:?}, {:?}, {:?})",
                    com, fst_inner, snd_inner
                )
            }
            TypeKind::Unit => "TypeKind::Unit".to_string(),
            TypeKind::Unknown => "TypeKind::Unknown".to_string(),
        };
        write!(f, "{}", kind)
    }
}

impl TypeKind {
    pub fn is_primitive(&self) -> bool {
        if let TypeKind::Primitive(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_string()
        } else {
            false
        }
    }

    pub fn is_cstring(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_cstring()
        } else {
            false
        }
    }

    pub fn is_str(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_str()
        } else {
            false
        }
    }

    pub fn is_string_like(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_string_like()
        } else {
            false
        }
    }

    pub fn is_composite(&self) -> bool {
        if let TypeKind::Composite(_, _, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_custom(&self) -> bool {
        if let TypeKind::Value(Value::Custom(_)) = self {
            true
        } else {
            false
        }
    }

    pub fn is_vec(&self) -> bool {
        if let TypeKind::Composite(Composite::Vec, _, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_array(&self) -> bool {
        if let TypeKind::Composite(Composite::Array(_), _, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_option(&self) -> bool {
        if let TypeKind::Composite(Composite::Option, _, _) = self {
            true
        } else {
            false
        }
    }

    pub fn inner_composite_rust_type(&self) -> Option<RustType> {
        match self {
            TypeKind::Primitive(_) => None,
            TypeKind::Value(_) => None,
            TypeKind::Composite(Composite::Vec, inner, _)
            | TypeKind::Composite(Composite::Array(_), inner, _) => {
                inner.as_ref().map(|x| (*x.clone()))
            }
            TypeKind::Composite(_, _, _) => None,
            TypeKind::Unit => None,
            TypeKind::Unknown => None,
        }
    }

    pub fn key_val_composite_rust_types(&self) -> Option<(RustType, RustType)> {
        match self {
            TypeKind::Primitive(_) => None,
            TypeKind::Value(_) => None,
            TypeKind::Composite(Composite::HashMap, key_ty, val_ty) => {
                let key = key_ty
                    .as_ref()
                    .map(|x| *x.clone())
                    .expect("hashmap should have key type");
                let val = val_ty
                    .as_ref()
                    .map(|x| *x.clone())
                    .expect("hashmap should have val type");

                Some((key, val))
            }
            TypeKind::Composite(_, _, _) => None,
            TypeKind::Unit => None,
            TypeKind::Unknown => None,
        }
    }
}
