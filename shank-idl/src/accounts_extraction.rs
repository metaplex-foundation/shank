use anyhow::{format_err, Result};
use std::collections::HashMap;

use shank_macro_impl::{
    instruction::InstructionAccount,
    parsers::get_derive_attr,
    syn::{Field, Fields, ItemStruct, Path, Type},
    DERIVE_ACCOUNTS_ATTR,
};

/// Extract ShankAccounts structs and their metadata
pub fn extract_shank_accounts_structs<'a>(
    structs: impl Iterator<Item = &'a ItemStruct>,
) -> Result<HashMap<String, Vec<InstructionAccount>>> {
    let mut accounts_map = HashMap::new();

    for struct_item in structs {
        if let Some(_attr) =
            get_derive_attr(&struct_item.attrs, DERIVE_ACCOUNTS_ATTR)
        {
            let struct_name = struct_item.ident.to_string();
            let accounts = extract_accounts_from_struct(struct_item)?;
            accounts_map.insert(struct_name, accounts);
        }
    }

    Ok(accounts_map)
}

/// Extract individual accounts from a ShankAccounts struct by calling its __shank_accounts method
fn extract_accounts_from_struct(
    struct_item: &ItemStruct,
) -> Result<Vec<InstructionAccount>> {
    // This is where we need to get the account metadata.
    // The challenge is that at parse time, we can't execute the __shank_accounts() method.
    // We need to parse the struct fields and their #[account(...)] attributes directly.

    let struct_name = &struct_item.ident;

    // Parse the struct fields and extract account information
    let mut accounts = Vec::new();

    if let Fields::Named(fields) = &struct_item.fields {
        for (index, field) in fields.named.iter().enumerate() {
            let _field_name = field.ident.as_ref().ok_or_else(|| {
                format_err!("Field without name in struct {}", struct_name)
            })?;

            // Parse the #[account(...)] attributes on this field
            let account = parse_account_attributes(field, index)?;
            accounts.push(account);
        }
    } else {
        return Err(format_err!(
            "ShankAccounts struct {} must have named fields",
            struct_name
        ));
    }

    Ok(accounts)
}

/// Parse #[account(...)] attributes from a struct field
fn parse_account_attributes(
    field: &Field,
    index: usize,
) -> Result<InstructionAccount> {
    let field_name = field.ident.as_ref().unwrap().to_string();

    // Initialize default values
    let mut writable = false;
    let mut signer = false;
    let mut optional = false;
    let mut optional_signer = false;
    let mut desc: Option<String> = None;

    // Check if the field type is Option<&AccountInfo> to detect optional typing
    let has_option_type = if let Type::Path(type_path) = &field.ty {
        if let Some(segment) = type_path.path.segments.first() {
            segment.ident == "Option"
        } else {
            false
        }
    } else {
        false
    };

    // Parse #[account(...)] attributes
    for attr in &field.attrs {
        if attr.path.is_ident("account") {
            // Use a simple string-based parsing approach for now
            // This is a simplified version - in production we'd want more robust parsing
            let tokens_str = attr.tokens.to_string();

            // Simple parsing of common attributes
            if tokens_str.contains("mut") || tokens_str.contains("writable") {
                writable = true;
            }
            if tokens_str.contains("signer") {
                signer = true;
            }
            if tokens_str.contains("optional_signer") {
                optional_signer = true;
            } else if tokens_str.contains("optional") {
                optional = true;
            }

            // Extract description using simple regex-like approach
            if let Some(desc_start) = tokens_str.find("desc = \"") {
                let desc_content = &tokens_str[desc_start + 8..];
                if let Some(desc_end) = desc_content.find('"') {
                    desc = Some(desc_content[..desc_end].to_string());
                }
            }
        }
    }

    // Handle interaction between Option<> types and attribute flags:
    // - If has Option<> type and optional_signer attribute: only set optional_signer = true
    // - If has Option<> type and optional attribute: set optional = true
    // - If has Option<> type but no attribute: default to optional = true
    if has_option_type && !optional && !optional_signer {
        // If Option<> type but no explicit optional/optional_signer attribute,
        // assume it's a regular optional account
        optional = true;
    }

    // For optional_signer accounts, ensure the regular optional flag is not set
    // The IDL should use is_optional_signer=true, is_optional=false for these
    if optional_signer {
        optional = false;
    }

    Ok(InstructionAccount {
        index: Some(index as u32),
        ident: field.ident.as_ref().unwrap().clone(),
        name: field_name,
        writable,
        signer,
        optional_signer,
        optional,
        desc,
    })
}

/// Resolve accounts for a given struct path
pub fn resolve_accounts_for_struct_path<'a>(
    accounts_map: &'a HashMap<String, Vec<InstructionAccount>>,
    struct_path: &Path,
) -> Option<&'a Vec<InstructionAccount>> {
    let struct_name = struct_path
        .segments
        .last()
        .map(|seg| seg.ident.to_string())?;

    accounts_map.get(&struct_name)
}
