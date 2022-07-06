use std::convert::TryFrom;

use syn::{Attribute, Error as ParseError, ItemEnum, Result as ParseResult};

use super::ParsedEnumVariant;

#[derive(Debug, PartialEq)]
pub struct ParsedEnum {
    /// The enum itself, i.e. CreateOwner
    pub ident: syn::Ident,

    /// The enum variants, i.e. All, Completed
    pub variants: Vec<ParsedEnumVariant>,

    /// Attributes found on the enum
    pub attrs: Vec<Attribute>,
}

impl TryFrom<&ItemEnum> for ParsedEnum {
    type Error = ParseError;

    fn try_from(item_enum: &ItemEnum) -> ParseResult<Self> {
        let ItemEnum {
            ident,
            variants,
            attrs,
            ..
        } = item_enum;

        let mut implicit_discriminant = 0;
        let variants = variants
            .into_iter()
            .enumerate()
            .map(|(slot, x)| {
                let parsed = ParsedEnumVariant::try_from((
                    slot,
                    implicit_discriminant,
                    x,
                ));
                implicit_discriminant = parsed
                    .as_ref()
                    .map(|x| x.discriminant + 1)
                    .unwrap_or(implicit_discriminant + 1);
                parsed
            })
            .collect::<ParseResult<Vec<ParsedEnumVariant>>>()?;

        Ok(ParsedEnum {
            ident: ident.clone(),
            variants,
            attrs: attrs.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::ItemEnum;

    use super::*;

    fn parse_enum(code: TokenStream) -> ParseResult<ParsedEnum> {
        let item_enum = syn::parse2::<ItemEnum>(code)
            .expect("Should parse ItemEnum successfully");
        (&item_enum).try_into()
    }

    fn assert_enum_variant(
        variant: &ParsedEnumVariant,
        name: &str,
        expected_slot: usize,
        expected_discriminant: usize,
        fields_len: usize,
        attrs_len: usize,
    ) {
        let ParsedEnumVariant {
            ident,
            fields,
            slot,
            discriminant,
            attrs,
        } = variant;

        assert_eq!(ident.to_string(), name);
        assert_eq!(slot, &expected_slot, "slot");
        assert_eq!(discriminant, &expected_discriminant, "discriminant");
        assert_eq!(fields.len(), fields_len, "fields");
        assert_eq!(attrs.len(), attrs_len, "attrs");
    }

    fn assert_enum_variant_field_names(
        variant: &ParsedEnumVariant,
        names: &[&str],
    ) {
        assert_eq!(
            variant.fields.len(),
            names.len(),
            "amount of fields does not match amount of names"
        );
        for (idx, f) in variant.fields.iter().enumerate() {
            assert_eq!(
                f.ident.as_ref().unwrap().to_string().as_str(),
                names[idx]
            );
        }
    }

    #[test]
    fn parse_c_style_enum_without_attrs() {
        let parsed = parse_enum(quote! {
            pub enum Color {
                Red, Green, Blue
            }
        })
        .expect("Should parse fine");

        assert_eq!(parsed.ident.to_string(), "Color", "enum ident");
        assert_eq!(parsed.variants.len(), 3, "variants");
        assert_eq!(parsed.attrs.len(), 0, "enum attrs");

        assert_enum_variant(&parsed.variants[0], "Red", 0, 0, 0, 0);
        assert_enum_variant(&parsed.variants[1], "Green", 1, 1, 0, 0);
        assert_enum_variant(&parsed.variants[2], "Blue", 2, 2, 0, 0);

        let parsed = parse_enum(quote! {
            pub enum Color {
                Red = 0xf00, Green = 0x0f0, Blue = 0x00f
            }
        })
        .expect("Should parse fine");

        assert_enum_variant(&parsed.variants[0], "Red", 0, 0xf00, 0, 0);
        assert_enum_variant(&parsed.variants[1], "Green", 1, 0x0f0, 0, 0);
        assert_enum_variant(&parsed.variants[2], "Blue", 2, 0x00f, 0, 0);
    }

    #[test]
    fn parse_c_style_enum_with_attrs() {
        let parsed = parse_enum(quote! {
            #[derive(ShankInstruction)]
            pub enum Instruction {
                #[account(0, name = "creator", sig)]
                #[account(1, name = "thing", mut)]
                CreateThing,
                #[account(name = "creator", sig)]
                CloseThing
            }
        })
        .expect("Should parse fine");

        assert_eq!(parsed.ident.to_string(), "Instruction", "enum ident");
        assert_eq!(parsed.variants.len(), 2, "variants");
        assert_eq!(parsed.attrs.len(), 1, "enum attrs");

        assert_enum_variant(&parsed.variants[0], "CreateThing", 0, 0, 0, 2);
        assert_enum_variant(&parsed.variants[1], "CloseThing", 1, 1, 0, 1);
    }

    #[test]
    fn parse_enum_without_attrs() {
        let parsed = parse_enum(quote! {
            pub enum Instruction {
                CreateThing(CreateArgs),
                CloseThing
            }
        })
        .expect("Should parse fine");

        assert_eq!(parsed.ident.to_string(), "Instruction", "enum ident");
        assert_eq!(parsed.variants.len(), 2, "variants");
        assert_eq!(parsed.attrs.len(), 0, "enum attrs");

        assert_enum_variant(&parsed.variants[0], "CreateThing", 0, 0, 1, 0);
        assert_enum_variant(&parsed.variants[1], "CloseThing", 1, 1, 0, 0);
    }

    #[test]
    fn parse_enum_with_attrs() {
        let parsed = parse_enum(quote! {
            #[derive(ShankInstruction)]
            pub enum Instruction {
                #[account(0, name = "creator", sig)]
                #[account(1, name = "thing", mut)]
                CreateThing = 100,
                #[account(name = "creator", sig)]
                CloseThing(CloseArgs) = 101
            }
        })
        .expect("Should parse fine");

        assert_eq!(parsed.ident.to_string(), "Instruction", "enum ident");
        assert_eq!(parsed.variants.len(), 2, "variants");
        assert_eq!(parsed.attrs.len(), 1, "enum attrs");

        assert_enum_variant(&parsed.variants[0], "CreateThing", 0, 100, 0, 2);
        assert_enum_variant(&parsed.variants[1], "CloseThing", 1, 101, 1, 1);
    }

    #[test]
    fn parse_data_enum_with_named_field() {
        let parsed = parse_enum(quote! {
            pub enum CollectionDetails {
                V1 { size: u64 },
            }
        })
        .expect("Should parse fine");

        assert_eq!(parsed.ident.to_string(), "CollectionDetails", "enum ident");
        assert_eq!(parsed.variants.len(), 1, "variants");
        assert_eq!(parsed.attrs.len(), 0, "enum attrs");

        assert_enum_variant(&parsed.variants[0], "V1", 0, 0, 1, 0);
        assert_enum_variant_field_names(&parsed.variants[0], &["size"]);
    }

    #[test]
    fn parse_data_enum_with_multiple_named_fields() {
        let parsed = parse_enum(quote! {
            pub enum CollectionInfo {
                V1 {
                    symbol: String,
                    verified_creators: Vec<Pubkey>,
                    whitelist_root: [u8; 32],
                },
                V2 {
                    collection_mint: Pubkey,
                },
            }
        })
        .expect("Should parse fine");

        assert_eq!(parsed.ident.to_string(), "CollectionInfo", "enum ident");
        assert_eq!(parsed.variants.len(), 2, "variants");
        assert_eq!(parsed.attrs.len(), 0, "enum attrs");

        assert_enum_variant(&parsed.variants[0], "V1", 0, 0, 3, 0);
        assert_enum_variant(&parsed.variants[1], "V2", 1, 1, 1, 0);

        assert_enum_variant_field_names(
            &parsed.variants[0],
            &["symbol", "verified_creators", "whitelist_root"],
        );
        assert_enum_variant_field_names(
            &parsed.variants[1],
            &["collection_mint"],
        );
    }

    #[test]
    fn parse_data_enum_with_named_and_unnamed_fields() {
        let parsed = parse_enum(quote! {
            pub enum CollectionInfo {
                V1 {
                    symbol: String,
                    verified_creators: Vec<Pubkey>,
                    whitelist_root: [u8; 32],
                },
                V2(Pubkey),
            }
        })
        .expect("Should parse fine");

        assert_eq!(parsed.ident.to_string(), "CollectionInfo", "enum ident");
        assert_eq!(parsed.variants.len(), 2, "variants");
        assert_eq!(parsed.attrs.len(), 0, "enum attrs");

        assert_enum_variant(&parsed.variants[0], "V1", 0, 0, 3, 0);
        assert_enum_variant(&parsed.variants[1], "V2", 1, 1, 1, 0);

        assert_enum_variant_field_names(
            &parsed.variants[0],
            &["symbol", "verified_creators", "whitelist_root"],
        );
        assert_eq!(
            &parsed.variants[1].fields[0].ident, &None,
            "unnamed field has no ident"
        );
    }
}
