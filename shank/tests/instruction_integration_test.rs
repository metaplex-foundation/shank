use shank::{ShankAccounts, ShankInstruction};

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[test]
fn test_accounts_with_instruction_integration() {
    // Define accounts struct
    #[derive(ShankAccounts)]
    pub struct CreateTokenAccounts<'info> {
        #[account(mut, signer, desc = "Payer and authority")]
        pub payer: AccountInfo<'info>,
        
        #[account(mut, desc = "Token mint to create")]
        pub mint: AccountInfo<'info>,
        
        #[account(desc = "System program")]
        pub system_program: AccountInfo<'info>,
        
        #[account(desc = "Token program")]
        pub token_program: AccountInfo<'info>,
    }
    
    // Use accounts struct in instruction
    #[derive(ShankInstruction)]
    pub enum TokenInstruction {
        #[accounts(CreateTokenAccounts)]
        CreateToken { 
            decimals: u8,
            supply: u64,
        },
    }
    
    // Test that both compile and work together
    let accounts_metadata = CreateTokenAccounts::__shank_accounts();
    assert_eq!(accounts_metadata.len(), 4);
    
    // Verify the accounts are properly structured
    assert_eq!(accounts_metadata[0].1, "payer");
    assert_eq!(accounts_metadata[0].2, true);  // mut
    assert_eq!(accounts_metadata[0].3, true);  // signer
    
    assert_eq!(accounts_metadata[1].1, "mint");
    assert_eq!(accounts_metadata[1].2, true);  // mut
    assert_eq!(accounts_metadata[1].3, false); // not signer
}

#[test]
fn test_multiple_instructions_different_accounts() {
    // Define different account structures for different instructions
    #[derive(ShankAccounts)]
    pub struct InitializeAccounts<'info> {
        #[account(mut, signer)]
        pub authority: AccountInfo<'info>,
        
        #[account(mut, desc = "Account to initialize")]
        pub target: AccountInfo<'info>,
        
        pub system_program: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct TransferAccounts<'info> {
        #[account(signer, desc = "Transfer authority")]
        pub authority: AccountInfo<'info>,
        
        #[account(mut, desc = "Source account")]
        pub from: AccountInfo<'info>,
        
        #[account(mut, desc = "Destination account")]
        pub to: AccountInfo<'info>,
        
        #[account(optional, desc = "Optional fee account")]
        pub fee_account: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct CloseAccounts<'info> {
        #[account(mut, signer)]
        pub authority: AccountInfo<'info>,
        
        #[account(mut, desc = "Account to close")]
        pub target: AccountInfo<'info>,
        
        #[account(mut, desc = "Destination for funds")]
        pub destination: AccountInfo<'info>,
    }
    
    // Use all different accounts in one instruction enum
    #[derive(ShankInstruction)]
    pub enum ProgramInstruction {
        #[accounts(InitializeAccounts)]
        Initialize,
        
        #[accounts(TransferAccounts)]
        Transfer { amount: u64 },
        
        #[accounts(CloseAccounts)]
        Close,
        
        // Test mixing new accounts style with old style
        #[account(0, writable, name = "data", desc = "Data account")]
        #[account(1, signer, name = "authority", desc = "Authority")]
        OldStyle { value: u32 },
    }
    
    // Test that each accounts struct works independently
    assert_eq!(InitializeAccounts::__shank_accounts().len(), 3);
    assert_eq!(TransferAccounts::__shank_accounts().len(), 4);
    assert_eq!(CloseAccounts::__shank_accounts().len(), 3);
    
    // Verify specific account configurations
    let init_accounts = InitializeAccounts::__shank_accounts();
    assert_eq!(init_accounts[0].1, "authority");
    assert_eq!(init_accounts[0].2, true);  // mut
    assert_eq!(init_accounts[0].3, true);  // signer
    
    let transfer_accounts = TransferAccounts::__shank_accounts();
    assert_eq!(transfer_accounts[3].1, "fee_account");
    assert_eq!(transfer_accounts[3].5, true);  // optional
}

