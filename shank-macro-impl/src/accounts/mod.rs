use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;
use syn::{DeriveInput, Error as ParseError, Fields, Result};

mod account_field;
mod account_struct;

pub use account_field::AccountField;
pub use account_struct::AccountStruct;

pub struct Accounts {
    ident: syn::Ident,
    generics: syn::Generics,
    account_struct: AccountStruct,
}

impl TryFrom<DeriveInput> for Accounts {
    type Error = ParseError;

    fn try_from(input: DeriveInput) -> Result<Self> {
        // Only support structs with named fields
        let fields = match input.data {
            syn::Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => fields.named,
                _ => {
                    return Err(ParseError::new_spanned(
                        &input.ident,
                        "ShankAccounts can only be derived for structs with named fields",
                    ))
                }
            },
            _ => {
                return Err(ParseError::new_spanned(
                    &input.ident,
                    "ShankAccounts can only be derived for structs",
                ))
            }
        };

        let account_struct = AccountStruct::from_fields(fields)?;

        Ok(Self {
            ident: input.ident,
            generics: input.generics,
            account_struct,
        })
    }
}

impl Accounts {
    pub fn gen_impl(&self) -> TokenStream {
        let ident = &self.ident;
        let (impl_gen, type_gen, where_clause) = self.generics.split_for_impl();
        let account_list = self.account_struct.gen_account_list();
        let context_impl = self.gen_context_impl();

        quote! {
            impl #impl_gen #ident #type_gen #where_clause {
                #[doc(hidden)]
                pub fn __shank_accounts() -> Vec<(u32, &'static str, bool, bool, bool, bool, Option<String>)> {
                    vec![
                        #account_list
                    ]
                }
            }

            // Generate context implementation
            #context_impl
        }
    }

    fn gen_context_impl(&self) -> TokenStream {
        let ident = &self.ident;
        let (impl_gen, type_gen, where_clause) = self.generics.split_for_impl();
        let fields = &self.account_struct.fields;
        
        // All accounts must be provided, but optional ones can be program_id placeholders
        let expected_accounts = fields.len();
        let _total_accounts = fields.len();
        
        // Use the same lifetime as the struct, or skip context method if no lifetimes
        let context_method = if let Some(lifetime) = self.generics.lifetimes().next() {
            let lifetime_ident = &lifetime.lifetime;
            
            let account_assignments = fields.iter().enumerate().map(|(idx, field)| {
                let field_name = &field.name;
                if field.optional || field.optional_signer {
                    quote! {
                        #field_name: if accounts[#idx].key == &crate::ID {
                            None
                        } else {
                            Some(&accounts[#idx])
                        }
                    }
                } else {
                    quote! {
                        #field_name: &accounts[#idx]
                    }
                }
            });

            quote! {
                /// Create a context from a slice of accounts
                /// 
                /// This method parses the accounts according to the struct definition
                /// and returns a Context containing the account struct.
                /// 
                /// Optional accounts are determined by checking if the account key 
                /// equals the program ID (crate::ID). If so, they are set to None, otherwise Some.
                pub fn context(
                    accounts: &#lifetime_ident [AccountInfo<#lifetime_ident>]
                ) -> ::shank::Context<#lifetime_ident, Self, AccountInfo<#lifetime_ident>> {
                    if accounts.len() < #expected_accounts {
                        panic!("Expected at least {} accounts, got {}", #expected_accounts, accounts.len());
                    }

                    let account_struct = Self {
                        #(#account_assignments,)*
                    };

                    ::shank::Context {
                        accounts: account_struct,
                        remaining_accounts: &accounts[#expected_accounts..],
                    }
                }
            }
        } else {
            // No lifetime parameters, don't generate context method
            quote! {}
        };

        quote! {
            impl #impl_gen #ident #type_gen #where_clause {
                #context_method
            }
        }
    }
}