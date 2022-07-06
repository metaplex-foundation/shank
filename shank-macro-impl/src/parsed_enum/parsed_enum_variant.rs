use std::convert::TryFrom;

use syn::{
    Attribute, Error as ParseError, Expr, ExprLit, Field, Ident, Lit,
    Result as ParseResult, Variant,
};

use crate::types::RustType;

// -----------------
// Enum Variant
// -----------------
#[derive(Debug, PartialEq)]
pub struct ParsedEnumVariant {
    /// The identifier name of the variant, i.e. Red
    pub ident: Ident,

    /// The fields of the variant, empty for c-like enums
    pub fields: Vec<ParsedEnumVariantField>,

    /// The variant slot starting with 0
    pub slot: usize,

    /// The variant discriminator which defaults to the slot,
    /// but can be assigned other values, i.e. Red =  0xf00
    pub discriminant: usize,

    /// Attributes found on the enum variant
    pub attrs: Vec<Attribute>,
}

impl TryFrom<(usize, usize, &Variant)> for ParsedEnumVariant {
    type Error = ParseError;

    fn try_from(
        (slot, implicit_discriminant, variant): (usize, usize, &Variant),
    ) -> ParseResult<Self> {
        let fields = variant
            .fields
            .iter()
            .enumerate()
            .map(ParsedEnumVariantField::try_from)
            .collect::<ParseResult<Vec<ParsedEnumVariantField>>>()?;

        let discriminant = match &variant.discriminant {
            Some((_, expr)) => match expr {
                Expr::Lit(ExprLit { lit, .. }) => {
                    match lit {
                        Lit::Int(lit) => {
                            lit
                                .base10_parse()
                                .map_err(|err| ParseError::new_spanned(
                                        lit,
                                        format!("Invalid discriminant value, only `usize` literals supported. {}", err)))
                        } ,
                        _ => Err( ParseError::new_spanned(
                                expr,
                                "Only literal integer enum variant discriminators supported",
                            )),

                    }
                }
                _ => Err(ParseError::new_spanned(
                    expr,
                    "Only literal enum variant discriminators supported",
                )),
            },
            None => Ok(implicit_discriminant),
        }?;

        Ok(Self {
            ident: variant.ident.clone(),
            fields,
            slot,
            discriminant,
            attrs: variant.attrs.clone(),
        })
    }
}

// -----------------
// Enum Variant Field
// -----------------
#[derive(Debug, PartialEq)]
pub struct ParsedEnumVariantField {
    /// The Rust type of the field
    pub rust_type: RustType,

    /// Name of the field, not present for tuple fields
    pub ident: Option<Ident>,

    /// The slot (starting with 0) of the field
    pub slot: usize,
}

impl TryFrom<(usize, &Field)> for ParsedEnumVariantField {
    type Error = ParseError;

    fn try_from((slot, field): (usize, &Field)) -> ParseResult<Self> {
        let rust_type = RustType::try_from(&field.ty)?;
        Ok(ParsedEnumVariantField {
            rust_type,
            ident: field.ident.clone(),
            slot,
        })
    }
}
