use std::convert::{TryFrom, TryInto};

use anyhow::{Error, Result};

use serde::{Deserialize, Serialize};
use shank_macro_impl::types::{
    Composite, Primitive, RustType, TypeKind, Value,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IdlType {
    Array(Box<IdlType>, usize),
    Bool,
    Bytes,
    Defined(String),
    I128,
    I16,
    I32,
    I64,
    I8,
    Option(Box<IdlType>),
    FixedSizeOption {
        inner: Box<IdlType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sentinel: Option<Vec<u8>>,
    },
    Tuple(Vec<IdlType>),
    PublicKey,
    String,
    U128,
    U16,
    U32,
    U64,
    U8,
    Vec(Box<IdlType>),
    HashMap(Box<IdlType>, Box<IdlType>),
    BTreeMap(Box<IdlType>, Box<IdlType>),
    HashSet(Box<IdlType>),
    BTreeSet(Box<IdlType>),
}

/// Generates the sentinel value for a given IdlType.
/// Returns None if the type doesn't have a well-defined sentinel value.
/// Sentinel values are represented as little-endian byte arrays.
fn generate_sentinel_for_type(idl_type: &IdlType) -> Option<Vec<u8>> {
    match idl_type {
        // Integer types use MAX value as sentinel
        IdlType::I8 => Some(vec![0x7F]),                                          // i8::MAX
        IdlType::U8 => Some(vec![0xFF]),                                          // u8::MAX
        IdlType::I16 => Some(vec![0xFF, 0x7F]),                                   // i16::MAX
        IdlType::U16 => Some(vec![0xFF, 0xFF]),                                   // u16::MAX
        IdlType::I32 => Some(vec![0xFF, 0xFF, 0xFF, 0x7F]),                       // i32::MAX
        IdlType::U32 => Some(vec![0xFF, 0xFF, 0xFF, 0xFF]),                       // u32::MAX
        IdlType::I64 => Some(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]), // i64::MAX
        IdlType::U64 => Some(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // u64::MAX
        IdlType::I128 => Some(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]), // i128::MAX
        IdlType::U128 => Some(vec![0xFF; 16]),                                    // u128::MAX

        // Pubkey uses all zeros as sentinel
        IdlType::PublicKey => Some(vec![0x00; 32]),

        // Other types don't have well-defined sentinel values for PodOption
        _ => None,
    }
}

/// Maps podded/bytemuck types to their corresponding IDL types.
/// Returns Some(IdlType) if the type name matches a known podded type, None otherwise.
/// Sentinel values are represented as little-endian byte arrays.
fn map_podded_type(name: &str) -> Option<IdlType> {
    match name {
        // Fixed-width optional types (use sentinel values instead of tag byte)
        // Sentinel values are the maximum value for each type in little-endian format
        "OptionalI64" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::I64),
            sentinel: Some(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]), // i64::MAX
        }),
        "OptionalU64" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::U64),
            sentinel: Some(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // u64::MAX
        }),
        "OptionalI32" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::I32),
            sentinel: Some(vec![0xFF, 0xFF, 0xFF, 0x7F]), // i32::MAX
        }),
        "OptionalU32" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::U32),
            sentinel: Some(vec![0xFF, 0xFF, 0xFF, 0xFF]), // u32::MAX
        }),
        "OptionalI16" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::I16),
            sentinel: Some(vec![0xFF, 0x7F]), // i16::MAX
        }),
        "OptionalU16" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::U16),
            sentinel: Some(vec![0xFF, 0xFF]), // u16::MAX
        }),
        "OptionalI8" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::I8),
            sentinel: Some(vec![0x7F]), // i8::MAX
        }),
        "OptionalU8" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::U8),
            sentinel: Some(vec![0xFF]), // u8::MAX
        }),
        "OptionalPubkey" => Some(IdlType::FixedSizeOption {
            inner: Box::new(IdlType::PublicKey),
            sentinel: Some(vec![0x00; 32]), // All bits 0 for Pubkey (32 bytes)
        }),
        _ => None,
    }
}

impl TryFrom<RustType> for IdlType {
    type Error = Error;

