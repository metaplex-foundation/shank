use proc_macro2::TokenStream;
use quote::quote;

use super::{parse_accounts_struct, AccountsStruct};

fn parse(input: TokenStream) -> AccountsStruct {
    parse_accounts_struct(input)
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
        eprintln!("{:#?}", res)
    }
}

mod accounts_mpl_examples_metaplex {

    use super::*;

    #[test]
    fn auction_manager_state_v2() {
        let res = parse(quote! {
            pub struct AuctionManagerStateV2 {
                pub status: AuctionManagerStatus,
                /// When all configs are validated the auction is started and auction manager moves to Running
                pub safety_config_items_validated: u64,
                /// how many bids have been pushed to accept payment
                pub bids_pushed_to_accept_payment: u64,

                pub has_participation: bool,
            }
        });
        eprintln!("{:#?}", res)
    }
}
