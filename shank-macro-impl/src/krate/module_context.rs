/// Adapted from: https://github.com/project-serum/anchor/blob/d8d720067dd6e2a3bec50207b84008276c914732/lang/syn/src/parser/context.rs
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use syn::{Error as ParseError, Result as ParseResult};

/// Module parse context
///
/// Keeps track of items defined within a module.
#[derive(Copy, Clone)]
pub struct ModuleContext<'krate> {
    pub detail: &'krate ParsedModule,
}

impl<'krate> ModuleContext<'krate> {
    pub fn items(&self) -> impl Iterator<Item = &syn::Item> {
        self.detail.items.iter()
    }
}

#[derive(Debug)]
pub struct ParsedModule {
    pub name: String,
    pub file: PathBuf,
    pub path: String,
    pub items: Vec<syn::Item>,
}

impl ParsedModule {
    pub fn parse_recursive(
        root: &Path,
    ) -> Result<BTreeMap<String, ParsedModule>, anyhow::Error> {
        let root_content = std::fs::read_to_string(root)?;
        Self::parse_content_recursive(root, root_content)
    }

    fn parse_content_recursive(
        root: &Path,
        root_content: String,
    ) -> Result<BTreeMap<String, ParsedModule>, anyhow::Error> {
        let mut modules = BTreeMap::new();

        let root_file = syn::parse_file(&root_content)?;
        let root_mod = Self::new(
            String::new(),
            root.to_owned(),
            "crate".to_owned(),
            root_file.items,
        );

        struct UnparsedModule {
            file: PathBuf,
            path: String,
            name: String,
            item: syn::ItemMod,
        }

        let mut unparsed = root_mod
            .submodules()
            .map(|item| UnparsedModule {
                file: root_mod.file.clone(),
                path: root_mod.path.clone(),
                name: item.ident.to_string(),
                item: item.clone(),
            })
            .collect::<Vec<_>>();

        while let Some(to_parse) = unparsed.pop() {
            let path = format!("{}::{}", to_parse.path, to_parse.name);
            let module =
                Self::from_item_mod(&to_parse.file, &path, to_parse.item)?;

            unparsed.extend(module.submodules().map(|item| UnparsedModule {
                item: item.clone(),
                file: module.file.clone(),
                path: module.path.clone(),
                name: item.ident.to_string(),
            }));
            modules.insert(path, module);
        }

        modules.insert(root_mod.name.clone(), root_mod);

        Ok(modules)
    }

    fn from_item_mod(
        parent_file: &Path,
        parent_path: &str,
        item: syn::ItemMod,
    ) -> ParseResult<Self> {
        Ok(match item.content {
            Some((_, items)) => {
                // The module content is within the parent file being parsed
                Self::new(
                    parent_path.to_owned(),
                    parent_file.to_owned(),
                    item.ident.to_string(),
                    items,
                )
            }
            None => {
                // The module is referencing some other file, so we need to load that
                // to parse the items it has.
                let parent_dir = parent_file.parent().unwrap();
                let parent_filename =
                    parent_file.file_stem().unwrap().to_str().unwrap();
                let parent_mod_dir = parent_dir.join(parent_filename);

                let possible_file_paths = vec![
                    parent_dir.join(format!("{}.rs", item.ident)),
                    parent_dir.join(format!("{}/mod.rs", item.ident)),
                    parent_mod_dir.join(format!("{}.rs", item.ident)),
                    parent_mod_dir.join(format!("{}/mod.rs", item.ident)),
                ];

                let mod_file_path = possible_file_paths
                    .into_iter()
                    .find(|p| p.exists())
                    .ok_or_else(|| {
                        ParseError::new_spanned(&item, "could not find file")
                    })?;
                let mod_file_content = std::fs::read_to_string(&mod_file_path)
                    .map_err(|_| {
                        ParseError::new_spanned(&item, "could not read file")
                    })?;
                let mod_file = syn::parse_file(&mod_file_content)?;

                Self::new(
                    parent_path.to_owned(),
                    mod_file_path,
                    item.ident.to_string(),
                    mod_file.items,
                )
            }
        })
    }

    fn new(
        path: String,
        file: PathBuf,
        name: String,
        items: Vec<syn::Item>,
    ) -> Self {
        Self {
            name,
            file,
            path,
            items,
        }
    }

    fn submodules(&self) -> impl Iterator<Item = &syn::ItemMod> {
        self.items.iter().filter_map(|i| match i {
            syn::Item::Mod(item) => Some(item),
            _ => None,
        })
    }

