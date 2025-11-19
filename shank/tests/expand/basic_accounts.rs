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
pub struct BasicAccounts<'info> {
    #[account(mut, signer, desc = "The payer account")]
    pub payer: &'info AccountInfo<'info>,
    
    #[account(mut, desc = "The data account")]
    pub data: &'info AccountInfo<'info>,
    
    #[account(desc = "The system program")]
    pub system_program: &'info AccountInfo<'info>,
}

fn main() {}