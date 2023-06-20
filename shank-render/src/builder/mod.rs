use proc_macro2::TokenStream;
use shank_macro_impl::builder::Builder;
use shank_macro_impl::syn::Result as ParseResult;

mod render_builders;
use self::render_builders::generate_builders;
/*
pub trait InstructionBuilder {
    fn instruction(&self) -> solana_program::instruction::Instruction;
}
*/
pub fn render_builders_impl(context: &Builder) -> ParseResult<TokenStream> {
    Ok(generate_builders(context))
}
