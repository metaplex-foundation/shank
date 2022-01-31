use proc_macro::TokenStream;

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
pub fn shank_instruction(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
