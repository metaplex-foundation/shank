use std::convert::TryFrom;

use proc_macro2::Span;

use syn::{
    punctuated::Punctuated, Attribute, Error as ParseError, Ident, Lit, Meta,
    MetaList, NestedMeta, Result as ParseResult, Token,
};

use crate::parsed_enum::{ParsedEnum, ParsedEnumVariant};

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramError {
    pub variant_ident: Ident,
    pub attr_ident: Ident,
    pub code: u32,
    pub name: String,
    pub desc: String,
}

const THIS_ERROR: &str = "error";
impl ProgramError {
    fn is_error_attr(attr: &Attribute) -> Option<&Attribute> {
        match attr
            .path
            .get_ident()
            .map(|x| x.to_string().as_str() == THIS_ERROR)
        {
            Some(true) => Some(attr),
            _ => None,
        }
    }
    pub fn from_error_attr(
        attr: &Attribute,
        variant_ident: &Ident,
        variant_discriminant: u32,
    ) -> ParseResult<ProgramError> {
        let meta = &attr.parse_meta()?;
        match meta {
            Meta::List(MetaList { nested, .. }) => {
                let ident = attr.path.get_ident().map_or_else(
                    || Ident::new("attr_ident", Span::call_site()),
                    |x| x.clone(),
                );
                Self::parse_account_error_args(
                    &nested,
                    &ident,
                    variant_ident,
                    variant_discriminant,
                )
            }
            Meta::Path(_) | Meta::NameValue(_) => Err(ParseError::new_spanned(
                attr,
                "#[error] attr requires list of arguments",
            )),
        }
    }

