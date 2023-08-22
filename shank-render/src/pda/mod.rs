mod pda_common;
mod render_pda;
mod render_seeds;

pub use pda_common::*;
use proc_macro2::{Span, TokenStream};
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
    include_comments: bool,
) -> ParseResult<TokenStream> {
    let processed_seeds = try_process_seeds(struct_attrs)?;
    if processed_seeds.is_empty() {
        return Ok(TokenStream::new());
    }

    let seeds_fn_ident = Ident::new("shank_seeds", Span::call_site());
    let seeds_fn_with_bump_ident =
        Ident::new("shank_seeds_with_bump", Span::call_site());
    let pda_fn_ident = Ident::new("shank_pda", Span::call_site());
    let pda_fn_with_bump_ident =
        Ident::new("shank_pda_with_bump", Span::call_site());

    let pub_seeds_fn = try_render_seeds_fn(
        &processed_seeds,
        &seeds_fn_ident,
        &seeds_fn_with_bump_ident,
        include_comments,
    )?;
    let pub_pda_fn = render_pda_fn(
        &processed_seeds,
        &seeds_fn_ident,
        &seeds_fn_with_bump_ident,
        &pda_fn_ident,
        &pda_fn_with_bump_ident,
        include_comments,
    )?;

    if let (Some(pub_seeds_fn), Some(pub_pda_fn)) = (pub_seeds_fn, pub_pda_fn) {
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
