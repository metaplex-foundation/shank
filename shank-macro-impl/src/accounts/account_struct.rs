use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Field, Result, Token};

use super::AccountField;

pub struct AccountStruct {
    pub fields: Vec<AccountField>,
}

impl AccountStruct {
    pub fn from_fields(fields: Punctuated<Field, Token![,]>) -> Result<Self> {
        let mut account_fields = Vec::new();

        for field in fields {
            let account_field = AccountField::from_field(&field)?;
            account_fields.push(account_field);
        }

        Ok(Self {
            fields: account_fields,
        })
    }

    pub fn gen_account_list(&self) -> TokenStream {
        let accounts = self
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| field.gen_account_metadata(idx));

        quote! {
            #(#accounts),*
        }
    }

    pub fn to_instruction_accounts(&self) -> Vec<crate::instruction::InstructionAccount> {
        self.fields
            .iter()
            .enumerate()
            .map(|(idx, field)| crate::instruction::InstructionAccount {
                ident: field.name.clone(),
                index: Some(idx as u32),
                name: field.name.to_string(),
                writable: field.writable,
                signer: field.signer,
                optional_signer: field.optional_signer,
                desc: field.desc.clone(),
                optional: field.optional,
            })
            .collect()
    }
}