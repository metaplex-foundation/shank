use shank::{ShankAccounts, ShankInstruction};

// Complex accounts struct testing all constraint types
#[derive(ShankAccounts)]
pub struct ComplexAccounts<'info> {
    #[account(mut, signer, desc = "Payer and authority")]
    pub payer: AccountInfo<'info>,
    
    #[account(mut, desc = "Mutable data account")]
    pub data_account: AccountInfo<'info>,
    
    #[account(signer, desc = "Additional signer")]
    pub signer_account: AccountInfo<'info>,
    
    #[account(optional, desc = "Optional account")]
    pub optional_account: Option<&'info AccountInfo<'info>>,
    
    #[account(optional_signer, desc = "Optional signer")]
    pub optional_signer: Option<&'info AccountInfo<'info>>,
    
    #[account(desc = "Read-only system program")]
    pub system_program: AccountInfo<'info>,
}

// Mixed instruction with both new and old style accounts
#[derive(ShankInstruction)]
pub enum MixedInstruction {
    /// Uses new accounts struct style
    #[accounts(ComplexAccounts)]
    NewStyleInstruction {
        amount: u64,
        metadata: Vec<u8>,
    },
    
    /// Uses old-style account attributes
    #[account(0, writable, signer, name = "authority", desc = "The authority")]
    #[account(1, writable, name = "target", desc = "Target account")]
    OldStyleInstruction {
        value: u32,
    },
}

// Mock AccountInfo
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}