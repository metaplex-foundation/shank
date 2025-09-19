use shank::ShankAccounts;

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[derive(ShankAccounts)]
pub struct ComplexAccounts<'info> {
    #[account(mut, signer, desc = "Payer and authority")]
    pub payer: &'info AccountInfo<'info>,
    
    #[account(mut, desc = "Mutable data")]
    pub data: &'info AccountInfo<'info>,
    
    #[account(desc = "Read-only account")]
    pub read_only: &'info AccountInfo<'info>,
    
    #[account(optional, mut, desc = "Optional mutable account")]
    pub optional_mut: Option<&'info AccountInfo<'info>>,
    
    #[account(optional, signer, desc = "Optional signer")]
    pub optional_signer: Option<&'info AccountInfo<'info>>,
    
    #[account(optional, mut, signer, desc = "Optional mutable signer")]
    pub optional_mut_signer: Option<&'info AccountInfo<'info>>,
}

fn main() {}