use shank::ShankAccounts;

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[derive(ShankAccounts)]
pub struct CustomLifetime<'a> {
    #[account(signer, desc = "Authority with custom lifetime")]
    pub authority: &'a AccountInfo<'a>,
    
    #[account(mut, desc = "Data with custom lifetime")]
    pub data: &'a AccountInfo<'a>,
}

fn main() {}