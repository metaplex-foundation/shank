use std::convert::TryFrom;
use std::fmt;

use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, Attribute, Error as ParseError, Ident, Meta,
    MetaList, NestedMeta, Result as ParseResult, Token,
};

use crate::types::{Composite, TypeKind, Value};
use crate::{
    instruction::account_attrs::identifier_from_nested_meta,
    types::{Primitive, RustType, RustTypeContext},
};

use super::{
    InstructionAccount, InstructionAccounts, InstructionVariantFields,
};

const IX_IDL: &str = "idl_instruction";

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IdlInstruction {
    Create,
    CreateBuffer,
    Write,
    SetAuthority,
    SetBuffer,
}

impl IdlInstruction {
    fn is_idl_instruction_attr(attr: &Attribute) -> Option<&Attribute> {
        match attr
            .path
            .get_ident()
            .map(|x| x.to_string().as_str() == IX_IDL)
        {
            Some(true) => Some(attr),
            _ => None,
        }
    }

    fn from_idl_instruction_attr(
        attr: &Attribute,
    ) -> ParseResult<IdlInstruction> {
        let meta = &attr.parse_meta()?;
        match meta {
            Meta::List(MetaList { nested, .. }) => {
                let ident = attr.path.get_ident().map_or_else(
                    || Ident::new("attr_ident", Span::call_site()),
                    |x| x.clone(),
                );
                Self::parse_idl_instruction_attr_args(ident, nested)
            }
            Meta::Path(_) | Meta::NameValue(_) => Err(ParseError::new_spanned(
                attr,
                "#[idl_instruction] attr requires list of arguments",
            )),
        }
    }

    fn parse_idl_instruction_attr_args(
        _ident: Ident,
        nested: &Punctuated<NestedMeta, Token![,]>,
    ) -> ParseResult<IdlInstruction> {
        if nested.is_empty() {
            return Err(ParseError::new_spanned(
                nested,
                "#[idl_instruction] attr requires at least the idl instruction name",
            ));
        }
        if nested.len() > 1 {
            return Err(ParseError::new_spanned(
                nested,
                "#[idl_instruction] attr can only have one idl instruction name",
            ));
        }
        let nested_meta = nested.first().unwrap();
        if let Some((ident, name)) = identifier_from_nested_meta(nested_meta) {
            match name.as_str() {
                "Create" => Ok(IdlInstruction::Create),
                "CreateBuffer" => Ok(IdlInstruction::CreateBuffer),
                "SetAuthority" => Ok(IdlInstruction::SetAuthority),
                "SetBuffer" => Ok(IdlInstruction::SetBuffer),
                "Write" => Ok(IdlInstruction::Write),
                _ => Err(ParseError::new_spanned(
                    ident,
                    "Invalid/unknown idl instruction name",
                )),
            }
        } else {
            Err(ParseError::new_spanned(
                nested,
                "#[idl_instruction] attr can only have one idl instruction name",
            ))
        }
    }

    pub fn to_accounts(&self, ident: Ident) -> InstructionAccounts {
        match self {
            IdlInstruction::Create => InstructionAccounts(vec![InstructionAccount {
                ident: ident.clone(),
                index: Some(0),
                name: "from".to_string(),
                desc: Some("Payer of the transaction".to_string()),
                signer: true,
                optional_signer: false,
                writable: true,
                optional: false,
            }, InstructionAccount {
                ident: ident.clone(),
                index: Some(1),
                name: "to".to_string(),
                desc: Some("The deterministically defined 'state' account being created via `create_account_with_seed`".to_string()),
                signer: false,
                optional_signer: false,
                writable: true,
                optional: false,
            }, InstructionAccount {
                ident: ident.clone(),
                index: Some(2),
                name: "base".to_string(),
                desc: Some("The program-derived-address signing off on the account creation. Seeds = &[] + bump seed.".to_string()),
                signer: false,
                optional_signer: false,
                writable: false,
                optional: false,
            }, InstructionAccount {
                ident: ident.clone(),
                index: Some(3),
                name: "system_program".to_string(),
                desc: Some("The system program".to_string()),
                signer: false,
                optional_signer: false,
                writable: false,
                optional: false,
            }, InstructionAccount {
                ident,
                index: Some(4),
                name: "program".to_string(),
                desc: Some("The program whose state is being constructed".to_string()),
                signer: false,
                optional_signer: false,
                writable: false,
                optional: false,
            }]),
            IdlInstruction::CreateBuffer =>
                InstructionAccounts(vec![InstructionAccount {
                        ident: ident.clone(),
                        index: Some(0),
                        name: "buffer".to_string(),
                        desc: None,
                        signer: false,
                        optional_signer: false,
                        writable: true,
                        optional: false,
                    }, InstructionAccount {
                        ident,
                        index: Some(1),
                        name: "authority".to_string(),
                        desc: None,
                        signer: true,
                        optional_signer: false,
                        writable: false,
                        optional: false,
                    }]),
            IdlInstruction::SetBuffer =>
                InstructionAccounts(vec![InstructionAccount {
                        ident: ident.clone(),
                        index: Some(0),
                        name: "buffer".to_string(),
                        desc: Some("The buffer with the new idl data.".to_string()),
                        signer: false,
                        optional_signer: false,
                        writable: true,
                        optional: false,
                    }, InstructionAccount {
                        ident: ident.clone(),
                        index: Some(1),
                        name: "idl".to_string(),
                        desc: Some("The idl account to be updated with the buffer's data.".to_string()),
                        signer: false,
                        optional_signer: false,
                        writable: true,
                        optional: false,
                    }, InstructionAccount {
                        ident,
                        index: Some(2),
                        name: "authority".to_string(),
                        desc: None,
                        signer: true,
                        optional_signer: false,
                        writable: false,
                        optional: false,
                    }]),
            IdlInstruction::SetAuthority | IdlInstruction::Write =>
                InstructionAccounts(vec![InstructionAccount {
                        ident: ident.clone(),
                        index: Some(0),
                        name: "idl".to_string(),
                        desc: None,
                        signer: false,
                        optional_signer: false,
                        writable: true,
                        optional: false,
                    }, InstructionAccount {
                        ident,
                        index: Some(2),
                        name: "authority".to_string(),
                        desc: None,
                        signer: true,
                        optional_signer: false,
                        writable: false,
                        optional: false,
                    }]),
        }
    }

