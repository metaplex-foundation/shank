use proc_macro2::TokenStream;
use quote::quote;
use shank_macro_impl::{instruction::InstructionVariant, syn};

pub(crate) fn generate_context(variant: &InstructionVariant) -> TokenStream {
    // Define the account type path based on feature flags
    let account_info_type = if cfg!(feature = "pinocchio") {
        quote! { pinocchio::account_info::AccountInfo }
    } else {
        quote! { solana_program::account_info::AccountInfo<'a> }
    };

    // Define the ProgramError type path based on feature flags
    let program_error_type = if cfg!(feature = "pinocchio") {
        quote! { pinocchio::program_error::ProgramError }
    } else {
        quote! { solana_program::sysvar::slot_history::ProgramError> }
    };

    // Define how to retrieve the key of the account
    let key_retrieval = if cfg!(feature = "pinocchio") {
        quote! { key() }
    } else {
        quote! { key }
    };

    // accounts fields
    let struct_fields = variant.accounts.iter().map(|account| {
        let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
        if account.optional {
            quote! {
                pub #account_name: Option<&'a #account_info_type>
            }
        } else {
            quote! {
                pub #account_name: &'a #account_info_type
            }
        }
    });

    // accounts initialization
    let account_fields = variant.accounts.iter().enumerate().map(|(index, account)| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            if account.optional {
                quote! {
                    #account_name: if accounts[#index].#key_retrieval == &crate::ID { None } else { Some(&accounts[#index]) }
                }
            } else {
                quote! {
                    #account_name: &accounts[#index]
                }
            }
        });

    let expected = variant.accounts.len();
    let name =
        syn::parse_str::<syn::Ident>(&format!("{}Accounts", variant.ident))
            .unwrap();

    // Use the same account type in the impl block
    quote! {
        pub struct #name<'a> {
            #(#struct_fields,)*
        }
        impl<'a> #name<'a> {
            pub fn context(
                accounts: &'a [#account_info_type]
            ) -> Result<Context<'a, Self>, #program_error_type> {
                if accounts.len() < #expected {
                    return Err(#program_error_type::NotEnoughAccountKeys);
                }

                Ok(Context {
                    accounts: Self { #(#account_fields,)* },
                    remaining_accounts: &accounts[#expected..],
                })
            }
        }
    }
}
