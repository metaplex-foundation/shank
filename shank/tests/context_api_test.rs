use shank::ShankAccounts;

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

// Test the new Context API pattern
#[test]
fn test_context_api_pattern() {
    #[derive(ShankAccounts)]
    pub struct CreateMachineV1Accounts<'info> {
        #[account(mut, signer, desc = "The new machine asset account")]
        pub machine: &'info AccountInfo<'info>,
        
        #[account(mut, desc = "The Core machine collection")]
        pub machine_collection: &'info AccountInfo<'info>,
        
        #[account(desc = "The account paying for the storage fees")]
        pub owner: &'info AccountInfo<'info>,
        
        #[account(mut, signer, desc = "The account paying for the storage fees")]
        pub payer: &'info AccountInfo<'info>,
        
        #[account(optional, signer, desc = "The authority signing for account creation")]
        pub authority: Option<&'info AccountInfo<'info>>,
        
        #[account(desc = "The mpl core program")]
        pub mpl_core_program: &'info AccountInfo<'info>,
        
        #[account(desc = "The system program")]
        pub system_program: &'info AccountInfo<'info>,
    }

    // Test that the IDL metadata is generated correctly
    let metadata = CreateMachineV1Accounts::__shank_accounts();
    assert_eq!(metadata.len(), 7);
    
    // Check a few key accounts
    assert_eq!(metadata[0].1, "machine");
    assert_eq!(metadata[0].2, true);  // mut
    assert_eq!(metadata[0].3, true);  // signer
    
    assert_eq!(metadata[4].1, "authority");
    assert_eq!(metadata[4].5, true);  // optional
    assert_eq!(metadata[4].3, true);  // signer (optional_signer translates to both optional and signer)
}

#[test]
fn test_accounts_struct_with_references() {
    #[derive(ShankAccounts)]
    pub struct TestAccounts<'info> {
        #[account(mut, signer)]
        pub payer: &'info AccountInfo<'info>,
        
        #[account(mut)]
        pub data: &'info AccountInfo<'info>,
        
        #[account(optional)]
        pub optional_account: Option<&'info AccountInfo<'info>>,
    }

    let metadata = TestAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 3);
    
    // Verify account details
    assert_eq!(metadata[0].1, "payer");
    assert_eq!(metadata[0].2, true);  // mut
    assert_eq!(metadata[0].3, true);  // signer
    
    assert_eq!(metadata[1].1, "data"); 
    assert_eq!(metadata[1].2, true);  // mut
    assert_eq!(metadata[1].3, false); // not signer
    
    assert_eq!(metadata[2].1, "optional_account");
    assert_eq!(metadata[2].5, true);  // optional
}

// Test different lifetime names work
#[test]
fn test_different_lifetime_names() {
    #[derive(ShankAccounts)]
    pub struct CustomLifetimeAccounts<'a> {
        #[account(signer)]
        pub authority: &'a AccountInfo<'a>,
        
        #[account(mut)]
        pub data: &'a AccountInfo<'a>,
    }
    
    let metadata = CustomLifetimeAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 2);
    assert_eq!(metadata[0].1, "authority");
    assert_eq!(metadata[1].1, "data");
}

#[test] 
fn test_no_constraints_with_references() {
    #[derive(ShankAccounts)]
    pub struct SimpleAccounts<'info> {
        pub read_only1: &'info AccountInfo<'info>,
        pub read_only2: &'info AccountInfo<'info>,
    }
    
    let metadata = SimpleAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 2);
    
    // Both should be read-only (no constraints)
    assert_eq!(metadata[0].2, false); // not mut
    assert_eq!(metadata[0].3, false); // not signer
    assert_eq!(metadata[1].2, false); // not mut  
    assert_eq!(metadata[1].3, false); // not signer
}