use std::{collections::HashSet, iter::FromIterator};

use syn::Attribute;

use crate::{
    parsers::get_derive_names, DERIVE_ACCOUNT_ATTR, DERIVE_INSTRUCTION_ATTR,
};

#[derive(Debug)]
pub struct DetectCustomTypeConfig {
    /// If any of those derives is detected that struct is considered a Custom Struct
    pub include_derives: HashSet<String>,

    /// If any of those derives is detected that struct is NOT considered a Custom Struct
    pub skip_derives: HashSet<String>,
}

impl Default for DetectCustomTypeConfig {
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

impl DetectCustomTypeConfig {
    pub fn are_custom_type_attrs(&self, attrs: &[Attribute]) -> bool {
        let derives = get_derive_names(attrs);
        let mut saw_include = false;
        for derive in derives {
            if self.skip_derives.contains(&derive) {
                return false;
            }
            if !saw_include {
                saw_include = self.include_derives.contains(&derive);
            }
        }
        return saw_include;
    }
}
