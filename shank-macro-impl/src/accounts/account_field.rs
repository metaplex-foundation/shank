use proc_macro2::Ident;

use syn::Result as ParseResult;

use super::{common::ident_string, parse_account_field_ty, Ty};

// -----------------
// Types
// -----------------

#[derive(Debug)]
pub enum AccountField {
    Field(Field),
    CompositeField(CompositeField),
}

impl AccountField {
    fn ident(&self) -> &Ident {
        match self {
            AccountField::Field(field) => &field.ident,
            AccountField::CompositeField(c_field) => &c_field.ident,
        }
    }
}

#[derive(Debug)]
pub struct Field {
    pub ident: Ident,
    pub ty: Ty,
}

#[derive(Debug)]
pub struct CompositeField {
    pub ident: Ident,
    pub symbol: String,
    pub raw_field: syn::Field,
}

// -----------------
// Parsers
// -----------------
pub fn parse_account_field(f: &syn::Field) -> ParseResult<AccountField> {
    let ident = f.ident.clone().unwrap();
    let account_field = match is_field_primitive(f)? {
        true => {
            let ty = parse_account_field_ty(f)?;
            AccountField::Field(Field { ident, ty })
        }
        false => AccountField::CompositeField(CompositeField {
            ident,
            symbol: ident_string(f)?,
            raw_field: f.clone(),
        }),
    };
    Ok(account_field)
}

fn is_field_primitive(f: &syn::Field) -> ParseResult<bool> {
    let r = matches!(
        ident_string(f)?.as_str(),
        "ProgramState"
            | "ProgramAccount"
            | "CpiAccount"
            | "Sysvar"
            | "AccountInfo"
            | "UncheckedAccount"
            | "CpiState"
            | "Loader"
            | "AccountLoader"
            | "Account"
            | "Program"
            | "Signer"
            | "SystemAccount"
            | "ProgramData"
    );
    Ok(r)
}
