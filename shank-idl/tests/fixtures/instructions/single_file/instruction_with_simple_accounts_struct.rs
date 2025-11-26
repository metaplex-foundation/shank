use shank::{ShankAccounts, ShankInstruction};

// Simple accounts struct for testing basic functionality
#[derive(ShankAccounts)]
pub struct SimpleAccounts<'info> {
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    
    #[account(mut)]
    pub data_account: AccountInfo<'info>,
}

#[derive(ShankInstruction)]
pub enum SimpleInstruction {
    #[accounts(SimpleAccounts)]
    Execute { value: u32 },
}

// Mock AccountInfo
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}