mod process_seeds;
mod render_pda;
mod render_seeds;

use proc_macro2::{Span, TokenStream};
pub use process_seeds::*;
pub use render_pda::*;
pub use render_seeds::*;

use quote::quote;
use shank_macro_impl::{
    parsed_struct::StructAttrs,
    syn::{Ident, Result as ParseResult},
};

pub fn render_pda_and_seeds_impl(
    struct_attrs: &StructAttrs,
    account_type_ident: &Ident,
) -> ParseResult<TokenStream> {
    let processed_seeds = try_process_seeds(struct_attrs)?;
    if processed_seeds.is_empty() {
        return Ok(TokenStream::new());
    }

    let seeds_fn_ident = Ident::new("shank_seeds", Span::call_site());
    let pda_fn_ident = Ident::new("shank_pda", Span::call_site());

    let pub_seeds_fn = try_render_seeds_fn(&processed_seeds, &seeds_fn_ident)?;
    let pub_pda_fn =
        render_pda_fn(&processed_seeds, &seeds_fn_ident, &pda_fn_ident);

    if let (Some(pub_seeds_fn), Some(pub_pda_fn)) = (pub_seeds_fn, pub_pda_fn) {
        // TODO(thlorenz): Include some helpful comments for each fn
        Ok(quote! {
            impl #account_type_ident {
                #pub_seeds_fn
                #pub_pda_fn
            }
        })
    } else {
        Ok(TokenStream::new())
    }
}
