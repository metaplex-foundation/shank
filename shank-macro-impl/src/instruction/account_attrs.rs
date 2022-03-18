use std::convert::TryFrom;

use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, Attribute, Error as ParseError, Ident, Lit, Meta,
    MetaList, MetaNameValue, NestedMeta, Result as ParseResult, Token,
};

const IX_ACCOUNT: &str = "account";

#[derive(Debug, PartialEq)]
pub struct InstructionAccount {
    pub ident: Ident,
    pub index: Option<u32>,
    pub name: String,
    pub writable: bool,
    pub signer: bool,
    pub desc: Option<String>,
    pub optional: bool,
}

#[derive(Debug, PartialEq)]
pub struct InstructionAccounts(pub Vec<InstructionAccount>);

impl InstructionAccount {
    fn is_account_attr(attr: &Attribute) -> Option<&Attribute> {
        match attr
            .path
            .get_ident()
            .map(|x| x.to_string().as_str() == IX_ACCOUNT)
        {
            Some(true) => Some(attr),
            _ => None,
        }
    }

    pub fn from_account_attr(
        attr: &Attribute,
    ) -> ParseResult<InstructionAccount> {
        let meta = &attr.parse_meta()?;

        match meta {
            Meta::List(MetaList { nested, .. }) => {
                let ident = attr.path.get_ident().map_or_else(
                    || Ident::new("attr_ident", Span::call_site()),
                    |x| x.clone(),
                );
                Self::parse_account_attr_args(ident, &nested)
            }
            Meta::Path(_) | Meta::NameValue(_) => Err(ParseError::new_spanned(
                attr,
                "#[account] attr requires list of arguments",
            )),
        }
    }

    fn parse_account_attr_args(
        ident: Ident,
        nested: &Punctuated<NestedMeta, Token![,]>,
    ) -> ParseResult<InstructionAccount> {
        if nested.is_empty() {
            return Err(ParseError::new_spanned(
                nested,
                "#[account] attr requires at least the account name",
            ));
        }

        let mut index: Option<u32> = None;
        let mut writable = false;
        let mut signer = false;
        let mut desc = None;
        let mut account_name = None;
        let mut optional = false;

        for meta in nested {
            if let Some((ident, name, value)) =
                string_assign_from_nested_meta(meta)?
            {
                // name/desc
                match name.as_str() {
                    "desc" | "description" => desc = Some(value),
                    "name" if value.trim().is_empty() => {
                        return Err(ParseError::new_spanned(
                            ident,
                            "account name cannot be empty",
                        ))
                    }
                    "name" => account_name = Some(value),
                    _ => return Err(ParseError::new_spanned(
                        ident,
                        "Only desc/description or name can be assigned strings",
                    )),
                };
            } else if let Some((ident, name)) =
                identifier_from_nested_meta(meta)
            {
                // signer, writable, optional ...
                match name.as_str() {
                    "signer" | "sign" | "sig" | "s" => signer = true,
                    "writable" | "write" | "writ" | "mut" | "w" => {
                        writable = true;
                    }
                    "optional" | "option" | "opt" => optional = true,
                    _ => {
                        return Err(ParseError::new_spanned(
                            ident,
                            "Invalid/unknown account meta configuration",
                        ));
                    }
                };
            } else {
                // account index (optional)
                match meta {
                    NestedMeta::Lit(Lit::Int(idx)) => {
                        index = Some(idx.base10_parse()?);
                    }
                    _ => {
                        return Err(ParseError::new_spanned(
                            meta,
                            "Invalid account specification",
                        ));
                    }
                }
            }
        }
        match account_name {
            Some(name) => Ok(Self {
                ident,
                index,
                name,
                writable,
                signer,
                desc,
                optional,
            }),
            None => {
                Err(ParseError::new_spanned(nested, "Missing account name"))
            }
        }
    }
}

impl TryFrom<&[Attribute]> for InstructionAccounts {
    type Error = ParseError;

    fn try_from(attrs: &[Attribute]) -> ParseResult<Self> {
        let accounts = attrs
            .into_iter()
            .filter_map(InstructionAccount::is_account_attr)
            .map(InstructionAccount::from_account_attr)
            .collect::<ParseResult<Vec<InstructionAccount>>>()?;

        for (idx, acc) in accounts.iter().enumerate() {
            match acc.index {
                Some(acc_idx) if acc_idx != idx as u32 => {
                    return Err(ParseError::new_spanned(
                        &acc.ident,
                        format!(
                            "Account index {} does not match its position {}",
                            acc_idx, idx,
                        ),
                    ));
                }
                _ => {}
            }
        }

        Ok(InstructionAccounts(accounts))
    }
}

// -----------------
// Meta Extractors
// -----------------
fn string_assign_from_nested_meta(
    nested_meta: &NestedMeta,
) -> ParseResult<Option<(Ident, String, String)>> {
    match nested_meta {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            path, lit, ..
        })) => {
            let ident = path.get_ident();
            if let Some(ident) = ident {
                let token =  match lit {
                    Lit::Str(lit) => Ok(lit.value()),
                    _ => Err(ParseError::new_spanned(ident, "#[account(desc)] arg needs to be assigning to a string")),
                }?;
                Ok(Some((ident.clone(), ident.to_string(), token)))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

fn identifier_from_nested_meta(
    nested_meta: &NestedMeta,
) -> Option<(Ident, String)> {
    match nested_meta {
        NestedMeta::Meta(meta) => match meta {
            Meta::Path(_) => {
                meta.path().get_ident().map(|x| (x.clone(), x.to_string()))
            }
            // ignore named values and lists
            _ => None,
        },
        _ => None,
    }
}
