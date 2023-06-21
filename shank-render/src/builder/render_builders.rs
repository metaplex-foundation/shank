use proc_macro2::TokenStream;
use quote::quote;
use shank_macro_impl::{
    builder::Builder, instruction::InstructionVariantFields, syn,
};
use std::collections::HashMap;

pub(crate) fn generate_builders(context: &Builder) -> TokenStream {
    let mut default_pubkeys = HashMap::new();
    default_pubkeys.insert(
        "system_program".to_string(),
        syn::parse_str::<syn::ExprPath>("solana_program::system_program::ID")
            .unwrap(),
    );
    default_pubkeys.insert(
        "spl_token_program".to_string(),
        syn::parse_str::<syn::ExprPath>("spl_token::ID").unwrap(),
    );
    default_pubkeys.insert(
        "spl_ata_program".to_string(),
        syn::parse_str::<syn::ExprPath>("spl_associated_token_account::ID")
            .unwrap(),
    );
    default_pubkeys.insert(
        "sysvar_instructions".to_string(),
        syn::parse_str::<syn::ExprPath>(
            "solana_program::sysvar::instructions::ID",
        )
        .unwrap(),
    );
    default_pubkeys.insert(
        "authorization_rules_program".to_string(),
        syn::parse_str::<syn::ExprPath>("mpl_token_auth_rules::ID").unwrap(),
    );

    let variant_structs = context.variants.iter().map(|variant| {
        // struct block for the builder: this will contain both accounts and
        // args for the builder

        // accounts
        let struct_accounts = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            if account.optional {
                quote! {
                    pub #account_name: Option<solana_program::pubkey::Pubkey>
                }
            } else {
                quote! {
                    pub #account_name: solana_program::pubkey::Pubkey
                }
            }
        });

        // args
        let struct_args = variant.arguments.iter().map(|argument| {
            let ident_ty = syn::parse_str::<syn::Ident>(&argument.ty).unwrap();
            let arg_ty = if let Some(genetic_ty) = &argument.generic_ty {
                let arg_generic_ty =
                    syn::parse_str::<syn::Ident>(genetic_ty).unwrap();
                quote! { #ident_ty<#arg_generic_ty> }
            } else {
                quote! { #ident_ty }
            };
            let arg_name = syn::parse_str::<syn::Ident>(&argument.name).unwrap();

            quote! {
                pub #arg_name: #arg_ty
            }
        });

        // builder block: this will have all accounts and args as optional fields
        // that need to be set before the build method is called

        // accounts
        let builder_accounts = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            quote! {
                pub #account_name: Option<solana_program::pubkey::Pubkey>
            }
        });

        // accounts initialization
        let builder_initialize_accounts = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            quote! {
                #account_name: None
            }
        });

        // args
        let builder_args = variant.arguments.iter().map(|argument| {
            let ident_ty = syn::parse_str::<syn::Ident>(&argument.ty).unwrap();
            let arg_ty = if let Some(genetic_ty) = &argument.generic_ty {
                let arg_generic_ty =
                    syn::parse_str::<syn::Ident>(genetic_ty).unwrap();
                quote! { #ident_ty<#arg_generic_ty> }
            } else {
                quote! { #ident_ty }
            };
            let arg_name = syn::parse_str::<syn::Ident>(&argument.name).unwrap();

            quote! {
                pub #arg_name: Option<#arg_ty>
            }
        });

        // args initialization
        let builder_initialize_args =
        variant.arguments.iter().map(|argument| {
            let arg_name = syn::parse_str::<syn::Ident>(&argument.name).unwrap();
            quote! {
                #arg_name: None
            }
        });

        // account setter methods
        let builder_accounts_methods = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            quote! {
                pub fn #account_name(&mut self, #account_name: solana_program::pubkey::Pubkey) -> &mut Self {
                    self.#account_name = Some(#account_name);
                    self
                }
            }
        });

        // args setter methods
        let builder_args_methods =
            variant.arguments.iter().map(|argument| {
                let ident_ty = syn::parse_str::<syn::Ident>(&argument.ty).unwrap();
                let arg_ty = if let Some(genetic_ty) = &argument.generic_ty {
                    let arg_generic_ty =
                        syn::parse_str::<syn::Ident>(genetic_ty).unwrap();
                    quote! { #ident_ty<#arg_generic_ty> }
                } else {
                    quote! { #ident_ty }
                };
                let arg_name = syn::parse_str::<syn::Ident>(&argument.name).unwrap();

                quote! {
                    pub fn #arg_name(&mut self, #arg_name: #arg_ty) -> &mut Self {
                        self.#arg_name = Some(#arg_name);
                        self
                    }
                }
            });

        // required accounts
        let required_accounts = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();

            if account.optional {
                quote! {
                    #account_name: self.#account_name
                }
            } else {
                // are we dealing with a default pubkey?
                if default_pubkeys.contains_key(&account.name) {
                    let pubkey = default_pubkeys.get(&account.name).unwrap();
                    // we add the default key as the fallback value
                    quote! {
                        #account_name: self.#account_name.unwrap_or(#pubkey)
                    }
                }
                else {
                    // if not a default pubkey, we will need to have it set
                    quote! {
                        #account_name: self.#account_name.ok_or(concat!(stringify!(#account_name), " is not set"))?
                    }
                }
            }
        });

        // required args
        let required_args = variant.arguments.iter().map(|argument| {
            let arg_name = syn::parse_str::<syn::Ident>(&argument.name).unwrap();
            quote! {
                #arg_name: self.#arg_name.clone().ok_or(concat!(stringify!(#arg_name), " is not set"))?
            }
        });

        // args parameter list
        let args: Vec<TokenStream> = match &variant.field_tys {
            InstructionVariantFields::Named(field_tys) => {
                field_tys.iter().map(|(name, ty)| {
                    let name = syn::parse_str::<syn::Ident>(name).unwrap();
                    let ty = &ty.ident;

                    quote! { #name: #ty }
                }).collect()
            }
            InstructionVariantFields::Unnamed(field_tys) => {
                field_tys.iter().enumerate().map(|(idx, ty)| {
                    let name = syn::parse_str::<syn::Ident>(&format!("arg{idx}")).unwrap();
                    let ty = &ty.ident;

                    quote! { #name: #ty }
                }).collect()
            }
        };

        // instruction args
        let instruction_args: Vec<TokenStream> = match &variant.field_tys {
            InstructionVariantFields::Named(field_tys) => {
                field_tys.iter().map(|(name, ty)| {
                    let name = syn::parse_str::<syn::Ident>(name).unwrap();
                    let ty = &ty.ident;

                    quote! { pub #name: #ty }
                }).collect()
            }
            InstructionVariantFields::Unnamed(field_tys) => {
                field_tys.iter().enumerate().map(|(idx, ty)| {
                    let name = syn::parse_str::<syn::Ident>(&format!("arg{idx}")).unwrap();
                    let ty = &ty.ident;

                    quote! { pub #name: #ty }
                }).collect()
            }
        };

        // required instruction args
        let required_instruction_args: Vec<TokenStream> = match &variant.field_tys {
            InstructionVariantFields::Named(field_tys) => {
                field_tys.iter().map(|(name, _)| {
                    let name = syn::parse_str::<syn::Ident>(name).unwrap();
                    quote! { #name }
                }).collect()
            }
            InstructionVariantFields::Unnamed(field_tys) => {
                field_tys.iter().enumerate().map(|(idx, _)| {
                    let name = syn::parse_str::<syn::Ident>(&format!("arg{idx}")).unwrap();
                    quote! { #name }
                }).collect()
            }
        };

        /*
        // instruction args
        let instruction_args = if let Some(args) = &variant.tuple {
            let arg_ty = syn::parse_str::<syn::Ident>(args).unwrap();
            quote! { pub args: #arg_ty, }
        } else {
            quote! { }
        };

        // required instruction args
        let required_instruction_args = if variant.tuple.is_some() {
            quote! { args, }
        } else {
            quote! { }
        };
        */

        // builder name
        let name = &variant.ident;
        let builder_name = syn::parse_str::<syn::Ident>(&format!("{}Builder", name)).unwrap();

        quote! {
            pub struct #name {
                #(#struct_accounts,)*
                #(#struct_args,)*
                #(#instruction_args,)*
            }

            pub struct #builder_name {
                #(#builder_accounts,)*
                #(#builder_args,)*
            }

            impl #builder_name {
                pub fn new() -> Box<#builder_name> {
                    Box::new(#builder_name {
                        #(#builder_initialize_accounts,)*
                        #(#builder_initialize_args,)*
                    })
                }

                #(#builder_accounts_methods)*
                #(#builder_args_methods)*

                pub fn build(&mut self, #(#args,)*) -> Result<Box<#name>, Box<dyn std::error::Error>> {
                    Ok(Box::new(#name {
                        #(#required_accounts,)*
                        #(#required_args,)*
                        #(#required_instruction_args,)*
                    }))
                }
            }
        }
    });

    quote! {
        pub mod builders {
            use super::*;

            #(#variant_structs)*
        }
    }
}
