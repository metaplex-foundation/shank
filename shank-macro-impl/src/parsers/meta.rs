//! Helpers to parse the arguments of list-style attributes such as
//! `#[account(0, writable, name = "foo", desc = "the foo")]`.
//!
//! `syn` 2 no longer pre-parses attribute arguments into `NestedMeta`s but
//! exposes them as raw tokens instead. The [`NestedMeta`] type below restores
//! the syn 1 shape (a meta item or a bare literal) so the shank attribute
//! grammar stays exactly what it was, while the underlying parser understands
//! every Rust edition that `syn` 2 supports.
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Error as ParseError, Ident, Lit, LitBool, Meta, MetaList, Path,
    Result as ParseResult, Token,
};

/// One comma separated argument of a list-style attribute.
#[derive(Debug, Clone)]
// Mirrors syn 1's `NestedMeta`; boxing `Meta` would complicate every match.
#[allow(clippy::large_enum_variant)]
pub enum NestedMeta {
    /// A path (`writable`), a list (`pubkey("desc")`) or an assignment
    /// (`name = "foo"`).
    Meta(Meta),
    /// A bare literal such as `0` or `"seed"`.
    Lit(Lit),
}

/// The comma separated arguments of a list-style attribute.
pub type NestedMetas = Punctuated<NestedMeta, Token![,]>;

impl Parse for NestedMeta {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if input.peek(Lit) && !(input.peek(LitBool) && input.peek2(Token![=])) {
            input.parse().map(NestedMeta::Lit)
        } else if peek_bare_keyword(input) {
            // syn 1 accepted keywords as meta paths which shank relies on for
            // flags like `mut` in `#[account(0, mut, name = "foo")]`, whereas
            // syn 2 only accepts them via `Ident::parse_any`.
            let ident = Ident::parse_any(input)?;
            Ok(NestedMeta::Meta(Meta::Path(Path::from(ident))))
        } else {
            input.parse().map(NestedMeta::Meta)
        }
    }
}

/// Peeks a keyword such as `mut` that is used as a bare flag, i.e. is the
/// whole argument.
fn peek_bare_keyword(input: ParseStream) -> bool {
    if input.peek(Ident) || !input.peek(Ident::peek_any) {
        return false;
    }
    let ahead = input.fork();
    Ident::parse_any(&ahead).is_ok()
        && (ahead.is_empty() || ahead.peek(Token![,]))
}

impl ToTokens for NestedMeta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            NestedMeta::Meta(meta) => meta.to_tokens(tokens),
            NestedMeta::Lit(lit) => lit.to_tokens(tokens),
        }
    }
}

/// Parses the arguments of a list-style meta item, i.e. the `a, "b", c = "d"`
/// inside `name(a, "b", c = "d")`.
pub fn parse_nested_metas_of_list(list: &MetaList) -> ParseResult<NestedMetas> {
    list.parse_args_with(Punctuated::parse_terminated)
}

/// Parses the arguments of a list-style attribute, i.e. the `a, "b", c = "d"`
/// inside `#[name(a, "b", c = "d")]`.
///
/// Fails with `err_msg` if the attribute is not list-style, e.g. `#[name]` or
/// `#[name = "value"]`.
pub fn parse_nested_metas(
    attr: &Attribute,
    err_msg: &str,
) -> ParseResult<NestedMetas> {
    match &attr.meta {
        Meta::List(list) => parse_nested_metas_of_list(list),
        Meta::Path(_) | Meta::NameValue(_) => {
            Err(ParseError::new_spanned(attr, err_msg))
        }
    }
}
