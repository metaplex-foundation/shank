use shank::ShankAccounts;

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[derive(ShankAccounts)]
pub struct OptionalAccounts<'info> {
    #[account(signer, desc = "Required authority")]
    pub authority: &'info AccountInfo<'info>,
    
    #[account(optional, desc = "Optional data account")]
    pub optional_data: Option<&'info AccountInfo<'info>>,
    
    #[account(optional, signer, desc = "Optional authority")]
    pub optional_authority: Option<&'info AccountInfo<'info>>,
}

fn main() {}