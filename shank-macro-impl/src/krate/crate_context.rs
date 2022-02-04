/// Adapted from: https://github.com/project-serum/anchor/blob/d8d720067dd6e2a3bec50207b84008276c914732/lang/syn/src/parser/context.rs
use std::{collections::BTreeMap, path::Path};

use super::module_context::{ModuleContext, ParsedModule};

/// Crate parse context
///
/// Keeps track of modules defined within a crate.
#[derive(Debug)]
pub struct CrateContext {
    modules: BTreeMap<String, ParsedModule>,
}

impl CrateContext {
    pub fn consts(&self) -> impl Iterator<Item = &syn::ItemConst> {
        self.modules.iter().flat_map(|(_, ctx)| ctx.consts())
    }

    pub fn structs(&self) -> impl Iterator<Item = &syn::ItemStruct> {
        self.modules.iter().flat_map(|(_, ctx)| ctx.structs())
    }

    pub fn enums(&self) -> impl Iterator<Item = &syn::ItemEnum> {
        self.modules.iter().flat_map(|(_, ctx)| ctx.enums())
    }

    pub fn macros(&self) -> impl Iterator<Item = &syn::ItemMacro> {
        self.modules.iter().flat_map(|(_, ctx)| ctx.macros())
    }

    pub fn modules(&self) -> impl Iterator<Item = ModuleContext> {
        self.modules
            .iter()
            .map(move |(_, detail)| ModuleContext { detail })
    }

    pub fn root_module(&self) -> ModuleContext {
        ModuleContext {
            detail: self.modules.get("crate").unwrap(),
        }
    }

    pub fn all_items(&self) -> impl Iterator<Item = &syn::Item> {
        self.modules.iter().flat_map(|(_, ctx)| ctx.all_items())
    }

    pub fn all_items_vec(&self) -> Vec<syn::Item> {
        self.modules
            .iter()
            .flat_map(|(_, ctx)| ctx.all_items())
            .cloned()
            .collect()
    }

    pub fn parse(root: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        Ok(CrateContext {
            modules: ParsedModule::parse_recursive(root.as_ref())?,
        })
    }
}