    pub fn structs(&self) -> impl Iterator<Item = &syn::ItemStruct> {
        self.items.iter().filter_map(|i| match i {
            syn::Item::Struct(item) => Some(item),
            _ => None,
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = &syn::ItemEnum> {
        self.items.iter().filter_map(|i| match i {
            syn::Item::Enum(item) => Some(item),
            _ => None,
        })
    }

    pub fn macros(&self) -> impl Iterator<Item = &syn::ItemMacro> {
        self.items.iter().filter_map(|i| match i {
            syn::Item::Macro(item) => Some(item),
            _ => None,
        })
    }

    pub fn consts(&self) -> impl Iterator<Item = &syn::ItemConst> {
        self.items.iter().filter_map(|i| match i {
            syn::Item::Const(item) => Some(item),
            _ => None,
        })
    }

    pub fn all_items(&self) -> impl Iterator<Item = &syn::Item> {
        self.items.iter()
    }
}

// -----------------
// Tests
// -----------------
#[cfg(test)]
mod tests {

    use assert_matches::assert_matches;
    use proc_macro2::TokenStream;
    use quote::quote;

    use super::*;

    #[derive(Debug)]
    struct SingleModule {
        submodules: Vec<syn::ItemMod>,
        structs: Vec<syn::ItemStruct>,
        enums: Vec<syn::ItemEnum>,
        consts: Vec<syn::ItemConst>,
    }

    impl SingleModule {
        fn from_code(code: TokenStream) -> Self {
            let s = code.to_string();
            let p = Path::new("/single_module.rs");
            let parsed = ParsedModule::parse_content_recursive(p, s)
                .expect("Failed to parse");
            let m = parsed.get("crate").expect("Could not find root module");

            Self {
                submodules: m.submodules().cloned().collect(),
                structs: m.structs().cloned().collect(),
                enums: m.enums().cloned().collect(),
                consts: m.consts().cloned().collect(),
            }
        }
    }

    #[test]
    fn parse_single_module_one_struct() {
        let parsed = SingleModule::from_code(quote! { struct MyStruct {} });
        assert_matches!(&parsed, SingleModule { submodules, structs, enums, consts } => {
            assert_eq!(submodules.len(), 0, "submodules");
            assert_eq!(structs.len(), 1, "structs");
            assert_eq!(enums.len(), 0, "enums");
            assert_eq!(consts.len(), 0, "consts");

            assert_eq!(structs.first().expect("at least one struct").ident, "MyStruct");
        });
    }

    #[test]
    fn parse_single_module_one_enum() {
        let parsed = SingleModule::from_code(quote! { enum Direction { Up }  });
        assert_matches!(&parsed, SingleModule { submodules, structs, enums, consts } => {
            assert_eq!(submodules.len(), 0, "submodules");
            assert_eq!(structs.len(), 0, "structs");
            assert_eq!(enums.len(), 1, "enums");
            assert_eq!(consts.len(), 0, "consts");

            let en = enums.first().expect("at least one enum");
            assert_eq!(en.ident, "Direction");
            assert_eq!(en.variants.len(), 1, "enum variants");
        });
    }

    #[test]
    fn parse_single_module_one_const() {
        let parsed = SingleModule::from_code(quote! { const ONE: u8 = 1;  });
        assert_matches!(&parsed, SingleModule { submodules, structs, enums, consts } => {
            assert_eq!(submodules.len(), 0, "submodules");
            assert_eq!(structs.len(), 0, "structs");
            assert_eq!(enums.len(), 0, "enums");
            assert_eq!(consts.len(), 1, "consts");

            let co = consts.first().expect("at least one const");
            assert_eq!(co.ident, "ONE");
        });
    }

    #[test]
    fn parse_single_module_one_empty_submod() {
        let parsed = SingleModule::from_code(quote! { mod submod {} });
        assert_matches!(&parsed, SingleModule { submodules, structs, enums, consts } => {
            assert_eq!(submodules.len(), 1, "submodules");
            assert_eq!(structs.len(), 0, "structs");
            assert_eq!(enums.len(), 0, "enums");
            assert_eq!(consts.len(), 0, "consts");

            let mo = submodules.first().expect("at least one const");
            assert_eq!(mo.content.as_ref().unwrap().1.len(), 0, "submod content");
        });
    }

    #[test]
    fn parse_single_module_one_submod_with_struct() {
        let parsed = SingleModule::from_code(quote! {
            mod submod {
                struct InnerStruct {}
            }
        });
        assert_matches!(&parsed, SingleModule { submodules, structs, enums, consts } => {
            assert_eq!(submodules.len(), 1, "submodules");
            assert_eq!(structs.len(), 0, "structs");
            assert_eq!(enums.len(), 0, "enums");
            assert_eq!(consts.len(), 0, "consts");

            let mo = submodules.first().expect("at least one const");
            assert_eq!(mo.content.as_ref().unwrap().1.len(), 1, "submod content");
        });
    }

    #[test]
    fn parse_single_module_two_structs_one_enum_one_const() {
        let parsed = SingleModule::from_code(quote! {
            struct MyStruct {}
            enum Direction { Up }
            const HELLO: &str = "Hola";
        });
        assert_matches!(&parsed, SingleModule { submodules, structs, enums, consts } => {
            assert_eq!(submodules.len(), 0, "submodules");
            assert_eq!(structs.len(), 1, "structs");
            assert_eq!(enums.len(), 1, "enums");
            assert_eq!(consts.len(), 1, "consts");

            assert_eq!(structs.first().expect("at least one struct").ident, "MyStruct");
            assert_eq!(enums.first().expect("at least one enum").ident, "Direction");
            assert_eq!(consts.first().expect("at least one const").ident, "HELLO");
        });
    }
}
