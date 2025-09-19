use proc_macro2::TokenStream;
use shank_macro_impl::accounts::Accounts;
use std::convert::TryFrom;
use syn::{DeriveInput, Error as ParseError};

pub fn derive_accounts(input: DeriveInput) -> Result<TokenStream, ParseError> {
    let accounts = Accounts::try_from(input)?;
    Ok(accounts.gen_impl())
}