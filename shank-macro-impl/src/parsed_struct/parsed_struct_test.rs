use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::types::{Composite, RustType, TypeKind};

use super::{
    parse_struct, struct_field_attr::StructFieldAttr, ParsedStruct, StructField,
};
use assert_matches::assert_matches;

fn parse(input: TokenStream) -> ParsedStruct {
    parse_struct(input)
}

fn match_field(field: &StructField, field_ident: &str, type_ident: &str) {
    assert_matches!(field, StructField { ident, rust_type, attrs } => {
        assert_eq!(ident, field_ident);
        assert_eq!(rust_type.ident, type_ident);
        assert_eq!(attrs, &HashSet::new());
    });
}

fn match_vec_field(field: &StructField, field_ident: &str, inner_ty: &str) {
    assert_matches!(field, StructField { ident, rust_type, .. } => {
        assert_eq!(ident, field_ident);
        assert_eq!(rust_type.ident, "Vec");
        let vec_inner = rust_type.kind.inner_composite_rust_type().expect("should have inner vec type");
        assert_eq!(vec_inner.ident, inner_ty, "inner vec type");
    });
}

fn match_array_field(
    field: &StructField,
    field_ident: &str,
    inner_ty: &str,
    size: usize,
) {
    assert_matches!(field, StructField { ident, rust_type, .. } => {
        assert_eq!(ident, field_ident);
        assert_eq!(rust_type.ident, "Array");
        assert_matches!(&rust_type.kind, TypeKind::Composite(Composite::Array(array_size), inner)  => {
            let inner_rust_ty = inner.get(0).expect("array should have inner type");
            assert_eq!(inner_rust_ty.ident, inner_ty, "inner array type");
            assert_eq!(*array_size, size, "array size");
        });
    });
}

fn match_array_field_with_attrs(
    field: &StructField,
    field_ident: &str,
    inner_ty: &str,
    size: usize,
    field_attrs: &HashSet<StructFieldAttr>,
) {
    assert_matches!(field, StructField { ident, rust_type, attrs } => {
        assert_eq!(ident, field_ident);
        assert_eq!(rust_type.ident, "Array");
        assert_matches!(&rust_type.kind, TypeKind::Composite(Composite::Array(array_size), inner)  => {
            let inner_rust_ty = inner.get(0).expect("array should have inner type");
            assert_eq!(inner_rust_ty.ident, inner_ty, "inner array type");
            assert_eq!(*array_size, size, "array size");
        });
        assert_eq!(attrs, field_attrs);
    });
}

// -----------------
// Tuple asserts
// -----------------
fn assert_tuple(
    ident: &Ident,
    field_ident: &str,
    rust_type: &RustType,
    inner_tys: &[&str],
) {
    assert_eq!(ident, field_ident);
    assert_eq!(rust_type.ident, "Tuple");
    match &rust_type.kind {
        TypeKind::Composite(_, inners) => {
            for (idx, inner) in inners.iter().enumerate() {
                assert_eq!(inner.ident.to_string(), inner_tys[idx]);
            }
        }
        _ => assert!(false, "expected composite to represent tuple"),
    };
}

fn match_tuple_field(
    field: &StructField,
    field_ident: &str,
    inner_tys: &[&str],
) {
    assert_matches!(field, StructField { ident, rust_type, .. } => {
        assert_tuple(ident, field_ident, rust_type, inner_tys);
    });
}

fn match_inner_tuple_field(
    field: &StructField,
    field_ident: &str,
    inner_tys: &[&str],
) {
    assert_matches!(field, StructField { ident, rust_type, .. } => {
        let rust_type = rust_type.kind.inner_composite_rust_type().unwrap();
        assert_tuple(ident, field_ident, &rust_type, inner_tys);
    });
}

fn match_inner_tuples_field(
    field: &StructField,
    field_ident: &str,
    inner_tys1: &[&str],
    inner_tys2: &[&str],
) {
    assert_matches!(field, StructField { ident, rust_type, .. } => {
        let (rt1, rt2) = rust_type.kind.inner_composite_rust_types();
        assert_tuple(ident, field_ident, &rt1.unwrap(), inner_tys1);
        assert_tuple(ident, field_ident, &rt2.unwrap(), inner_tys2);
    });
}

mod accounts_mpl_examples_auction_house {

    use super::*;

    #[test]
    fn auction_house() {
        let res = parse(quote! {
            pub struct AuctionHouse {
                pub auction_house_fee_account: Pubkey,
                pub auction_house_treasury: Pubkey,
                pub treasury_withdrawal_destination: Pubkey,
                pub fee_withdrawal_destination: Pubkey,
                pub treasury_mint: Pubkey,
                pub authority: Pubkey,
                pub creator: Pubkey,
                pub bump: u8,
                pub treasury_bump: u8,
                pub fee_payer_bump: u8,
                pub seller_fee_basis_points: u16,
                pub requires_sign_off: bool,
                pub can_change_sale_price: bool,
            }
        });

        assert_eq!(res.ident.to_string(), "AuctionHouse");
        match_field(&res.fields[0], "auction_house_fee_account", "Pubkey");
        match_field(&res.fields[1], "auction_house_treasury", "Pubkey");
        match_field(
            &res.fields[2],
            "treasury_withdrawal_destination",
            "Pubkey",
        );
        match_field(&res.fields[3], "fee_withdrawal_destination", "Pubkey");
        match_field(&res.fields[4], "treasury_mint", "Pubkey");
        match_field(&res.fields[5], "authority", "Pubkey");
        match_field(&res.fields[6], "creator", "Pubkey");
        match_field(&res.fields[7], "bump", "u8");
        match_field(&res.fields[8], "treasury_bump", "u8");
        match_field(&res.fields[9], "fee_payer_bump", "u8");
        match_field(&res.fields[10], "seller_fee_basis_points", "u16");
        match_field(&res.fields[11], "requires_sign_off", "bool");
        match_field(&res.fields[12], "can_change_sale_price", "bool");
    }
}

