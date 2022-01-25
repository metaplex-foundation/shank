use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IdlRustType {
    Array(Box<IdlRustType>, usize),
    Bool,
    Bytes,
    Defined(String),
    I128,
    I16,
    I32,
    I64,
    I8,
    Option(Box<IdlRustType>),
    PublicKey,
    String,
    U128,
    U16,
    U32,
    U64,
    U8,
    Vec(Box<IdlRustType>),
}

impl std::str::FromStr for IdlRustType {
    type Err = anyhow::Error;

    // TODO(thlorenz): actually processing the `syn::Type` is more verbose but also cleaner and
    // definitely more efficient than this approach which came directly from anchor
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_string();
        fn array_from_str(inner: &str) -> IdlRustType {
            match inner.strip_suffix(']') {
                None => {
                    let (raw_type, raw_length) = inner.rsplit_once(';').unwrap();
                    let ty = IdlRustType::from_str(raw_type).unwrap();
                    let len = raw_length.replace('_', "").parse::<usize>().unwrap();
                    IdlRustType::Array(Box::new(ty), len)
                }
                Some(nested_inner) => array_from_str(&nested_inner[1..]),
            }
        }
        s.retain(|c| !c.is_whitespace());
        let r = match s.as_str() {
            "bool" => IdlRustType::Bool,
            "u8" => IdlRustType::U8,
            "i8" => IdlRustType::I8,
            "u16" => IdlRustType::U16,
            "i16" => IdlRustType::I16,
            "u32" => IdlRustType::U32,
            "i32" => IdlRustType::I32,
            "u64" => IdlRustType::U64,
            "i64" => IdlRustType::I64,
            "u128" => IdlRustType::U128,
            "i128" => IdlRustType::I128,
            "Vec<u8>" => IdlRustType::Bytes,
            "String" => IdlRustType::String,
            "Pubkey" => IdlRustType::PublicKey,
            _ => match s.to_string().strip_prefix("Option<") {
                None => match s.to_string().strip_prefix("Vec<") {
                    None => {
                        if s.to_string().starts_with('[') {
                            array_from_str(&s)
                        } else {
                            IdlRustType::Defined(s.to_string())
                        }
                    }
                    Some(inner) => {
                        let inner_ty = Self::from_str(
                            inner
                                .strip_suffix('>')
                                .ok_or_else(|| anyhow::anyhow!("Invalid option"))?,
                        )?;
                        IdlRustType::Vec(Box::new(inner_ty))
                    }
                },
                Some(inner) => {
                    let inner_ty = Self::from_str(
                        inner
                            .strip_suffix('>')
                            .ok_or_else(|| anyhow::anyhow!("Invalid option"))?,
                    )?;
                    IdlRustType::Option(Box::new(inner_ty))
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
    fn idl_type_from_str_multidimensional_array() {
        assert_eq!(
            IdlRustType::from_str("[[u8;16];32]").unwrap(),
            IdlRustType::Array(
                Box::new(IdlRustType::Array(Box::new(IdlRustType::U8), 16)),
                32
            )
        );
    }

    #[test]
    fn idl_type_from_str_array() {
        assert_eq!(
            IdlRustType::from_str("[Pubkey;16]").unwrap(),
            IdlRustType::Array(Box::new(IdlRustType::PublicKey), 16)
        );
    }

    #[test]
    fn idl_type_from_str_array_with_underscored_length() {
        assert_eq!(
            IdlRustType::from_str("[u8;50_000]").unwrap(),
            IdlRustType::Array(Box::new(IdlRustType::U8), 50_000)
        );
    }

    #[test]
    fn idl_type_from_str_option() {
        assert_eq!(
            IdlRustType::from_str("Option<bool>").unwrap(),
            IdlRustType::Option(Box::new(IdlRustType::Bool))
        )
    }

    #[test]
    fn idl_type_from_str_vector() {
        assert_eq!(
            IdlRustType::from_str("Vec<bool>").unwrap(),
            IdlRustType::Vec(Box::new(IdlRustType::Bool))
        )
    }
}
