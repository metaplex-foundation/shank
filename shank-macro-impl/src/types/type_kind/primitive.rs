use std::{fmt::Debug, mem::size_of};

use crate::types::traits::ByteSize;

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

impl ByteSize for Primitive {
    fn byte_size(&self) -> usize {
        match self {
            Primitive::U8 => size_of::<u8>(),
            Primitive::I8 => size_of::<i8>(),
            Primitive::U16 => size_of::<u16>(),
            Primitive::I16 => size_of::<i16>(),
            Primitive::U32 => size_of::<u32>(),
            Primitive::I32 => size_of::<i32>(),
            Primitive::U64 => size_of::<u64>(),
            Primitive::I64 => size_of::<i64>(),
            Primitive::U128 => size_of::<u128>(),
            Primitive::I128 => size_of::<i128>(),
            Primitive::USize => size_of::<usize>(),
            Primitive::Bool => size_of::<bool>(),
        }
    }
}
