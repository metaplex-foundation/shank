use instruction::derive_instruction;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error as ParseError};

mod instruction;

// -----------------
// #[derive(ShankAccount)]
// -----------------
#[proc_macro_derive(ShankAccount)]
pub fn shank_account(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

// -----------------
// #[derive(ShankInstructions)]
// -----------------
#[proc_macro_derive(ShankInstruction, attributes(account))]
pub fn shank_instruction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_instruction(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

fn to_compile_error(error: ParseError) -> proc_macro2::TokenStream {
    let compile_error = ParseError::to_compile_error(&error);
    quote!(#compile_error)
}
