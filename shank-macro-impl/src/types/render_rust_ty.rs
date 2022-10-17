use proc_macro2::TokenStream;
use quote::quote;

use crate::types::{ParsedReference, TypeKind};

use super::{Primitive, RustType};

impl RustType {
    pub fn render(&self) -> TokenStream {
        let ty = match &self.kind {
            TypeKind::Primitive(kind) => kind.render(),
            TypeKind::Value(_) => todo!(),
            TypeKind::Composite(_, _) => todo!(),
            TypeKind::Unit => todo!("should not render unit rust type"),
            TypeKind::Unknown => {
                todo!("should not render unknown rust type")
            }
        };

        let reference = match &self.reference {
            ParsedReference::Owned => TokenStream::new(),
            ParsedReference::Ref(Some(pr)) => quote! { &#pr },
            ParsedReference::Ref(None) => quote! { & },
            ParsedReference::RefMut(None) => quote! { &mut },
            ParsedReference::RefMut(Some(pr)) => quote! { &#pr mut },
        };

        quote! { #reference #ty }
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

#[cfg(test)]
mod tests {
    use crate::types::Primitive;

    use super::*;

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
    }

    #[test]
    fn ref_primitive() {
        assert_tokens_match(
            RustType::ref_primitive("x", Primitive::USize).render(),
            quote! { &usize },
        );
        assert_tokens_match(
            RustType::ref_primitive("x", Primitive::I64).render(),
            quote! { &i64 },
        );
    }

    #[test]
    fn refmut_primitive() {
        assert_tokens_match(
            RustType::refmut_primitive("x", Primitive::Bool).render(),
            quote! { &mut bool },
        );
        assert_tokens_match(
            RustType::refmut_primitive("x", Primitive::U64).render(),
            quote! { &mut u64 },
        );
    }
}
