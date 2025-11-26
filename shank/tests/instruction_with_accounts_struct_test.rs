use shank::{ShankAccounts, ShankInstruction};

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

// Mock AccountInfo for testing (in real programs, import from solana_program)
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32], // Mock pubkey
    pub data: &'info [u8],
    pub owner: &'info [u8; 32], // Mock pubkey
}

// Define accounts structs using the new ShankAccounts macro with real AccountInfo types
#[derive(ShankAccounts)]
pub struct CreateVaultAccounts<'info> {
    #[account(mut, desc = "Initialized fractional share mint")]
    pub fraction_mint: &'info AccountInfo<'info>,

    #[account(mut, desc = "Initialized redeem treasury")]
    pub redeem_treasury: &'info AccountInfo<'info>,

    #[account(mut, desc = "Fraction treasury")]
    pub fraction_treasury: &'info AccountInfo<'info>,

    #[account(mut, desc = "Uninitialized vault account")]
    pub vault: &'info AccountInfo<'info>,

    #[account(optional_signer, desc = "Authority on the vault")]
    pub authority: Option<&'info AccountInfo<'info>>,

    #[account(desc = "Pricing Lookup Address")]
    pub pricing_lookup_address: &'info AccountInfo<'info>,

    #[account(desc = "Token program")]
    pub token_program: &'info AccountInfo<'info>,

    #[account(desc = "Rent sysvar")]
    pub rent: &'info AccountInfo<'info>,
}

#[derive(ShankAccounts)]
pub struct ActivateVaultAccounts<'info> {
    #[account(
        mut,
        desc = "Initialized inactivated fractionalized token vault"
    )]
    pub vault: &'info AccountInfo<'info>,

    #[account(mut, desc = "Fraction mint")]
    pub fraction_mint: &'info AccountInfo<'info>,

    #[account(mut, desc = "Fraction treasury")]
    pub fraction_treasury: &'info AccountInfo<'info>,

    #[account(desc = "Fraction mint authority for the program")]
    pub fraction_mint_authority: &'info AccountInfo<'info>,

    #[account(signer, desc = "Authority on the vault")]
    pub vault_authority: &'info AccountInfo<'info>,

    #[account(desc = "Token program")]
    pub token_program: &'info AccountInfo<'info>,
}

// Test instruction enum using the new #[accounts(StructName)] attribute
#[test]
fn test_instruction_with_accounts_struct_compiles() {
    #[derive(Debug, Clone, ShankInstruction)]
    pub enum VaultInstruction {
        /// Initialize a token vault
        #[accounts(CreateVaultAccounts)]
        InitVault(u8),

        /// Activates the vault
        #[accounts(ActivateVaultAccounts)]
        ActivateVault(u64),
    }
}

// Test instruction with simple accounts struct
#[test]
fn test_simple_accounts_struct_compiles() {
    #[derive(ShankAccounts)]
    pub struct SimpleAccounts<'info> {
        #[account(mut, signer)]
        pub payer: &'info AccountInfo<'info>,

        #[account(mut)]
        pub account_to_modify: &'info AccountInfo<'info>,

        pub system_program: &'info AccountInfo<'info>,
    }

    #[derive(Debug, Clone, ShankInstruction)]
    pub enum SimpleInstruction {
        #[accounts(SimpleAccounts)]
        Execute,

        #[accounts(SimpleAccounts)]
        ExecuteWithArgs(u64),
    }
}

// Test that the old-style account attributes still work
#[test]
fn test_backward_compatibility() {
    #[derive(Debug, Clone, ShankInstruction)]
    pub enum OldStyleInstruction {
        #[account(0, writable, name = "vault", desc = "Vault account")]
        #[account(1, signer, name = "authority", desc = "Authority")]
        #[account(2, name = "system_program", desc = "System program")]
        CreateVaultOldStyle,
    }
}
