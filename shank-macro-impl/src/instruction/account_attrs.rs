use std::convert::TryFrom;

use syn::{
    punctuated::Punctuated, Attribute, Error as ParseError, Ident, Lit, Meta,
    MetaList, MetaNameValue, NestedMeta, Result as ParseResult, Token,
};

const IX_ACCOUNT: &str = "account";

#[derive(Debug, PartialEq)]
pub struct InstructionAccount {
    pub index: Option<u32>,
    pub name: String,
    pub writable: bool,
    pub signer: bool,
    pub desc: Option<String>,
}

#[derive(Debug)]
struct InstructionAccounts(Vec<InstructionAccount>);

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
                Self::parse_account_attr_args(&nested)
            }
            Meta::Path(_) | Meta::NameValue(_) => Err(ParseError::new_spanned(
                attr,
                "#[account] attr requires list of arguments",
            )),
        }
    }

    fn parse_account_attr_args(
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

        for meta in nested {
            if let Some((ident, name, value)) =
                string_assign_from_nested_meta(meta)?
            {
                // desc = "account description"
                match name.as_str() {
                    "desc" | "description" => desc = Some(value),
                    "name" => account_name = Some(value),
                    _ => return Err(ParseError::new_spanned(
                        ident,
                        "Ony desc/description or name can be assigned strings",
                    )),
                };
            } else if let Some((ident, name)) =
                identifier_from_nested_meta(meta)
            {
                // signer, writable ...
                match name.as_str() {
                    "signer" | "sign" | "sig" | "s" => signer = true,
                    "writable" | "write" | "writ" | "mut" | "w" => {
                        writable = true;
                    }
                    _ => {
                        return Err(ParseError::new_spanned(
                            ident,
                            "Invalid/unkown account meta configuration",
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
                index,
                name,
                writable,
                signer,
                desc,
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
        // TODO(thlorenz): verify that either all or none have indexes + that they match the index
        // inside the vec
        let vec: ParseResult<Vec<InstructionAccount>> = attrs
            .into_iter()
            .filter_map(InstructionAccount::is_account_attr)
            .map(InstructionAccount::from_account_attr)
            .collect();
        Ok(InstructionAccounts(vec?))
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

// -----------------
// Tests
// -----------------

#[cfg(test)]
mod tests {

    use std::convert::TryInto;

    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::ItemEnum;

    fn parse_first_enum_variant_attrs(
        code: TokenStream,
    ) -> ParseResult<InstructionAccounts> {
        let parsed =
            syn::parse2::<ItemEnum>(code).expect("Should parse successfully");
        let attrs: &[Attribute] =
            parsed.variants.first().unwrap().attrs.as_ref();
        attrs.try_into()
    }

    #[test]
    fn account_readonly() {
        let accounts_indexed = parse_first_enum_variant_attrs(quote! {
            #[derive(Instruction)]
            pub enum Instructions {
                #[account(0, name="authority")]
                Indexed
            }
        })
        .expect("Should parse fine");
        assert_eq!(
            accounts_indexed.0[0],
            InstructionAccount {
                index: Some(0,),
                name: "authority".to_string(),
                writable: false,
                signer: false,
                desc: None,
            }
        );

        let accounts = parse_first_enum_variant_attrs(quote! {
            #[derive(Instruction)]
            pub enum Instructions {
                #[account(name="authority")]
                NotIndexed
            }
        })
        .expect("Should parse fine");

        assert_eq!(
            accounts.0[0],
            InstructionAccount {
                index: None,
                name: "authority".to_string(),
                writable: false,
                signer: false,
                desc: None,
            }
        );
    }

    #[test]
    fn account_signer() {
        let accounts_indexed = parse_first_enum_variant_attrs(quote! {
            #[derive(Instruction)]
                pub enum Instructions {
                    #[account(0, signer, name = "authority")]
                    Indexed
                }
        })
        .expect("Should parse fine");
        assert_eq!(
            accounts_indexed.0[0],
            InstructionAccount {
                index: Some(0,),
                name: "authority".to_string(),
                writable: false,
                signer: true,
                desc: None,
            }
        );

        let accounts = parse_first_enum_variant_attrs(quote! {
            #[derive(Instruction)]
            pub enum Instructions {
                #[account(name="authority", sign)]
                NotIndexed
            }
        })
        .expect("Should parse fine");

        assert_eq!(
            accounts.0[0],
            InstructionAccount {
                index: None,
                name: "authority".to_string(),
                writable: false,
                signer: true,
                desc: None,
            }
        );
    }

    #[test]
    fn account_writable() {
        let accounts_indexed = parse_first_enum_variant_attrs(quote! {
            #[derive(Instruction)]
            pub enum Instructions {
                #[account(0, name="authority", writable)]
                Indexed
            }
        })
        .expect("Should parse fine");
        assert_eq!(
            accounts_indexed.0[0],
            InstructionAccount {
                index: Some(0,),
                name: "authority".to_string(),
                writable: true,
                signer: false,
                desc: None,
            }
        );

        let accounts = parse_first_enum_variant_attrs(quote! {
            #[derive(Instruction)]
            pub enum Instructions {
                #[account(w, name="authority")]
                NotIndexed
            }
        })
        .expect("Should parse fine");

        assert_eq!(
            accounts.0[0],
            InstructionAccount {
                index: None,
                name: "authority".to_string(),
                writable: true,
                signer: false,
                desc: None,
            }
        );
    }

    #[test]
    fn account_desc() {
        let accounts_indexed = parse_first_enum_variant_attrs(quote! {
            #[derive(Instruction)]
            pub enum Instructions {
                #[account(0, name ="funnel", desc = "Readonly indexed account description")]
                Indexed
            }
        })
        .expect("Should parse fine");

        assert_eq!(
            accounts_indexed.0[0],
            InstructionAccount {
                index: Some(0,),
                name: "funnel".to_string(),
                writable: false,
                signer: false,
                desc: Some("Readonly indexed account description".to_string()),
            }
        );
    }
}
