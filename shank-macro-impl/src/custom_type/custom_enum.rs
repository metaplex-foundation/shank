use std::{convert::TryFrom, ops::Deref};

use syn::{
    parse::{Parse, ParseStream},
    Error as ParseError, ItemEnum, Result as ParseResult,
};

use crate::parsed_enum::ParsedEnum;

use super::DetectCustomTypeConfig;

pub struct CustomEnum(pub ParsedEnum);

impl TryFrom<&ItemEnum> for CustomEnum {
    type Error = ParseError;

    fn try_from(item: &ItemEnum) -> ParseResult<Self> {
        Ok(Self(ParsedEnum::try_from(item)?))
    }
}

impl Parse for CustomEnum {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let strct = <ItemEnum as Parse>::parse(input)?;
        CustomEnum::try_from(&strct)
    }
}

impl Deref for CustomEnum {
    type Target = ParsedEnum;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CustomEnum {
    pub fn is_custom_enum(&self, config: &DetectCustomTypeConfig) -> bool {
        config.are_custom_type_attrs(&self.attrs)
    }
}
