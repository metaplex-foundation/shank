use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, Result as ParseResult};

use crate::types::{Primitive, RustType};

use super::instruction::{Instruction, InstructionVariant};

fn parse_instruction(code: TokenStream) -> ParseResult<Option<Instruction>> {
    let item_enum = syn::parse2::<ItemEnum>(code)
        .expect("Should parse ItemEnum successfully");
    Instruction::try_from_item_enum((String::from("test_file"), &item_enum))
}

fn assert_instruction_variant(
    variant: &InstructionVariant,
    name: &str,
    expected_discriminant: usize,
    expected_field_ty: Option<RustType>,
    accounts_len: usize,
) {
    let InstructionVariant {
        ident,
        field_ty,
        accounts,
        discriminant,
    } = variant;

    assert_eq!(ident.to_string(), name);
    assert_eq!(discriminant, &expected_discriminant, "discriminant");
    assert_eq!(accounts.len(), accounts_len, "accounts");
    assert_eq!(field_ty, &expected_field_ty, "field type");
}

#[test]
fn parse_c_style_instruction() {
    let parsed = parse_instruction(quote! {
        #[derive(ShankInstruction)]
        pub enum Instruction {
            #[account(0, name = "creator", sig)]
            #[account(1, name = "thing", mut)]
            CreateThing,
            #[account(name = "creator", sig)]
            CloseThing
        }
    })
    .expect("Should parse fine")
    .expect("Should be instruction");

    assert_eq!(parsed.ident.to_string(), "Instruction", "enum ident");
    assert_eq!(parsed.variants.len(), 2, "variants");

    assert_instruction_variant(&parsed.variants[0], "CreateThing", 0, None, 2);
    assert_instruction_variant(&parsed.variants[1], "CloseThing", 1, None, 1);
}

#[test]
fn parse_custom_field_variant_instruction() {
    let parsed = parse_instruction(quote! {
        #[derive(ShankInstruction)]
        pub enum Instruction {
            CreateThing,
            #[account(name = "creator", sig)]
            CloseThing(CloseArgs)
        }
    })
    .expect("Should parse fine")
    .expect("Should be instruction");

    assert_eq!(parsed.ident.to_string(), "Instruction", "enum ident");
    assert_eq!(parsed.variants.len(), 2, "variants");

    dbg!(&parsed);

    assert_instruction_variant(&parsed.variants[0], "CreateThing", 0, None, 0);
    assert_instruction_variant(
        &parsed.variants[1],
        "CloseThing",
        1,
        Some(RustType::owned_custom_value("CloseArgs", "CloseArgs")),
        1,
    );
}

#[test]
fn parse_u8_field_variant_instruction() {
    let parsed = parse_instruction(quote! {
        #[derive(ShankInstruction)]
        pub enum Instruction {
            #[account(0, name = "creator", sig)]
            CreateThing,
            #[account(name = "creator", sig)]
            CloseThing(u8)
        }
    })
    .expect("Should parse fine")
    .expect("Should be instruction");

    assert_eq!(parsed.ident.to_string(), "Instruction", "enum ident");
    assert_eq!(parsed.variants.len(), 2, "variants");

    assert_instruction_variant(&parsed.variants[0], "CreateThing", 0, None, 1);
    assert_instruction_variant(
        &parsed.variants[1],
        "CloseThing",
        1,
        Some(RustType::owned_primitive("u8", Primitive::U8)),
        1,
    );
}

#[test]
fn parse_non_instruction_enum() {
    assert!(
        parse_instruction(quote! {
            pub enum Instruction {
                #[account(0, name = "creator", sig)]
                #[account(1, name = "thing", mut)]
                CreateThing,
                #[account(name = "creator", sig)]
                CloseThing
            }
        })
        .expect("Should parse fine")
        .is_none(),
        "should be none"
    );
    assert!(
        parse_instruction(quote! {
            #[derive(OtherDerive)]
            pub enum Instruction {
                #[account(0, name = "creator", sig)]
                #[account(1, name = "thing", mut)]
                CreateThing,
                #[account(name = "creator", sig)]
                CloseThing
            }
        })
        .expect("Should parse fine")
        .is_none(),
        "should be none"
    );
}
