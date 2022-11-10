use proc_macro2::TokenStream;
use quote::quote;

use crate::types::{ParsedReference, TypeKind};

use super::{Primitive, RustType, Value};

impl RustType {
    pub fn render(&self) -> TokenStream {
        let ty = match &self.kind {
            TypeKind::Primitive(prim) => prim.render(),
            TypeKind::Value(val) => val.render(),
            TypeKind::Composite(_, _) => todo!(),
            TypeKind::Unit => todo!("should not render unit rust type"),
            TypeKind::Unknown => {
                todo!("should not render unknown rust type")
            }
        };

        let reference = match &self.reference {
            ParsedReference::Owned => TokenStream::new(),
            ParsedReference::Ref(Some(lifetime)) => {
                format!("&'{}", lifetime).parse().unwrap()
            }
            ParsedReference::Ref(None) => quote! { & },
            ParsedReference::RefMut(None) => quote! { &mut },
            ParsedReference::RefMut(Some(lifetime)) => {
                format!("&'{} mut", lifetime).parse().unwrap()
            }
        };

        quote! { #reference #ty }
    }

    pub fn render_param(&self) -> TokenStream {
        let full_ty = match &self.kind {
            TypeKind::Primitive(_) => self.render(),
            TypeKind::Value(_) => self.render(),
            TypeKind::Composite(_, _) => todo!(),
            TypeKind::Unit => todo!("should not render unit rust type"),
            TypeKind::Unknown => {
                todo!("should not render unknown rust type")
            }
        };

        let ident = &self.ident;
        quote! { #ident: #full_ty }
    }
}

impl Primitive {
    fn render(&self) -> TokenStream {
        match self {
            Self::U8 => quote! { u8 },
            Self::I8 => quote! { i8 },
            Self::U16 => quote! { u16 },
            Self::I16 => quote! { i16 },
            Self::U32 => quote! { u32 },
            Self::I32 => quote! { i32 },
            Self::U64 => quote! { u64 },
            Self::I64 => quote! { i64 },
            Self::U128 => quote! { u128 },
            Self::I128 => quote! { i128 },
            Self::USize => quote! { usize },
            Self::Bool => quote! { bool },
        }
    }
}

impl Value {
    fn render(&self) -> TokenStream {
        match self {
            Value::CString => quote! { ::std::ffi::CString },
            Value::String => quote! { String },
            Value::Str => quote! { str },
            Value::Custom(val) => val
                .parse()
                .expect(&format!("Failed to render Value::Custom({})", val)),
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::{Ident, Span};

    use crate::types::Primitive;

    use super::*;

    fn ident(s: &str) -> Option<Ident> {
        Some(Ident::new(s, Span::call_site()))
    }

    fn assert_tokens_match(a: TokenStream, b: TokenStream) {
        assert_eq!(a.to_string(), b.to_string(), "generated tokens match")
    }

    // -----------------
    // Primitives
    // -----------------
    #[test]
    fn owned_primitive() {
        assert_tokens_match(
            RustType::owned_primitive("x", Primitive::U8).render(),
            quote! { u8 },
        );
        assert_tokens_match(
            RustType::owned_primitive("x", Primitive::I128).render(),
            quote! { i128 },
        );
        // param
        assert_tokens_match(
            RustType::owned_primitive("x", Primitive::U8).render_param(),
            quote! { x: u8 },
        );
    }

    #[test]
    fn ref_primitive() {
        assert_tokens_match(
            RustType::ref_primitive("x", Primitive::USize, None).render(),
            quote! { &usize },
        );
        assert_tokens_match(
            RustType::ref_primitive("x", Primitive::I64, None).render(),
            quote! { &i64 },
        );

        // param
        assert_tokens_match(
            RustType::ref_primitive("x", Primitive::USize, None).render_param(),
            quote! { x: &usize },
        );
    }

    #[test]
    fn refmut_primitive() {
        assert_tokens_match(
            RustType::refmut_primitive("x", Primitive::Bool, None).render(),
            quote! { &mut bool },
        );
        assert_tokens_match(
            RustType::refmut_primitive("x", Primitive::U64, None).render(),
            quote! { &mut u64 },
        );

        // param
        assert_tokens_match(
            RustType::refmut_primitive("x", Primitive::Bool, None)
                .render_param(),
            quote! { x: &mut bool },
        );
    }

    #[test]
    fn ref_primitive_with_lifetime() {
        assert_tokens_match(
            RustType::ref_primitive("x", Primitive::USize, ident("a")).render(),
            "&'a usize".parse().unwrap(),
        );
        assert_tokens_match(
            RustType::ref_primitive("x", Primitive::I64, ident("lifetime"))
                .render(),
            "&'lifetime i64".parse().unwrap(),
        );

        // param
        assert_tokens_match(
            RustType::ref_primitive("x", Primitive::USize, ident("b"))
                .render_param(),
            "x: &'b usize".parse().unwrap(),
        );
    }

    // -----------------
    // Values (Strings)
    // -----------------
    #[test]
    fn owned_string() {
        assert_tokens_match(
            RustType::owned_string("my_string").render(),
            quote! { String },
        );
        // param
        assert_tokens_match(
            RustType::owned_string("my_string").render_param(),
            quote! { my_string: String },
        );
    }

    #[test]
    fn ref_str() {
        assert_tokens_match(
            RustType::ref_str("my_str", None).render(),
            quote! { &str },
        );
        // param
        assert_tokens_match(
            RustType::ref_str("my_str", None).render_param(),
            quote! { my_str: &str },
        );
    }

    #[test]
    fn ref_str_with_lifetime() {
        assert_tokens_match(
            RustType::ref_str("my_str", ident("lt")).render(),
            "&'lt str".parse().unwrap(),
        );
        // param
        assert_tokens_match(
            RustType::ref_str("my_str", ident("lt")).render_param(),
            "my_str: &'lt str".parse().unwrap(),
        );
    }

    #[test]
    fn ref_string_mut_with_lifetime() {
        assert_tokens_match(
            RustType::ref_string_mut("my_str", ident("lt")).render(),
            "&'lt mut String".parse().unwrap(),
        );
        // param
        assert_tokens_match(
            RustType::ref_string_mut("my_str", ident("lt")).render_param(),
            "my_str: &'lt mut String".parse().unwrap(),
        );
    }

    // -----------------
    // Values (Custom)
    // -----------------
    #[test]
    fn owned_account_info() {
        assert_tokens_match(
            RustType::owned_custom_value(
                "my_info",
                "::solana_program::account_info::AccountInfo<'info>",
            )
            .render(),
            "::solana_program::account_info::AccountInfo<'info>"
                .parse()
                .unwrap(),
        );
        // param
        assert_tokens_match(
            RustType::owned_custom_value(
                "my_info",
                "::solana_program::account_info::AccountInfo<'info>",
            )
            .render_param(),
            "my_info: ::solana_program::account_info::AccountInfo<'info>"
                .parse()
                .unwrap(),
        );
    }

    #[test]
    fn ref_account_info() {
        assert_tokens_match(
            RustType::ref_custom_value(
                "my_info",
                "::solana_program::account_info::AccountInfo<'info>",
                None,
            )
            .render(),
            "& ::solana_program::account_info::AccountInfo<'info>"
                .parse()
                .unwrap(),
        );
        // param
        assert_tokens_match(
            RustType::ref_custom_value(
                "my_info",
                "::solana_program::account_info::AccountInfo<'info>",
                None,
            )
            .render_param(),
            "my_info: & ::solana_program::account_info::AccountInfo<'info>"
                .parse()
                .unwrap(),
        );
    }

    #[test]
    fn ref_account_info_with_lifetime() {
        assert_tokens_match(
            RustType::ref_custom_value(
                "my_info",
                "::solana_program::account_info::AccountInfo<'info>",
                ident("b"),
            )
            .render(),
            "&'b ::solana_program::account_info::AccountInfo<'info>"
                .parse()
                .unwrap(),
        );
        // param
        assert_tokens_match(
            RustType::ref_custom_value(
                "my_info",
                "::solana_program::account_info::AccountInfo<'info>",
                ident("b"),
            )
            .render_param(),
            "my_info: &'b ::solana_program::account_info::AccountInfo<'info>"
                .parse()
                .unwrap(),
        );
    }

    #[test]
    fn ref_account_info_mut_with_lifetime() {
        assert_tokens_match(
            RustType::ref_mut_custom_value(
                "my_info",
                "::solana_program::account_info::AccountInfo<'info>",
                ident("b"),
            )
            .render(),
            "&'b mut ::solana_program::account_info::AccountInfo<'info>"
                .parse()
                .unwrap(),
        );
        // param
        assert_tokens_match(
            RustType::ref_mut_custom_value(
                "my_info",
                "::solana_program::account_info::AccountInfo<'info>",
                ident("b"),
            )
            .render_param(),
            "my_info: &'b mut ::solana_program::account_info::AccountInfo<'info>"
                .parse()
                .unwrap(),
        );
    }
}
