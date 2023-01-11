use crate::types::{Primitive, RustType, TypeKind, Value};
use std::convert::TryFrom;
use syn::{Error as ParseError, Result as ParseResult};

const PROGRAM_ID_DESC: &str = "The id of the program";
const PROGRAM_ID_NAME: &str = "program_id";
pub const PUBKEY_TY: &str = "Pubkey";
pub const FULL_PUBKEY_TY: &str = "::solana_program::pubkey::Pubkey";
pub const ACCOUNT_INFO_TY: &str = "AccountInfo";
pub const FULL_ACCOUNT_INFO_TY: &str =
    "::solana_program::account_info::AccountInfo";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Seed {
    Literal(String),
    ProgramId,
    /// Seed param with (name, desc, type)
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedArg {
    pub name: String,
    pub desc: String,
    pub ty: RustType,
}
impl SeedArg {
    fn new(name: String, desc: String, ty: RustType) -> Self {
        Self { name, desc, ty }
    }
}

#[derive(Debug)]
pub struct ProcessedSeed {
    pub seed: Seed,
    pub arg: Option<SeedArg>,
}

impl ProcessedSeed {
    fn new(seed: Seed, arg: Option<SeedArg>) -> Self {
        Self { seed, arg }
    }
}

impl TryFrom<&Seed> for ProcessedSeed {
    type Error = ParseError;
    fn try_from(seed: &Seed) -> ParseResult<Self> {
        // NOTE: We include lifetimes as part of the render step to guarantee that they match
        // NOTE: All seeds need to be passed as references since we return an array containing
        //       them and thus cannot take ownership.
        match seed {
            Seed::Literal(_) => Ok(ProcessedSeed::new(seed.clone(), None)),
            Seed::ProgramId => {
                let name = PROGRAM_ID_NAME.to_string();
                let desc = PROGRAM_ID_DESC.to_string();
                let ty = RustType::reference(
                    PUBKEY_TY,
                    TypeKind::Value(Value::Custom(FULL_PUBKEY_TY.to_string())),
                    None,
                );
                Ok(ProcessedSeed::new(
                    seed.clone(),
                    Some(SeedArg::new(name, desc, ty)),
                ))
            }
            Seed::Param(name, desc, maybe_kind) => {
                let ty = match maybe_kind.as_ref().map(String::as_str) {
                    Some(PUBKEY_TY) | None => {
                        let kind = TypeKind::Value(Value::Custom(
                            FULL_PUBKEY_TY.to_string(),
                        ));
                        RustType::reference(PUBKEY_TY, kind, None)
                    }
                    Some(ACCOUNT_INFO_TY) => {
                        let kind = TypeKind::Value(Value::Custom(
                            FULL_ACCOUNT_INFO_TY.to_string(),
                        ));
                        RustType::reference(ACCOUNT_INFO_TY, kind, None)
                    }
                    Some(ty_name) => {
                        let ty = RustType::try_from(ty_name)?;
                        match ty.get_primitive() {
                            Some(Primitive::U8) => {
                                // u8 is the only primitive we allow for seeds and it is the only
                                // type that we don't pass by ref
                                // Instead when passed to the seeds fn it is wrapped as &[u8]
                                RustType::owned("u8", ty.kind)
                            }
                            Some(_) => {
                                return Err(ParseError::new_spanned(
                                    ty.ident,
                                    "Only u8 primitive is allowed for seeds. All other primitives need to be passed as strings."));
                            }
                            None => ty.as_reference(None),
                        }
                    }
                };
                Ok(ProcessedSeed::new(
                    seed.clone(),
                    Some(SeedArg::new(name.to_owned(), desc.to_owned(), ty)),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn process_seed_literal() {
        let seed = Seed::Literal("uno".to_string());
        let ProcessedSeed { arg, .. } = ProcessedSeed::try_from(&seed)
            .expect("Should parse seed without error");

        assert!(arg.is_none());
    }

    #[test]
    fn process_seed_program_id() {
        let seed = Seed::ProgramId;
        let ProcessedSeed { arg, .. } = ProcessedSeed::try_from(&seed)
            .expect("Should parse seed without error");

        assert_matches!(arg, Some(SeedArg { name, desc, ty }) => {
            assert_eq!(name, PROGRAM_ID_NAME);
            assert_eq!(desc, PROGRAM_ID_DESC);
            assert_eq!(ty.ident.to_string().as_str(), "Pubkey");
            assert!(ty.kind.is_custom());
            assert_eq!(&format!("{:?}", ty.kind), "TypeKind::Value(Value::Custom(\"::solana_program::pubkey::Pubkey\"))")
        });
    }

    #[test]
    fn process_seed_pubkey() {
        let seed =
            Seed::Param("mypubkey".to_string(), "my desc".to_string(), None);
        let ProcessedSeed { arg, .. } = ProcessedSeed::try_from(&seed)
            .expect("Should parse seed without error");

        assert_matches!(arg, Some(SeedArg { name, desc, ty }) => {
            assert_eq!(name, "mypubkey");
            assert_eq!(desc, "my desc");
            assert_eq!(ty.ident.to_string().as_str(), "Pubkey");
            assert!(ty.kind.is_custom());
            assert_eq!(&format!("{:?}", ty.kind), "TypeKind::Value(Value::Custom(\"::solana_program::pubkey::Pubkey\"))")
        });
    }

    #[test]
    fn process_seed_u8() {
        let seed = Seed::Param(
            "myu8".to_string(),
            "u8 desc".to_string(),
            Some("u8".to_string()),
        );
        let ProcessedSeed { arg, .. } = ProcessedSeed::try_from(&seed)
            .expect("Should parse seed without error");

        assert_matches!(arg, Some(SeedArg { name, desc, ty }) => {
            assert_eq!(name, "myu8");
            assert_eq!(desc, "u8 desc");
            assert_eq!(ty.ident.to_string().as_str(), "u8");
            assert!(ty.kind.is_primitive());
            assert_eq!(&format!("{:?}", ty.kind), "TypeKind::Primitive(Primitive::U8)")
        });
    }
}