    fn try_from(rust_ty: RustType) -> Result<Self> {
        let idl_ty = match rust_ty.kind {
            TypeKind::Primitive(prim) => match prim {
                Primitive::U8 => IdlType::U8,
                Primitive::I8 => IdlType::I8,
                Primitive::I16 => IdlType::I16,
                Primitive::U16 => IdlType::U16,
                Primitive::I32 => IdlType::I32,
                Primitive::U32 => IdlType::U32,
                Primitive::I64 => IdlType::I64,
                Primitive::U64 => IdlType::U64,
                Primitive::U128 => IdlType::U128,
                Primitive::I128 => IdlType::I128,
                // ebpf is 64-bit architecture
                Primitive::USize => IdlType::U64,
                Primitive::Bool => IdlType::Bool,
            },
            TypeKind::Value(val) => match val {
                Value::CString | Value::String | Value::Str => IdlType::String,
                Value::Custom(name) => {
                    // Check for podded/bytemuck types first
                    if let Some(podded_type) = map_podded_type(&name) {
                        podded_type
                    } else if name == "Pubkey" {
                        IdlType::PublicKey
                    } else {
                        IdlType::Defined(name)
                    }
                }
            },
            TypeKind::Composite(kind, inners) => match kind {
                Composite::Vec => match inners.first().cloned() {
                    Some(inner) => {
                        let inner_idl: IdlType = inner.try_into()?;
                        if inner_idl == IdlType::U8 {
                            // Vec<u8>
                            IdlType::Bytes
                        } else {
                            IdlType::Vec(Box::new(inner_idl))
                        }
                    }
                    None => {
                        anyhow::bail!("Rust Vec Composite needs inner type")
                    }
                },
                Composite::Array(size) => match inners.first().cloned() {
                    Some(inner) => {
                        let inner_idl: IdlType = inner.try_into()?;
                        IdlType::Array(Box::new(inner_idl), size)
                    }
                    None => {
                        anyhow::bail!("Rust Array Composite needs inner type")
                    }
                },

                Composite::Option => match inners.first().cloned() {
                    Some(inner) => {
                        let inner_idl: IdlType = inner.try_into()?;
                        IdlType::Option(Box::new(inner_idl))
                    }
                    None => {
                        anyhow::bail!("Rust Option Composite needs inner type")
                    }
                },
                Composite::PodOption => match inners.first().cloned() {
                    Some(inner) => {
                        let inner_idl: IdlType = inner.try_into()?;
                        // Generate sentinel for primitives and well-known types
                        // For custom types (Defined), sentinel will be None initially and
                        // will be populated during post-processing from the type's #[pod_sentinel] attribute
                        let sentinel = generate_sentinel_for_type(&inner_idl);
                        IdlType::FixedSizeOption {
                            inner: Box::new(inner_idl),
                            sentinel,
                        }
                    }
                    None => {
                        anyhow::bail!("Rust PodOption Composite needs inner type")
                    }
                },
                Composite::Tuple => {
                    if inners.len() < 2 {
                        anyhow::bail!("Rust Tuple Composite needs at least two inner types");
                    } else {
                        let idl_types: Result<Vec<IdlType>> =
                            inners.into_iter().map(IdlType::try_from).collect();
                        IdlType::Tuple(idl_types?)
                    }
                }
                Composite::HashMap => {
                    match (inners.first().cloned(), inners.get(1).cloned()) {
                        (Some(inner1), Some(inner2)) => {
                            let inner1_idl: IdlType = inner1.try_into()?;
                            let inner2_idl: IdlType = inner2.try_into()?;
                            IdlType::HashMap(
                                Box::new(inner1_idl),
                                Box::new(inner2_idl),
                            )
                        }
                        _ => {
                            anyhow::bail!(
                                "Rust HashMap Composite needs two inner types"
                            )
                        }
                    }
                }
                Composite::BTreeMap => {
                    match (inners.first().cloned(), inners.get(1).cloned()) {
                        (Some(inner1), Some(inner2)) => {
                            let inner1_idl: IdlType = inner1.try_into()?;
                            let inner2_idl: IdlType = inner2.try_into()?;
                            IdlType::BTreeMap(
                                Box::new(inner1_idl),
                                Box::new(inner2_idl),
                            )
                        }
                        _ => {
                            anyhow::bail!(
                                "Rust BTreeMap Composite needs two inner types"
                            )
                        }
                    }
                }
                Composite::HashSet => match inners.first().cloned() {
                    Some(inner) => {
                        let inner_idl: IdlType = inner.try_into()?;
                        IdlType::HashSet(Box::new(inner_idl))
                    }
                    _ => {
                        anyhow::bail!(
                            "Rust HashSet Composite needs one inner type"
                        )
                    }
                },
                Composite::BTreeSet => match inners.first().cloned() {
                    Some(inner) => {
                        let inner_idl: IdlType = inner.try_into()?;
                        IdlType::BTreeSet(Box::new(inner_idl))
                    }
                    _ => {
                        anyhow::bail!(
                            "Rust BTreeSet Composite needs one inner type"
                        )
                    }
                },
                Composite::Custom(_) => {
                    anyhow::bail!(
                        "Rust Custom Composite IDL type not yet supported"
                    )
                }
            },
            TypeKind::Unit => anyhow::bail!("IDL types cannot be Unit ()"),
            TypeKind::Unknown => {
                anyhow::bail!("Can only convert known types to IDL type")
            }
        };
        Ok(idl_ty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idl_from_rust_type_primivives() {
        for (rust_prim, idl_expected) in [
            (Primitive::U8, IdlType::U8),
            (Primitive::U16, IdlType::U16),
            (Primitive::I128, IdlType::I128),
            (Primitive::Bool, IdlType::Bool),
            (Primitive::USize, IdlType::U64),
        ] {
            let rust_ty = RustType::owned_primitive("prim", rust_prim);
            let idl_ty: IdlType =
                rust_ty.try_into().expect("Failed to convert");
            assert_eq!(idl_ty, idl_expected);
        }
    }
    #[test]
    fn idl_from_rust_type_string() {
        let rust_ty = RustType::owned_string("s");
        let idl_ty: IdlType = rust_ty.try_into().expect("Failed to convert");
        assert_eq!(idl_ty, IdlType::String);
    }

    #[test]
    fn idl_from_rust_type_publickey() {
        let rust_ty = RustType::owned_custom_value("pk", "Pubkey");
        let idl_ty: IdlType = rust_ty.try_into().expect("Failed to convert");
        assert_eq!(idl_ty, IdlType::PublicKey);
    }

    #[test]
    fn idl_from_rust_type_custom() {
        let rust_ty = RustType::owned_custom_value("custom", "SomeUserStruct");
        let idl_ty: IdlType = rust_ty.try_into().expect("Failed to convert");
        assert_eq!(idl_ty, IdlType::Defined("SomeUserStruct".to_string()));
    }

    #[test]
    fn idl_from_rust_type_vec() {
        let rust_ty = RustType::owned_vec_primitive("vec_u16", Primitive::U16);
        let idl_ty: IdlType = rust_ty.try_into().expect("Failed to convert");
        assert_eq!(idl_ty, IdlType::Vec(Box::new(IdlType::U16)));
    }

    #[test]
    fn idl_from_rust_type_vec_u8() {
        let rust_ty = RustType::owned_vec_primitive("bytes", Primitive::U8);
        let idl_ty: IdlType = rust_ty.try_into().expect("Failed to convert");
        assert_eq!(idl_ty, IdlType::Bytes);
    }

    #[test]
    fn idl_from_rust_type_array_u8() {
        let rust_ty =
            RustType::owned_array_primitive("bytes", Primitive::U8, 5);
        let idl_ty: IdlType = rust_ty.try_into().expect("Failed to convert");
        assert_eq!(idl_ty, IdlType::Array(Box::new(IdlType::U8), 5));
    }

    #[test]
    fn idl_from_rust_type_option_i64() {
        let rust_ty = RustType::owned_option_primitive("bytes", Primitive::I64);
        let idl_ty: IdlType = rust_ty.try_into().expect("Failed to convert");
        assert_eq!(idl_ty, IdlType::Option(Box::new(IdlType::I64)));
    }
}
