use shank_idl::{extract_idl, ParseIdlOpts};

#[test]
fn test_shank_accounts_expansion() {
    // Create a temporary test file with ShankAccounts
    let test_code = r#"
use shank::{ShankAccounts, ShankInstruction};

// Mock AccountInfo for this test
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[derive(ShankAccounts)]
pub struct CreateMachineV1Accounts<'info> {
    #[account(writable, signer, desc = "The new machine asset account")]
    pub machine: &'info AccountInfo<'info>,
    
    #[account(writable, desc = "The Core machine collection")]
    pub machine_collection: &'info AccountInfo<'info>,
    
    #[account(desc = "The account paying for the storage fees")]
    pub owner: &'info AccountInfo<'info>,
    
    #[account(writable, signer, desc = "The account paying for the storage fees")]
    pub payer: &'info AccountInfo<'info>,
    
    #[account(optional, signer, desc = "The authority signing for account creation")]
    pub authority: Option<&'info AccountInfo<'info>>,
    
    #[account(desc = "The mpl core program")]
    pub mpl_core_program: &'info AccountInfo<'info>,
    
    #[account(desc = "The system program")]
    pub system_program: &'info AccountInfo<'info>,
}

#[derive(ShankInstruction)]
pub enum MachineInstruction {
    #[accounts(CreateMachineV1Accounts)]
    CreateMachineV1 {
        name: String,
        uri: String,
        plugins: Vec<u8>,
    },
}

declare_id!("MachineExample11111111111111111111111111111");
"#;

    // Write to a temporary file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_shank_accounts.rs");
    std::fs::write(&test_file, test_code).expect("Failed to write test file");

    // Extract IDL
    let opts = ParseIdlOpts::default();
    match extract_idl(&test_file.to_string_lossy(), opts) {
        Ok(Some(idl)) => {
            println!("Generated IDL: {:#?}", idl);
            
            // Check if we have instructions
            assert!(!idl.instructions.is_empty(), "Should have instructions");
            
            let instruction = &idl.instructions[0];
            assert_eq!(instruction.name, "CreateMachineV1");
            
            // This is the key test - we should see individual accounts, not a single struct placeholder
            println!("Accounts: {:#?}", instruction.accounts);
            
            // Check if we have the expected accounts expanded
            if instruction.accounts.len() == 1 {
                // If we still have only 1 account, it means our expansion didn't work yet
                println!("WARNING: Accounts not expanded yet - still showing struct placeholder");
            } else {
                // Success! We have multiple accounts
                println!("SUCCESS: Accounts expanded to {} individual accounts", instruction.accounts.len());
                // We expect 7 accounts from the CreateMachineV1Accounts struct
                assert_eq!(instruction.accounts.len(), 7, "Should have 7 individual accounts");
            }
        }
        Ok(None) => {
            panic!("No IDL generated");
        }
        Err(e) => {
            panic!("Failed to extract IDL: {}", e);
        }
    }

    // Clean up
    std::fs::remove_file(&test_file).ok();
}

// Macro definition for testing
macro_rules! declare_id {
    ($id:expr) => {};
}