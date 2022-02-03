use proc_macro2::{Ident, Literal};
use syn::ItemMacro;

/// A very simplified parsed macro which only supports the use case we need for now
#[derive(Debug)]
pub struct ParsedMacro {
    pub path_idents: Vec<Ident>,
    pub path: String,
    pub literal: Option<String>,
}

impl From<&ItemMacro> for ParsedMacro {
    fn from(item_macro: &ItemMacro) -> Self {
        let path_idents: Vec<Ident> = item_macro
            .mac
            .path
            .segments
            .iter()
            .map(|x| x.ident.clone())
            .collect();

        let path = path_idents
            .iter()
            .map(|ident| ident.to_string())
            .reduce(|acc, ident| format!("{}::{}", acc, ident))
            .unwrap_or("".to_string());

        let literal = syn::parse2::<Literal>(item_macro.mac.tokens.clone())
            .map_or(None, |lit| {
                Some(lit.to_string().trim_matches('"').to_string())
            });

        Self {
            path_idents,
            path,
            literal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::ItemMacro;

    fn parse_macro(code: TokenStream) -> ParsedMacro {
        let item_macro = syn::parse2::<ItemMacro>(code)
            .expect("Should parse ItemMacro successfully");
        (&item_macro).into()
    }

    #[test]
    fn macro_program_id_qualified_solana_program() {
        let parsed = parse_macro(quote! {
            solana_program::declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
        });

        assert_eq!(parsed.path, "solana_program::declare_id", "path");
        assert_eq!(parsed.path_idents.len(), 2, "path idents");
        assert_eq!(
            parsed.literal,
            Some("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s".to_string())
        )
    }

    #[test]
    fn macro_program_id_imported_solana_program() {
        let parsed = parse_macro(quote! {
            declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
        });

        assert_eq!(parsed.path, "declare_id", "path");
        assert_eq!(parsed.path_idents.len(), 1, "path idents");
        assert_eq!(
            parsed.literal,
            Some("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s".to_string())
        )
    }
}