    fn parse_account_error_args(
        nested: &Punctuated<NestedMeta, Token![,]>,
        attr_ident: &Ident,
        variant_ident: &Ident,
        variant_discriminant: u32,
    ) -> ParseResult<ProgramError> {
        if nested.len() != 1 {
            return Err(ParseError::new_spanned(
                nested,
                "shank supports only #[error]s with exactly the error message string",
            ));
        }
        let meta = &nested[0];
        match meta {
            NestedMeta::Lit(Lit::Str(lit_str)) => {
                let desc = lit_str.value();
                Ok(ProgramError {
                    attr_ident: attr_ident.clone(),
                    variant_ident: variant_ident.clone(),
                    code: variant_discriminant,
                    name: variant_ident.to_string(),
                    desc,
                })
            }
            _ => Err(ParseError::new_spanned(
                nested,
                "shank supports only #[error]s with exactly the error message string",
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ProgramErrors(pub Vec<ProgramError>);
impl TryFrom<&ParsedEnum> for ProgramErrors {
    type Error = ParseError;

    fn try_from(parsed_enum: &ParsedEnum) -> ParseResult<Self> {
        let program_errors = parsed_enum
            .variants
            .iter()
            .map(ProgramError::try_from_variant)
            .collect::<ParseResult<Vec<Option<ProgramError>>>>()?;
        let program_errors = program_errors
            .into_iter()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<ProgramError>>();

        Ok(ProgramErrors(program_errors))
    }
}
impl ProgramError {
    fn try_from_variant(
        variant: &ParsedEnumVariant,
    ) -> ParseResult<Option<Self>> {
        let program_errors = variant
            .attrs
            .iter()
            .filter_map(ProgramError::is_error_attr)
            .map(|attr| {
                ProgramError::from_error_attr(
                    attr,
                    &variant.ident,
                    variant.discriminant as u32,
                )
            })
            .collect::<ParseResult<Vec<ProgramError>>>()?;
        if program_errors.len() > 1 {
            Err(ParseError::new_spanned(
                &variant.ident,
                "shank expects no more than one #[error]s per variant",
            ))
        } else {
            Ok(program_errors.to_owned().first().map(|x| x.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::ItemEnum;

    use super::*;

    fn parse_enum_errors(code: TokenStream) -> ParseResult<ProgramErrors> {
        let item_enum =
            syn::parse2::<ItemEnum>(code).expect("Should parse successfully");
        let parsed_enum = ParsedEnum::try_from(&item_enum)?;

        ProgramErrors::try_from(&parsed_enum)
    }

    #[test]
    fn enum_with_one_error_attr() {
        let program_errors = parse_enum_errors(quote! {
            pub enum VaultError {
                /// Invalid instruction data passed in.
                #[error("Failed to unpack instruction data")]
                InstructionUnpackError,
            }
        })
        .expect("Should parse fine")
        .0;

        assert_eq!(program_errors.len(), 1, "extracts one program error");
        assert_eq!(program_errors[0].code, 0);
        assert_eq!(program_errors[0].name, "InstructionUnpackError");
        assert_eq!(program_errors[0].desc, "Failed to unpack instruction data");
    }

    #[test]
    fn enum_with_two_error_attrs() {
        let program_errors = parse_enum_errors(quote! {
            pub enum VaultError {
                /// Lamport balance below rent-exempt threshold.
                #[error("Lamport balance below rent-exempt threshold")]
                NotRentExempt,

                /// Already initialized
                #[error("Already initialized")]
                AlreadyInitialized,
            }
        })
        .expect("Should parse fine")
        .0;

        assert_eq!(program_errors.len(), 2, "extracts two program errors");
        assert_eq!(program_errors[0].code, 0);
        assert_eq!(program_errors[0].name, "NotRentExempt");
        assert_eq!(program_errors[1].code, 1);
        assert_eq!(
            program_errors[0].desc,
            "Lamport balance below rent-exempt threshold"
        );
        assert_eq!(program_errors[1].name, "AlreadyInitialized");
        assert_eq!(program_errors[1].desc, "Already initialized");
    }

    #[test]
    fn enum_with_two_error_attrs_and_extra_variants() {
        let program_errors = parse_enum_errors(quote! {
            pub enum VaultError {
                /// Lamport balance below rent-exempt threshold.
                #[error("Lamport balance below rent-exempt threshold")]
                NotRentExempt,

                #[not_error("hello")]
                NotAnError,

                AlsoNotAnError,

                /// Already initialized
                #[error("Already initialized")]
                AlreadyInitialized,
            }
        })
        .expect("Should parse fine")
        .0;

        assert_eq!(program_errors.len(), 2, "extracts two program errors");
        assert_eq!(program_errors[0].name, "NotRentExempt");
        assert_eq!(
            program_errors[0].desc,
            "Lamport balance below rent-exempt threshold"
        );
        assert_eq!(program_errors[1].name, "AlreadyInitialized");
        assert_eq!(program_errors[1].desc, "Already initialized");
    }

    #[test]
    fn enum_without_error_attrs() {
        let program_errors = parse_enum_errors(quote! {
            pub enum VaultError {
                #[not_error("hello")]
                NotAnError,

                AlsoNotAnError,
            }
        })
        .expect("Should parse fine")
        .0;

        assert_eq!(program_errors.len(), 0, "extracts no program error");
    }

    #[test]
    fn enum_with_two_error_attrs_discriminant_starting_at_3000() {
        let program_errors = parse_enum_errors(quote! {
            pub enum VaultError {
                /// Lamport balance below rent-exempt threshold.
                #[error("Lamport balance below rent-exempt threshold")]
                NotRentExempt = 3000,

                /// Already initialized
                #[error("Already initialized")]
                AlreadyInitialized,
            }
        })
        .expect("Should parse fine")
        .0;

        assert_eq!(program_errors.len(), 2, "extracts two program errors");
        assert_eq!(program_errors[0].code, 3000);
        assert_eq!(program_errors[0].name, "NotRentExempt");
        assert_eq!(program_errors[1].code, 3001);
        assert_eq!(
            program_errors[0].desc,
            "Lamport balance below rent-exempt threshold"
        );
        assert_eq!(program_errors[1].name, "AlreadyInitialized");
        assert_eq!(program_errors[1].desc, "Already initialized");
    }

    #[test]
    fn enum_with_two_error_attrs_two_exlicit_and_one_implicit_discriminants() {
        let program_errors = parse_enum_errors(quote! {
            pub enum VaultError {
                #[error("Lamport balance below rent-exempt threshold")]
                NotRentExempt = 333,

                #[error("Not allowed")]
                NotAllowed,

                #[error("Already initialized")]
                AlreadyInitialized = 222,
            }
        })
        .expect("Should parse fine")
        .0;

        assert_eq!(program_errors.len(), 3, "extracts three program errors");
        assert_eq!(program_errors[0].code, 333);
        assert_eq!(program_errors[1].code, 334);
        assert_eq!(program_errors[2].code, 222);
    }
}
