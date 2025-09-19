use shank::{ShankAccounts, ShankInstruction};

// Define account structures using new ShankAccounts macro
#[derive(ShankAccounts)]
pub struct CreateVaultAccounts<'info> {
    #[account(mut, signer, desc = "The payer and authority")]
    pub payer: AccountInfo<'info>,
    
    #[account(mut, desc = "The vault account to create")]
    pub vault: AccountInfo<'info>,
    
    #[account(desc = "System program")]
    pub system_program: AccountInfo<'info>,
}

#[derive(ShankAccounts)]
pub struct UpdateVaultAccounts<'info> {
    #[account(mut, desc = "The vault to update")]
    pub vault: AccountInfo<'info>,
    
    #[account(signer, desc = "Vault authority")]
    pub authority: AccountInfo<'info>,
    
    #[account(optional, desc = "Optional new authority")]
    pub new_authority: Option<&'info AccountInfo<'info>>,
}

// Use account structures in instruction enum
#[derive(ShankInstruction)]
pub enum VaultInstruction {
    /// Create a new vault
    #[accounts(CreateVaultAccounts)]
    CreateVault {
        seed: String,
        space: u64,
    },
    
    /// Update vault configuration
    #[accounts(UpdateVaultAccounts)]
    UpdateVault {
        new_config: VaultConfig,
    },
}

// Supporting types
pub struct VaultConfig {
    pub fee: u64,
    pub enabled: bool,
}

// Mock AccountInfo when not using actual solana-program
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}