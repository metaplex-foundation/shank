use proc_macro2::{Ident, Span};
use std::convert::TryFrom;
use syn::{Error as ParseError, ItemMacro, Result as ParseResult};

use crate::parsed_macro::ParsedMacro;

/// Declared program id, i.e. `solana_program::declare_id!("<program id>")`
#[derive(Debug)]
pub struct ProgramId {
    pub id: String,
}

impl TryFrom<&[ItemMacro]> for ProgramId {
    type Error = ParseError;

    fn try_from(macros: &[ItemMacro]) -> ParseResult<Self> {
        let matches: Vec<(Ident, String)> = macros
            .iter()
            .map(ParsedMacro::from)
            .filter_map(
                |ParsedMacro {
                     path,
                     literal,
                     path_idents,
                 }| {
                    literal
                        .map(|lit| {
                            if path.ends_with("declare_id") {
                                Some((path_idents[0].clone(), lit.clone()))
                            } else {
                                None
                            }
                        })
                        .flatten()
                },
            )
            .collect();

        if matches.len() > 1 {
            Err(ParseError::new_spanned(
                &matches[0].0,
                format!(
                    "Found more than one program id candidate: {:?}. You should either have exactly one `declare_id!` in your code or override the program id via -p.",
                    matches.iter().map(|x| x.1.clone()).collect::<Vec<String>>()
                ),
            ))
        } else if matches.is_empty() {
            Err(ParseError::new(
                Span::call_site(),
                "Could not find a `declare_id(\"<program-id>\")` invocation in the program. If this is intentional provide a program address via the -p argument instead",
            ))
        } else {
            Ok(ProgramId {
                id: matches[0].1.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::ItemMacro;

    fn parse_program_id(codes: Vec<TokenStream>) -> ParseResult<ProgramId> {
        let item_macros = codes
            .into_iter()
            .map(syn::parse2::<ItemMacro>)
            .collect::<ParseResult<Vec<ItemMacro>>>()
            .expect("Should parse ItemMacro successfully");

        ProgramId::try_from(&item_macros[..])
    }

    #[test]
    fn program_id_qualified_solana_program() {
        let parsed = parse_program_id(vec![
            quote! {
                format!("Just another macro {}", s);
            },
            quote! {
                solana_program::declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
            },
        ])
        .expect("Should parse fine");

        assert_eq!(
            parsed.id,
            "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s".to_string()
        )
    }

    #[test]
    fn program_id_imported_solana_program() {
        let parsed = parse_program_id(vec![
            quote! {
                format!("Just another macro {}", s);
            },
            quote! {
                declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
            },
        ])
        .expect("Should parse fine");

        assert_eq!(
            parsed.id,
            "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s".to_string()
        )
    }

    #[test]
    fn program_id_two_declarations() {
        let err = parse_program_id(vec![
            quote! {
                declare_id!("otherid");
            },
            quote! {
                solana_program::declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
            },
        ])
        .expect_err("Should error");

        assert_eq!(
            err.to_string().as_str(),
            "Found more than one program id candidate: [\"otherid\", \"metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s\"]. You should either have exactly one `declare_id!` in your code or override the program id via -p."
        );
    }

    #[test]
    fn program_id_no_declaration() {
        let err = parse_program_id(vec![
            quote! {
                format!("Just another macro {}", s);
            },
            quote! {
                declare_some_other_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
            },
        ])
        .expect_err("Should error");

        assert_eq!(
            err.to_string().as_str(),
            "Could not find a `declare_id(\"<program-id>\")` invocation in the program. If this is intentional provide a program address via the -p argument instead"
        );
    }
}
