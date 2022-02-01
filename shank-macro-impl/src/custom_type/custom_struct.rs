use std::{
    collections::HashSet, convert::TryFrom, iter::FromIterator, ops::Deref,
};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Error as ParseError, ItemStruct, Result as ParseResult,
};

use crate::{
    account::DERIVE_ACCOUNT_ATTR, instruction::DERIVE_INSTRUCTION_ATTR,
    parsed_struct::ParsedStruct, parsers::get_derive_names,
};

// -----------------
// DetectCustomStructConfig
// -----------------
#[derive(Debug)]
pub struct DetectCustomStructConfig {
    /// If any of those derives is detected that struct is considered a Custom Struct
    pub include_derives: HashSet<String>,

    /// If any of those derives is detected that struct is NOT considered a Custom Struct
    pub skip_derives: HashSet<String>,
}

impl Default for DetectCustomStructConfig {
    fn default() -> Self {
        Self {
            include_derives: HashSet::from_iter(
                vec!["BorshSerialize", "BorshDeserialize"]
                    .into_iter()
                    .map(String::from),
            ),
            skip_derives: HashSet::from_iter(
                vec![DERIVE_ACCOUNT_ATTR, DERIVE_INSTRUCTION_ATTR]
                    .into_iter()
                    .map(String::from),
            ),
        }
    }
}

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
    pub fn is_custom_struct(&self, config: &DetectCustomStructConfig) -> bool {
        CustomStruct::are_custom_struct_attrs(&self.attrs, config)
    }

    pub fn are_custom_struct_attrs(
        attrs: &[Attribute],
        config: &DetectCustomStructConfig,
    ) -> bool {
        let derives = get_derive_names(attrs);
        let mut saw_include = false;
        for derive in derives {
            if config.skip_derives.contains(&derive) {
                return false;
            }
            if !saw_include {
                saw_include = config.include_derives.contains(&derive);
            }
        }
        return saw_include;
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
