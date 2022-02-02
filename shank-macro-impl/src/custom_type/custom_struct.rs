use std::{convert::TryFrom, ops::Deref};
use syn::{
    parse::{Parse, ParseStream},
    Error as ParseError, ItemStruct, Result as ParseResult,
};

use crate::parsed_struct::ParsedStruct;

use super::DetectCustomTypeConfig;

// -----------------
// CustomStruct
// -----------------
pub struct CustomStruct(pub ParsedStruct);

impl TryFrom<&ItemStruct> for CustomStruct {
    type Error = ParseError;

    fn try_from(item: &ItemStruct) -> ParseResult<Self> {
        Ok(Self(ParsedStruct::try_from(item)?))
    }
}

impl Parse for CustomStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let strct = <ItemStruct as Parse>::parse(input)?;
        CustomStruct::try_from(&strct)
    }
}

impl Deref for CustomStruct {
    type Target = ParsedStruct;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CustomStruct {
    pub fn is_custom_struct(&self, config: &DetectCustomTypeConfig) -> bool {
        config.are_custom_type_attrs(&self.attrs)
    }
}

// -----------------
// Tests
// -----------------
#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::custom_type::parse_custom_struct;

    fn assert_is_custom(tokens: TokenStream) {
        assert!(
            parse_custom_struct(tokens).is_custom_struct(&Default::default())
        );
    }

    fn assert_is_not_custom(tokens: TokenStream) {
        assert!(
            !parse_custom_struct(tokens).is_custom_struct(&Default::default())
        );
    }

    #[test]
    fn is_custom_struct_missing_derive() {
        assert_is_not_custom(quote! {
            struct MyStruct {}
        });

        assert_is_not_custom(quote! {
            #[BorshSerialize]
            struct MyStruct {}
        });
    }

    #[test]
    fn is_custom_struct_including_derive() {
        assert_is_custom(quote! {
            #[derive(BorshSerialize)]
            struct MyStruct {}
        });
    }

    #[test]
    fn is_custom_struct_including_borsh_derive_and_shank_derive() {
        assert_is_not_custom(quote! {
            #[derive(BorshSerialize, ShankInstruction)]
            struct MyStruct {}
        });
        assert_is_not_custom(quote! {
            #[derive(BorshSerialize)]
            #[derive(ShankInstruction)]
            struct MyStruct {}
        });
    }
}
