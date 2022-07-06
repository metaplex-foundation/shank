use std::fmt::Debug;

use crate::types::traits::{MaybeByteSize, OptionByteSize};

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
    pub(crate) fn is_string_like(&self) -> bool {
        use Value::*;
        match self {
            CString | String | Str => true,
            _ => false,
        }
    }

    pub(crate) fn is_string(&self) -> bool {
        use Value::*;
        match self {
            String => true,
            _ => false,
        }
    }

    pub(crate) fn is_cstring(&self) -> bool {
        use Value::*;
        match self {
            CString => true,
            _ => false,
        }
    }

    pub(crate) fn is_str(&self) -> bool {
        use Value::*;
        match self {
            Str => true,
            _ => false,
        }
    }
}

impl MaybeByteSize for Value {
    fn maybe_byte_size(&self) -> OptionByteSize {
        todo!()
    }
}
