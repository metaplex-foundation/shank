use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream},
    Error as ParseError, Generics, ItemStruct, Result as ParseResult,
};

use super::{parse_account_field, AccountField};

// -----------------
// Types
// -----------------
#[derive(Debug)]
pub struct AccountsStruct {
    // Name of the accounts struct.
    pub ident: Ident,
    // Generics + lifetimes on the accounts struct.
    pub generics: Generics,
    // Fields on the accounts struct.
    pub fields: Vec<AccountField>,
}

impl AccountsStruct {
    pub fn new(strct: ItemStruct, fields: Vec<AccountField>) -> Self {
        let ident = strct.ident.clone();
        let generics = strct.generics;
        Self {
            ident,
            generics,
            fields,
        }
    }
}

impl Parse for AccountsStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let strct = <ItemStruct as Parse>::parse(input)?;
        parse_accounts(&strct)
    }
}

// -----------------
// Parsers
// -----------------

pub fn parse_accounts(strct: &syn::ItemStruct) -> ParseResult<AccountsStruct> {
    let fields = match &strct.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(parse_account_field)
            .collect::<ParseResult<Vec<AccountField>>>()?,
        _ => {
            return Err(ParseError::new_spanned(
                &strct.fields,
                "fields must be named",
            ))
        }
    };

    Ok(AccountsStruct::new(strct.clone(), fields))
}
