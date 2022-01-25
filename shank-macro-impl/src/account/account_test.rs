use proc_macro2::TokenStream;
use quote::quote;

use super::{parse_account_struct, AccountStruct, StructField};
use assert_matches::assert_matches;

fn parse(input: TokenStream) -> AccountStruct {
    parse_account_struct(input)
}

fn match_field(field: &StructField, field_ident: &str, type_ident: &str) {
    assert_matches!(field, StructField { ident, rust_type } => {
        assert_eq!(ident, field_ident);
        assert_eq!(rust_type.ident, type_ident);
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
        match_field(&res.fields[2], "treasury_withdrawal_destination", "Pubkey");
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
