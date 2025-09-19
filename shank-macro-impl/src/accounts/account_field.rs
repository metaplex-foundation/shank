use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident, Lit, Meta, MetaNameValue, NestedMeta, Result};

pub struct AccountField {
    pub name: Ident,
    pub writable: bool,
    pub signer: bool,
    pub optional_signer: bool,
    pub optional: bool,
    pub desc: Option<String>,
}

impl AccountField {
    pub fn from_field(field: &Field) -> Result<Self> {
        let name = field.ident.clone().unwrap();
        let mut writable = false;
        let mut signer = false;
        let mut optional_signer = false;
        let mut optional = false;
        let mut desc = None;

        // Parse #[account(...)] attributes
        for attr in &field.attrs {
            if attr.path.is_ident("account") {
                let meta = attr.parse_meta()?;
                if let Meta::List(list) = meta {
                    for nested in list.nested {
                        match nested {
                            NestedMeta::Meta(Meta::Path(path)) => {
                                if path.is_ident("writable") || path.is_ident("mut") || path.is_ident("write") || path.is_ident("writ") || path.is_ident("w") {
                                    writable = true;
                                } else if path.is_ident("signer") || path.is_ident("sign") || path.is_ident("sig") || path.is_ident("s") {
                                    signer = true;
                                } else if path.is_ident("optional_signer") {
                                    optional_signer = true;
                                } else if path.is_ident("optional") || path.is_ident("option") || path.is_ident("opt") {
                                    optional = true;
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        &path,
                                        format!("Unknown account attribute: {:?}", path.get_ident()),
                                    ));
                                }
                            }
                            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) => {
                                if path.is_ident("desc") || path.is_ident("description") || path.is_ident("docs") {
                                    if let Lit::Str(s) = lit {
                                        desc = Some(s.value());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Validate mutually exclusive attributes
        if signer && optional_signer {
            return Err(syn::Error::new_spanned(
                &name,
                "Account cannot be both 'signer' and 'optional_signer'",
            ));
        }

        Ok(Self {
            name,
            writable,
            signer,
            optional_signer,
            optional,
            desc,
        })
    }

    pub fn gen_account_metadata(&self, index: usize) -> TokenStream {
        let name_str = self.name.to_string();
        let index = index as u32;
        let writable = self.writable;
        let signer = self.signer;
        let optional_signer = self.optional_signer;
        let optional = self.optional;
        let desc = match &self.desc {
            Some(d) => quote! { Some(#d.to_string()) },
            None => quote! { None },
        };

        // Generate a tuple with the account metadata that doesn't require InstructionAccount to be public
        quote! {
            (
                #index,
                #name_str,
                #writable,
                #signer,
                #optional_signer,
                #optional,
                #desc,
            )
        }
    }
}