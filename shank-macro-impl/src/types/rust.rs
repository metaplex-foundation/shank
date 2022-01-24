use std::{convert::TryFrom, str::FromStr};

use serde::{Deserialize, Serialize};
use syn::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RustType {
    Array(Box<RustType>, usize),
    Bool,
    Bytes,
    Defined(String),
    I128,
    I16,
    I32,
    I64,
    I8,
    Option(Box<RustType>),
    PublicKey,
    String,
    U128,
    U16,
    U32,
    U64,
    U8,
    Vec(Box<RustType>),
}

impl TryFrom<&Type> for RustType {
    type Error = anyhow::Error;

    fn try_from(ty: &Type) -> Result<Self, Self::Error> {
        let s = format!("{:?}", ty);
        RustType::from_str(dbg!(&s))
    }
}

impl std::str::FromStr for RustType {
    type Err = anyhow::Error;

    // TODO(thlorenz): actually processing the `syn::Type` is more verbose but also cleaner and
    // definitely more efficient than this approach which came directly from anchor
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_string();
        fn array_from_str(inner: &str) -> RustType {
            match inner.strip_suffix(']') {
                None => {
                    let (raw_type, raw_length) = inner.rsplit_once(';').unwrap();
                    let ty = RustType::from_str(raw_type).unwrap();
                    let len = raw_length.replace('_', "").parse::<usize>().unwrap();
                    RustType::Array(Box::new(ty), len)
                }
                Some(nested_inner) => array_from_str(&nested_inner[1..]),
            }
        }
        s.retain(|c| !c.is_whitespace());
        let r = match s.as_str() {
            "bool" => RustType::Bool,
            "u8" => RustType::U8,
            "i8" => RustType::I8,
            "u16" => RustType::U16,
            "i16" => RustType::I16,
            "u32" => RustType::U32,
            "i32" => RustType::I32,
            "u64" => RustType::U64,
            "i64" => RustType::I64,
            "u128" => RustType::U128,
            "i128" => RustType::I128,
            "Vec<u8>" => RustType::Bytes,
            "String" => RustType::String,
            "Pubkey" => RustType::PublicKey,
            _ => match s.to_string().strip_prefix("Option<") {
                None => match s.to_string().strip_prefix("Vec<") {
                    None => {
                        if s.to_string().starts_with('[') {
                            array_from_str(&s)
                        } else {
                            RustType::Defined(s.to_string())
                        }
                    }
                    Some(inner) => {
                        let inner_ty = Self::from_str(
                            inner
                                .strip_suffix('>')
                                .ok_or_else(|| anyhow::anyhow!("Invalid option"))?,
                        )?;
                        RustType::Vec(Box::new(inner_ty))
                    }
                },
                Some(inner) => {
                    let inner_ty = Self::from_str(
                        inner
                            .strip_suffix('>')
                            .ok_or_else(|| anyhow::anyhow!("Invalid option"))?,
                    )?;
                    RustType::Option(Box::new(inner_ty))
                }
            },
        };
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn multidimensional_array() {
        assert_eq!(
            RustType::from_str("[[u8;16];32]").unwrap(),
            RustType::Array(Box::new(RustType::Array(Box::new(RustType::U8), 16)), 32)
        );
    }

    #[test]
    fn array() {
        assert_eq!(
            RustType::from_str("[Pubkey;16]").unwrap(),
            RustType::Array(Box::new(RustType::PublicKey), 16)
        );
    }

    #[test]
    fn array_with_underscored_length() {
        assert_eq!(
            RustType::from_str("[u8;50_000]").unwrap(),
            RustType::Array(Box::new(RustType::U8), 50_000)
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            RustType::from_str("Option<bool>").unwrap(),
            RustType::Option(Box::new(RustType::Bool))
        )
    }

    #[test]
    fn vector() {
        assert_eq!(
            RustType::from_str("Vec<bool>").unwrap(),
            RustType::Vec(Box::new(RustType::Bool))
        )
    }
}