    pub fn to_instruction_fields(
        &self,
        ident: Ident,
    ) -> InstructionVariantFields {
        match self {
            IdlInstruction::Create => InstructionVariantFields::Named(vec![(
                "data_len".to_string(),
                RustType {
                    ident,
                    kind: TypeKind::Primitive(Primitive::U64),
                    context: RustTypeContext::Default,
                    reference: crate::types::ParsedReference::Owned,
                },
            )]),
            IdlInstruction::SetAuthority => {
                InstructionVariantFields::Named(vec![(
                    "new_authority".to_string(),
                    RustType {
                        ident,
                        kind: TypeKind::Value(Value::Custom(
                            "Pubkey".to_string(),
                        )),
                        context: RustTypeContext::Default,
                        reference: crate::types::ParsedReference::Owned,
                    },
                )])
            }
            IdlInstruction::Write => InstructionVariantFields::Named(vec![(
                "idl_data".to_string(),
                RustType {
                    ident: ident.clone(),
                    kind: TypeKind::Composite(
                        Composite::Vec,
                        vec![RustType {
                            ident,
                            kind: TypeKind::Primitive(Primitive::U8),
                            context: RustTypeContext::CollectionItem,
                            reference: crate::types::ParsedReference::Owned,
                        }],
                    ),
                    context: RustTypeContext::Default,
                    reference: crate::types::ParsedReference::Owned,
                },
            )]),
            IdlInstruction::CreateBuffer | IdlInstruction::SetBuffer => {
                InstructionVariantFields::Unnamed(vec![])
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum IdlInstructionError {
    TooManyIdlInstructions(ParseError),
    NotEnoughIdlInstructions,
    OtherErr(syn::Error),
}

impl fmt::Display for IdlInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IdlInstructionError::TooManyIdlInstructions(err) => {
                write!(f, "{}", err)
            }
            IdlInstructionError::NotEnoughIdlInstructions => {
                write!(f, "no #[idl_instruction] attributes found")
            }
            IdlInstructionError::OtherErr(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl TryFrom<&[Attribute]> for IdlInstruction {
    type Error = IdlInstructionError;

    fn try_from(attrs: &[Attribute]) -> Result<Self, IdlInstructionError> {
        let idl_instructions = attrs
            .iter()
            .filter_map(IdlInstruction::is_idl_instruction_attr)
            .map(IdlInstruction::from_idl_instruction_attr)
            .collect::<ParseResult<Vec<IdlInstruction>>>()
            .map_err(IdlInstructionError::OtherErr)?;

        if idl_instructions.len() > 1 {
            Err(IdlInstructionError::TooManyIdlInstructions(
                ParseError::new_spanned(
                    attrs.first(),
                    "Only one #[idl_instruction] attr is allowed per instruction",
                ),
            ))
        } else if idl_instructions.is_empty() {
            Err(IdlInstructionError::NotEnoughIdlInstructions)
        } else {
            let ix = *idl_instructions.first().unwrap();
            Ok(ix)
        }
    }
}
