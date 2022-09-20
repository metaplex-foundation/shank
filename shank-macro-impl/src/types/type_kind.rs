use super::RustType;
use std::fmt::Debug;

// -----------------
// TypeKind
// -----------------
#[derive(Clone)]
pub enum TypeKind {
    Primitive(Primitive),
    Value(Value),
    Composite(Composite, Vec<RustType>),
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
                TypeKind::Composite(com1, inners1),
                TypeKind::Composite(com2, inners2),
            ) => com1 == com2 && inners1 == inners2,
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
            TypeKind::Composite(com, inners) => {
                format!("TypeKind::Composite({:?}, {:?})", com, inners)
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
        if let TypeKind::Composite(_, _) = self {
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
        if let TypeKind::Composite(Composite::Vec, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_array(&self) -> bool {
        if let TypeKind::Composite(Composite::Array(_), _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_option(&self) -> bool {
        if let TypeKind::Composite(Composite::Option, _) = self {
            true
        } else {
            false
        }
    }

    pub fn inner_composite_rust_type(&self) -> Option<RustType> {
        match self {
            TypeKind::Primitive(_) => None,
            TypeKind::Value(_) => None,
            TypeKind::Composite(Composite::Vec, inners)
            | TypeKind::Composite(Composite::Array(_), inners)
            | TypeKind::Composite(Composite::HashSet, inners)
            | TypeKind::Composite(Composite::BTreeSet, inners) => {
                inners.get(0).cloned()
            }
            TypeKind::Composite(_, _) => None,
            TypeKind::Unit => None,
            TypeKind::Unknown => None,
        }
    }

    pub fn inner_composite_rust_types(
        &self,
    ) -> (Option<RustType>, Option<RustType>) {
        match self {
            TypeKind::Primitive(_) => (None, None),
            TypeKind::Value(_) => (None, None),
            TypeKind::Composite(Composite::HashMap, inners)
            | TypeKind::Composite(Composite::BTreeMap, inners) => {
                (inners.get(0).cloned(), inners.get(1).cloned())
            }
            TypeKind::Composite(_, _) => (None, None),
            TypeKind::Unit => (None, None),
            TypeKind::Unknown => (None, None),
        }
    }

    pub fn key_val_composite_rust_types(&self) -> Option<(RustType, RustType)> {
        match self {
            TypeKind::Primitive(_) => None,
            TypeKind::Value(_) => None,
            TypeKind::Composite(composite, inners)
                if composite == &Composite::HashMap
                    || composite == &Composite::BTreeMap =>
            {
                let key = inners
                    .get(0)
                    .cloned()
                    .ok_or_else(|| {
                        format!("{:?} should have key type", composite)
                    })
                    .unwrap();

                let val = inners
                    .get(1)
                    .cloned()
                    .ok_or_else(|| {
                        format!("{:?} should have val type", composite)
                    })
                    .unwrap();

                Some((key, val))
            }
            TypeKind::Composite(_, _) => None,
            TypeKind::Unit => None,
            TypeKind::Unknown => None,
        }
    }
}

// --------------
// Primitive
// --------------
#[derive(Clone, PartialEq)]
pub enum Primitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    USize,
    Bool,
}

impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            Primitive::U8 => "Primitive::U8",
            Primitive::I8 => "Primitive::I8",
            Primitive::U16 => "Primitive::U16",
            Primitive::I16 => "Primitive::I16",
            Primitive::U32 => "Primitive::U32",
            Primitive::I32 => "Primitive::I32",
            Primitive::U64 => "Primitive::U64",
            Primitive::I64 => "Primitive::I64",
            Primitive::U128 => "Primitive::U128",
            Primitive::I128 => "Primitive::I128",
            Primitive::USize => "Primitive::Usize",
            Primitive::Bool => "Primitive::Bool",
        };
        write!(f, "{}", ty)
    }
}

// --------------
// Value
// --------------
#[derive(Clone, PartialEq)]
pub enum Value {
    CString,
    String,
    Str,
    Custom(String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::CString => write!(f, "Value::CString"),
            Value::Str => write!(f, "Value::Str"),
            Value::String => write!(f, "Value::String"),
            Value::Custom(name) => {
                write!(f, "Value::Custom(\"{}\")", name)
            }
        }
    }
}

impl Value {
    fn is_string_like(&self) -> bool {
        use Value::*;
        match self {
            CString | String | Str => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        use Value::*;
        match self {
            String => true,
            _ => false,
        }
    }

    fn is_cstring(&self) -> bool {
        use Value::*;
        match self {
            CString => true,
            _ => false,
        }
    }

    fn is_str(&self) -> bool {
        use Value::*;
        match self {
            Str => true,
            _ => false,
        }
    }
}

// --------------
// Composite
// --------------
#[derive(Clone, PartialEq)]
pub enum Composite {
    Vec,
    Array(usize),
    Tuple,
    Option,
    HashMap,
    BTreeMap,
    HashSet,
    BTreeSet,
    Custom(String),
}

impl Debug for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Composite::Vec => write!(f, "Composite::Vec"),
            Composite::Array(size) => write!(f, "Composite::Array({})", size),
            Composite::Tuple => write!(f, "Composite::Tuple"),
            Composite::Option => write!(f, "Composite::Option"),
            Composite::HashMap => write!(f, "Composite::HashMap"),
            Composite::BTreeMap => write!(f, "Composite::BTreeMap"),
            Composite::HashSet => write!(f, "Composite::HashSet"),
            Composite::BTreeSet => write!(f, "Composite::BTreeSet"),
            Composite::Custom(name) => {
                write!(f, "Composite::Custom(\"{}\")", name)
            }
        }
    }
}
