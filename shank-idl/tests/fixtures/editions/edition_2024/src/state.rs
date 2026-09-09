use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

#[derive(ShankAccount)]
#[seeds("counter", authority("The authority of the counter"))]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
    #[idl_name("lastUpdatedAt")]
    pub last_updated_at: i64,
    #[padding]
    pub _padding: [u8; 8],
    #[skip]
    pub cache: Option<u64>,
}