#[test]
fn test_complex_program_structure() {
    // Simulate a more complex program with various account patterns
    
    // Simple accounts for basic operations
    #[derive(ShankAccounts)]
    pub struct BasicAccounts<'info> {
        #[account(signer)]
        pub user: AccountInfo<'info>,
    }
    
    // Accounts for creating something
    #[derive(ShankAccounts)]
    pub struct CreateAccounts<'info> {
        #[account(mut, signer, desc = "Payer")]
        pub payer: AccountInfo<'info>,
        
        #[account(mut, desc = "New account")]
        pub new_account: AccountInfo<'info>,
        
        pub system_program: AccountInfo<'info>,
    }
    
    // Accounts for updating with optional authority
    #[derive(ShankAccounts)]
    pub struct UpdateAccounts<'info> {
        #[account(mut, desc = "Account to update")]
        pub target: AccountInfo<'info>,
        
        #[account(signer, desc = "Current authority")]
        pub current_authority: AccountInfo<'info>,
        
        #[account(optional, desc = "New authority")]
        pub new_authority: AccountInfo<'info>,
    }
    
    // Accounts for administrative operations
    #[derive(ShankAccounts)]
    pub struct AdminAccounts<'info> {
        #[account(signer, desc = "Admin authority")]
        pub admin: AccountInfo<'info>,
        
        #[account(mut, desc = "Config account")]
        pub config: AccountInfo<'info>,
        
        #[account(optional_signer, desc = "Optional co-signer")]
        pub co_signer: AccountInfo<'info>,
    }
    
    // Main instruction enum using all account types
    #[derive(ShankInstruction)]
    pub enum ComplexProgram {
        #[accounts(BasicAccounts)]
        Ping,
        
        #[accounts(CreateAccounts)]
        Create { 
            space: u64,
            seed: String,
        },
        
        #[accounts(UpdateAccounts)]
        Update {
            new_data: Vec<u8>,
        },
        
        #[accounts(AdminAccounts)]
        SetConfig {
            new_fee: u64,
            enabled: bool,
        },
        
        // Test that we can still mix with old-style if needed
        #[account(0, name = "emergency", desc = "Emergency account")]
        Emergency,
    }
    
    // Verify all account structures work correctly
    assert_eq!(BasicAccounts::__shank_accounts().len(), 1);
    assert_eq!(CreateAccounts::__shank_accounts().len(), 3);
    assert_eq!(UpdateAccounts::__shank_accounts().len(), 3);
    assert_eq!(AdminAccounts::__shank_accounts().len(), 3);
    
    // Verify specific constraints are applied correctly
    let admin_accounts = AdminAccounts::__shank_accounts();
    assert_eq!(admin_accounts[0].1, "admin");
    assert_eq!(admin_accounts[0].3, true);  // signer
    
    assert_eq!(admin_accounts[1].1, "config");
    assert_eq!(admin_accounts[1].2, true);  // mut
    
    assert_eq!(admin_accounts[2].1, "co_signer");
    assert_eq!(admin_accounts[2].4, true);  // optional_signer
}

#[test] 
fn test_nested_instruction_data_structures() {
    #[derive(ShankAccounts)]
    pub struct ComplexDataAccounts<'info> {
        #[account(mut, signer)]
        pub authority: AccountInfo<'info>,
        
        #[account(mut)]
        pub data_account: AccountInfo<'info>,
    }
    
    // Test with complex instruction data
    #[derive(ShankInstruction)]
    pub enum DataInstruction {
        #[accounts(ComplexDataAccounts)]
        ProcessData {
            // Test various data types work with accounts
            simple_data: u64,
            string_data: String,
            array_data: [u8; 32],
            vec_data: Vec<u64>,
            optional_data: Option<u32>,
        },
        
        #[accounts(ComplexDataAccounts)]
        BatchProcess {
            operations: Vec<String>,
            metadata: Option<Vec<u8>>,
        },
    }
    
    // Should compile successfully
    let metadata = ComplexDataAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 2);
}

#[test]
fn test_accounts_reuse_across_instructions() {
    // Test that the same accounts struct can be used by multiple instructions
    #[derive(ShankAccounts)]
    pub struct SharedAccounts<'info> {
        #[account(mut, signer, desc = "Shared authority")]
        pub authority: AccountInfo<'info>,
        
        #[account(mut, desc = "Shared data account")]
        pub data: AccountInfo<'info>,
        
        pub system_program: AccountInfo<'info>,
    }
    
    #[derive(ShankInstruction)]
    pub enum SharedInstruction {
        #[accounts(SharedAccounts)]
        Operation1 { value: u32 },
        
        #[accounts(SharedAccounts)]
        Operation2 { flag: bool },
        
        #[accounts(SharedAccounts)]
        Operation3,
    }
    
    // The same accounts metadata should work for all instructions
    let shared_metadata = SharedAccounts::__shank_accounts();
    assert_eq!(shared_metadata.len(), 3);
    
    // Verify the accounts are configured correctly for reuse
    assert_eq!(shared_metadata[0].1, "authority");
    assert_eq!(shared_metadata[0].2, true);  // mut
    assert_eq!(shared_metadata[0].3, true);  // signer
    
    assert_eq!(shared_metadata[1].1, "data");
    assert_eq!(shared_metadata[1].2, true);  // mut
    assert_eq!(shared_metadata[1].3, false); // not signer
    
    assert_eq!(shared_metadata[2].1, "system_program");
    assert_eq!(shared_metadata[2].2, false); // not mut
    assert_eq!(shared_metadata[2].3, false); // not signer
}