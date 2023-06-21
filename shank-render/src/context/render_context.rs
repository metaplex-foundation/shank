use proc_macro2::TokenStream;
use quote::quote;
use shank_macro_impl::{instruction::InstructionVariant, syn};

pub(crate) fn generate_context(variant: &InstructionVariant) -> TokenStream {
    // accounts names
    let fields = variant.accounts.iter().map(|account| {
        let account_name = syn::parse_str::<syn::Ident>(
            format!("{}_info", &account.name).as_str(),
        )
        .unwrap();
        quote! { #account_name }
    });

    // accounts fields
    let struct_fields = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(format!("{}_info", &account.name).as_str()).unwrap();
            if account.optional {
                quote! {
                    pub #account_name: Option<&'a solana_program::account_info::AccountInfo<'a>>
                }
            } else {
                quote! {
                    pub #account_name:&'a solana_program::account_info::AccountInfo<'a>
                }
            }
        });

    // accounts initialization for the impl block
    let impl_fields = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(format!("{}_info", &account.name).as_str()).unwrap();
            if account.optional {
                quote! {
                    let #account_name = Self::next_optional_account_info(account_info_iter)?;
                }
            } else {
                quote! {
                    let #account_name = solana_program::account_info::next_account_info(account_info_iter)?;
                }
            }
        });

    let name = &variant.ident;

    quote! {
        pub struct #name<'a> {
            #(#struct_fields,)*
        }
        impl<'a> #name<'a> {
            pub fn to_context(
                accounts: &'a [solana_program::account_info::AccountInfo<'a>]
            ) -> Result<Context<'a, Self>, solana_program::sysvar::slot_history::ProgramError> {
                let account_info_iter = &mut accounts.iter();

                #(#impl_fields)*

                let accounts = Self {
                    #(#fields,)*
                };

                Ok(Context {
                    accounts,
                    remaining_accounts: Vec::<&'a solana_program::account_info::AccountInfo<'a>>::from_iter(account_info_iter),
                })
            }
        }
    }
}
