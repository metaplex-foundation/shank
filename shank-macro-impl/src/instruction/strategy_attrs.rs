use std::collections::HashSet;

use syn::Attribute;

const DEFAULT_OPTIONAL_ACCOUNTS: &str = "default_optional_accounts";

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InstructionStrategy {
    DefaultOptionalAccounts,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstructionStrategies(pub HashSet<InstructionStrategy>);

impl InstructionStrategy {
    pub fn from_account_attr(attr: &Attribute) -> Option<InstructionStrategy> {
        match attr
            .path
            .get_ident()
            .map(|x| x.to_string().as_str() == DEFAULT_OPTIONAL_ACCOUNTS)
        {
            Some(true) => Some(InstructionStrategy::DefaultOptionalAccounts),
            _ => None,
        }
    }
}

impl From<&[Attribute]> for InstructionStrategies {
    fn from(attrs: &[Attribute]) -> Self {
        let strategies = attrs
            .into_iter()
            .filter_map(InstructionStrategy::from_account_attr)
            .collect::<HashSet<InstructionStrategy>>();

        InstructionStrategies(strategies)
    }
}
