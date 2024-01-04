use proc_macro2::TokenStream;
use quote::quote;
use shank_macro_impl::builder::Builder;
use shank_macro_impl::syn::Result as ParseResult;

mod render_builders;
use self::render_builders::generate_builders;

pub fn render_builders_impl(
    builder_item: &Builder,
) -> ParseResult<TokenStream> {
    let builders = builder_item
        .variants
        .iter()
        .map(|variant| generate_builders(&builder_item.ident, variant))
        .collect::<Vec<TokenStream>>();

    Ok(quote! {
            pub mod builders {
                use super::*;

                /// Trait that defines the interface for creating an instruction.
                pub trait InstructionBuilder {
                    fn instruction(&self) -> solana_program::instruction::Instruction;
                }

                #(#builders)*
        }
    })
}
