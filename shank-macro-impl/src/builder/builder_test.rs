use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, Result as ParseResult};

use crate::{instruction::InstructionVariantFields, types::RustType};

use super::{
    builder::{Builder, BuilderVariant},
    BuilderArgument,
};

fn parse_instruction(code: TokenStream) -> ParseResult<Option<Builder>> {
    let item_enum = syn::parse2::<ItemEnum>(code)
        .expect("Should parse ItemEnum successfully");
    Builder::try_from_item_enum(&item_enum, false)
}

fn assert_context_variant(
    variant: &BuilderVariant,
    name: &str,
    expected_discriminant: usize,
    expected_field_tys: &Vec<RustType>,
    expected_args: &Vec<BuilderArgument>,
    accounts_len: usize,
) {
    let BuilderVariant {
        ident,
        field_tys,
        accounts,
        discriminant,
        arguments,
        ..
    } = variant;

    assert_eq!(ident.to_string(), name);
    assert_eq!(discriminant, &expected_discriminant, "discriminant");
    assert_eq!(accounts.len(), accounts_len, "accounts");
    match field_tys {
        InstructionVariantFields::Named(field_tys) => {
            assert_eq!(
                field_tys.len(),
                expected_field_tys.len(),
                "fields size"
            );
            for field_idx in 0..expected_field_tys.len() {
                let (_field_name, field_ty) = field_tys.get(field_idx).unwrap();
                let expected_field_ty =
                    expected_field_tys.get(field_idx).unwrap();
                assert_eq!(field_ty, expected_field_ty, "field type");
            }
        }
        InstructionVariantFields::Unnamed(field_tys) => {
            assert_eq!(
                field_tys.len(),
                expected_field_tys.len(),
                "fields size"
            );
            for field_idx in 0..expected_field_tys.len() {
                let field_ty = field_tys.get(field_idx).unwrap();
                let expected_field_ty =
                    expected_field_tys.get(field_idx).unwrap();
                assert_eq!(field_ty, expected_field_ty, "field type");
            }
        }
    }

    assert_eq!(arguments.len(), expected_args.len(), "arguments");

    for argument_idx in 0..expected_args.len() {
        let BuilderArgument {
            name,
            ty,
            generic_ty,
        } = arguments.get(argument_idx).unwrap();

        let BuilderArgument {
            name: expected_name,
            ty: expected_ty,
            generic_ty: expected_generic_ty,
        } = expected_args.get(argument_idx).unwrap();

        assert_eq!(name, expected_name, "argument name");
        assert_eq!(ty, expected_ty, "argument type");
        assert_eq!(generic_ty, expected_generic_ty, "argument generic type");
    }
}

#[test]
fn parse_c_style_instruction_with_context() {
    let parsed = parse_instruction(quote! {
        #[derive(ShankInstruction, ShankBuilder)]
        pub enum Instruction {
            #[account(0, name = "creator", sig)]
            #[account(1, name = "thing", mut, optional)]
            #[args(first_arg: u64)]
            #[args(second_arg: u64)]
            CreateThing,
            #[account(name = "creator", sig)]
            #[args(composite_arg: Vec<u64>)]
            CloseThing,
        }
    })
    .expect("Should parse fine")
    .expect("Should be instruction");

    assert_eq!(parsed.ident.to_string(), "Instruction", "enum ident");
    assert_eq!(parsed.variants.len(), 2, "variants");
    assert!(
        !parsed.variants[0].accounts[0].optional,
        "non-optional account of first variant"
    );
    assert!(
        parsed.variants[0].accounts[1].optional,
        "optional account of first variant"
    );
    assert!(
        !parsed.variants[1].accounts[0].optional,
        "non-optional account of second variant"
    );

    assert_context_variant(
        &parsed.variants[0],
        "CreateThing",
        0,
        &vec![],
        &vec![
            BuilderArgument {
                name: String::from("first_arg"),
                ty: String::from("u64"),
                generic_ty: None,
            },
            BuilderArgument {
                name: String::from("second_arg"),
                ty: String::from("u64"),
                generic_ty: None,
            },
        ],
        2,
    );
    assert_context_variant(
        &parsed.variants[1],
        "CloseThing",
        1,
        &vec![],
        &vec![BuilderArgument {
            name: String::from("composite_arg"),
            ty: String::from("Vec"),
            generic_ty: Some(String::from("u64")),
        }],
        1,
    );
}