mod accounts_mpl_examples_metaplex {

    use super::*;

    #[test]
    fn auction_manager_state_v2() {
        let res = parse(quote! {
            pub struct AuctionManagerStateV2 {
                pub status: AuctionManagerStatus,
                pub safety_config_items_validated: u64,
                pub bids_pushed_to_accept_payment: u64,
                pub has_participation: bool,
            }
        });
        assert_eq!(res.ident.to_string(), "AuctionManagerStateV2");
        match_field(&res.fields[0], "status", "AuctionManagerStatus");
        match_field(&res.fields[1], "safety_config_items_validated", "u64");
        match_field(&res.fields[2], "bids_pushed_to_accept_payment", "u64");
        match_field(&res.fields[3], "has_participation", "bool");
    }
}

mod account_collection_examples {
    use super::*;

    #[test]
    fn vec() {
        let res = parse(quote! {
            pub struct AccountWithVecs {
                pub u8s: Vec<u8>,
                pub u64s: Vec<u64>,
                pub u128s: Vec<u128>,
                pub strings: Vec<String>,
                pub pubkeys: Vec<Pubkey>,
            }
        });
        match_vec_field(&res.fields[0], "u8s", "u8");
        match_vec_field(&res.fields[1], "u64s", "u64");
        match_vec_field(&res.fields[2], "u128s", "u128");
        match_vec_field(&res.fields[3], "strings", "String");
        match_vec_field(&res.fields[4], "pubkeys", "Pubkey");
    }

    #[test]
    fn sized_array() {
        let res = parse(quote! {
            pub struct AccountWithSizedArrays {
                pub u8s: [u8; 32],
                pub u64s: [u64; 16],
                pub strings: [String; 2],
                pub pubkeys: [Pubkey; 22],
            }
        });
        match_array_field(&res.fields[0], "u8s", "u8", 32);
        match_array_field(&res.fields[1], "u64s", "u64", 16);
        match_array_field(&res.fields[2], "strings", "String", 2);
        match_array_field(&res.fields[3], "pubkeys", "Pubkey", 22);
    }
}

mod account_with_padding_examples {
    use super::*;
    fn padding_attrs() -> HashSet<StructFieldAttr> {
        let mut set = HashSet::new();
        set.insert(StructFieldAttr::Padding);
        set
    }

    #[test]
    fn account_with_padding() {
        let res = parse(quote! {
            pub struct AccountWithPaddedFieldAtEnd {
                pub has_participation: bool,
                #[padding]
                _account_padding: [u8;36]
            }
        });
        match_field(&res.fields[0], "has_participation", "bool");
        match_array_field_with_attrs(
            &res.fields[1],
            "_account_padding",
            "u8",
            36,
            &padding_attrs(),
        );
    }

    #[test]
    fn account_with_only_padding() {
        let res = parse(quote! {
            pub struct AccountWithPaddedFieldAtEnd {
                #[padding]
                _account_padding: [u8;10]
            }
        });
        match_array_field_with_attrs(
            &res.fields[0],
            "_account_padding",
            "u8",
            10,
            &padding_attrs(),
        );
    }

    #[test]
    fn account_with_two_paddings() {
        let res = parse(quote! {
            pub struct AccountWithPaddedFieldAtEnd {
                pub has_participation: bool,
                #[padding]
                _account_padding: [u8;2],
                #[padding]
                _more_account_padding: [u8;3]
            }
        });
        match_field(&res.fields[0], "has_participation", "bool");
        match_array_field_with_attrs(
            &res.fields[1],
            "_account_padding",
            "u8",
            2,
            &padding_attrs(),
        );
        match_array_field_with_attrs(
            &res.fields[2],
            "_more_account_padding",
            "u8",
            3,
            &padding_attrs(),
        );
    }
}

mod account_tuples_examples {
    use super::*;

    #[test]
    fn top_level_tuples() {
        let res = parse(quote! {
            pub struct AccountWithTuples {
                pub u8_u8: (u8, u8),
                pub u8_u16_u32_u64: (u8, u16, u32, u64),
                pub u8_string_custom: (u8, String, Custom),
            }
        });
        match_tuple_field(&res.fields[0], "u8_u8", &["u8", "u8"]);
        match_tuple_field(
            &res.fields[1],
            "u8_u16_u32_u64",
            &["u8", "u16", "u32", "u64"],
        );
        match_tuple_field(
            &res.fields[2],
            "u8_string_custom",
            &["u8", "String", "Custom"],
        );
    }

    #[test]
    fn nested_tuples() {
        let res = parse(quote! {
            pub struct AccountWithNestedTuples {
                pub vec_u8_u8: Vec<(u8, u8)>,
                pub hashmap_string_u8_u16_option_u8_i16: HashMap<(String, u8, u16), (Option<u8>, i16)>,
            }
        });
        match_inner_tuple_field(&res.fields[0], "vec_u8_u8", &["u8", "u8"]);
        match_inner_tuples_field(
            &res.fields[1],
            "hashmap_string_u8_u16_option_u8_i16",
            &["String", "u8", "u16"],
            &["Option", "i16"],
        );
    }
}
