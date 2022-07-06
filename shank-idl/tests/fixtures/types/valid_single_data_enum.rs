// https://github.com/metaplex-foundation/solita/issues/53#issuecomment-1133910360
#[derive(BorshSerialize)]
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
