use shank::{ShankAccounts, ShankInstruction};

// Mock AccountInfo for testing (in real programs, import from solana_program)
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32], // Mock pubkey
    pub data: &'info [u8],
    pub owner: &'info [u8; 32], // Mock pubkey
}

// Test the enhanced ShankAccounts that generates context structs
#[test]
fn test_enhanced_accounts_with_context() {
    #[derive(ShankAccounts)]
    pub struct CreateVaultAccounts<'info> {
        #[account(mut, desc = "Vault account")]
        pub vault: AccountInfo<'info>,
        
        #[account(signer, desc = "Authority")]
        pub authority: AccountInfo<'info>,
        
        #[account(optional, desc = "Optional account")]
        pub optional_account: AccountInfo<'info>,
        
        #[account(desc = "System program")]
        pub system_program: AccountInfo<'info>,
    }

    // Test that the context struct is generated
    // This should generate CreateVaultAccountsContext<'a>
    // We can't easily test the runtime functionality without actual AccountInfo,
    // but we can test that it compiles
    
    // The generated code should provide:
    // - CreateVaultAccountsContext<'a> struct
    // - CreateVaultAccounts::context() method
    // - CreateVaultAccountsContext::from_accounts() method
    
    // This test primarily ensures the enhanced macro compiles successfully
}

// Test instruction using enhanced accounts struct
#[test] 
fn test_instruction_with_enhanced_accounts() {
    #[derive(ShankAccounts)]
    pub struct UpdateVaultAccounts<'info> {
        #[account(mut, desc = "Vault to update")]
        pub vault: AccountInfo<'info>,
        
        #[account(signer, desc = "Authority")]
        pub authority: AccountInfo<'info>,
    }
    
    #[derive(ShankInstruction)]
    pub enum VaultInstruction {
        #[accounts(UpdateVaultAccounts)]
        Update { new_value: u64 },
    }
    
    // This should work with both the old account attribute system and the new struct system
}

// Test multiple account structs with different configurations
#[test]
fn test_multiple_account_configurations() {
    #[derive(ShankAccounts)]
    pub struct SimpleAccounts<'info> {
        #[account(mut, signer)]
        pub payer: AccountInfo<'info>,
        
        #[account(mut)]
        pub data_account: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct ComplexAccounts<'info> {
        #[account(mut, signer, desc = "The payer")]
        pub payer: AccountInfo<'info>,
        
        #[account(writable, desc = "Data storage")]
        pub data: AccountInfo<'info>,
        
        #[account(optional_signer, desc = "Optional authority")]
        pub authority: AccountInfo<'info>,
        
        #[account(optional, desc = "Optional account")]
        pub optional_account: AccountInfo<'info>,
        
        pub system_program: AccountInfo<'info>,
    }
    
    #[derive(ShankInstruction)]
    pub enum TestInstruction {
        #[accounts(SimpleAccounts)]
        Simple,
        
        #[accounts(ComplexAccounts)]
        Complex { data: [u8; 32] },
    }
}

// Test backward compatibility with traditional account attributes
#[test]
fn test_backward_compatibility_with_traditional_accounts() {
    #[derive(ShankAccounts)]
    pub struct NewStyleAccounts<'info> {
        #[account(mut, desc = "New style account")]
        pub account: AccountInfo<'info>,
        
        #[account(signer)]
        pub authority: AccountInfo<'info>,
    }
    
    #[derive(ShankInstruction)]
    pub enum MixedInstruction {
        // New style using accounts struct
        #[accounts(NewStyleAccounts)]
        NewStyle,
        
        // Old style using inline attributes  
        #[account(0, writable, name="data", desc="Data account")]
        #[account(1, signer, name="authority", desc="Authority")]
        OldStyle,
    }
}