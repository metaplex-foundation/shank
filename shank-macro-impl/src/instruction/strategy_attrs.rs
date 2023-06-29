use std::collections::HashSet;

use syn::Attribute;

const LEGACY_OPTIONAL_ACCOUNTS_STRATEGY: &str =
    "legacy_optional_accounts_strategy";

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InstructionStrategy {
    LegacyOptionalAccounts,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstructionStrategies(pub HashSet<InstructionStrategy>);

impl InstructionStrategy {
    pub fn from_account_attr(attr: &Attribute) -> Option<InstructionStrategy> {
        match attr.path.get_ident().map(|x| {
            x.to_string().as_str() == LEGACY_OPTIONAL_ACCOUNTS_STRATEGY
        }) {
            Some(true) => Some(InstructionStrategy::LegacyOptionalAccounts),
            _ => None,
        }
    }
}

impl From<&[Attribute]> for InstructionStrategies {
    fn from(attrs: &[Attribute]) -> Self {
        let strategies = attrs
            .iter()
            .filter_map(InstructionStrategy::from_account_attr)
            .collect::<HashSet<InstructionStrategy>>();

        InstructionStrategies(strategies)
    }
}
