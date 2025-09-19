use shank::ShankAccounts;

// Mock AccountInfo for testing (in real programs, import from solana_program)
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32], // Mock pubkey
    pub data: &'info [u8],
    pub owner: &'info [u8; 32], // Mock pubkey
}

// Mock Pubkey for testing
pub struct Pubkey([u8; 32]);

// Mock ProgramError for testing
#[derive(Debug, PartialEq)]
pub enum ProgramError {
    NotEnoughAccountKeys,
}

#[test]
fn test_context_struct_generation() {
    #[derive(ShankAccounts)]
    pub struct TestAccounts<'info> {
        #[account(mut, desc = "Mutable account")]
        pub mutable_account: &'info AccountInfo<'info>,
        
        #[account(signer, desc = "Signer account")]
        pub signer_account: &'info AccountInfo<'info>,
        
        #[account(mut, signer, desc = "Mutable signer")]
        pub mutable_signer: &'info AccountInfo<'info>,
        
        #[account(optional, desc = "Optional account")]
        pub optional_account: &'info AccountInfo<'info>,
        
        #[account(desc = "Regular account")]
        pub regular_account: &'info AccountInfo<'info>,
    }

    // Test that the macro generates the expected IDL metadata method
    let accounts_metadata = TestAccounts::__shank_accounts();
    assert_eq!(accounts_metadata.len(), 5);
    
    // Check first account (mutable_account)
    assert_eq!(accounts_metadata[0].0, 0); // index
    assert_eq!(accounts_metadata[0].1, "mutable_account"); // name
    assert_eq!(accounts_metadata[0].2, true); // writable
    assert_eq!(accounts_metadata[0].3, false); // signer
    assert_eq!(accounts_metadata[0].4, false); // optional_signer
    assert_eq!(accounts_metadata[0].5, false); // optional
    
    // Check second account (signer_account)
    assert_eq!(accounts_metadata[1].0, 1); // index
    assert_eq!(accounts_metadata[1].1, "signer_account"); // name
    assert_eq!(accounts_metadata[1].2, false); // writable
    assert_eq!(accounts_metadata[1].3, true); // signer
    assert_eq!(accounts_metadata[1].4, false); // optional_signer
    assert_eq!(accounts_metadata[1].5, false); // optional
    
    // Check third account (mutable_signer)
    assert_eq!(accounts_metadata[2].0, 2); // index
    assert_eq!(accounts_metadata[2].1, "mutable_signer"); // name
    assert_eq!(accounts_metadata[2].2, true); // writable
    assert_eq!(accounts_metadata[2].3, true); // signer
    assert_eq!(accounts_metadata[2].4, false); // optional_signer
    assert_eq!(accounts_metadata[2].5, false); // optional
    
    // Check fourth account (optional_account)
    assert_eq!(accounts_metadata[3].0, 3); // index
    assert_eq!(accounts_metadata[3].1, "optional_account"); // name
    assert_eq!(accounts_metadata[3].2, false); // writable
    assert_eq!(accounts_metadata[3].3, false); // signer
    assert_eq!(accounts_metadata[3].4, false); // optional_signer
    assert_eq!(accounts_metadata[3].5, true); // optional
    
    // Check fifth account (regular_account)
    assert_eq!(accounts_metadata[4].0, 4); // index
    assert_eq!(accounts_metadata[4].1, "regular_account"); // name
    assert_eq!(accounts_metadata[4].2, false); // writable
    assert_eq!(accounts_metadata[4].3, false); // signer
    assert_eq!(accounts_metadata[4].4, false); // optional_signer
    assert_eq!(accounts_metadata[4].5, false); // optional
}

#[test]
fn test_all_anchor_constraint_combinations() {
    #[derive(ShankAccounts)]
    pub struct AnchorStyleAccounts<'info> {
        // Basic constraints
        #[account(mut)]
        pub mut_only: AccountInfo<'info>,
        
        #[account(signer)]
        pub signer_only: AccountInfo<'info>,
        
        #[account(mut, signer)]
        pub mut_and_signer: AccountInfo<'info>,
        
        // With descriptions (Anchor style)
        #[account(mut, desc = "Mutable account with description")]
        pub mut_with_desc: AccountInfo<'info>,
        
        #[account(signer, desc = "Signer with description")]
        pub signer_with_desc: AccountInfo<'info>,
        
        // Optional account (shank extension)
        #[account(optional)]
        pub optional_account: AccountInfo<'info>,
        
        #[account(optional_signer)]
        pub optional_signer: AccountInfo<'info>,
        
        // No constraints (just regular account)
        pub no_constraints: AccountInfo<'info>,
    }

    // Test that all combinations compile and generate correct metadata
    let metadata = AnchorStyleAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 8);
    
    // Verify each account's constraints
    assert_eq!(metadata[0].1, "mut_only");
    assert_eq!(metadata[0].2, true);  // writable
    assert_eq!(metadata[0].3, false); // signer
    
    assert_eq!(metadata[1].1, "signer_only");
    assert_eq!(metadata[1].2, false); // writable
    assert_eq!(metadata[1].3, true);  // signer
    
    assert_eq!(metadata[2].1, "mut_and_signer");
    assert_eq!(metadata[2].2, true);  // writable
    assert_eq!(metadata[2].3, true);  // signer
    
    assert_eq!(metadata[5].1, "optional_account");
    assert_eq!(metadata[5].5, true);  // optional
    
    assert_eq!(metadata[6].1, "optional_signer");
    assert_eq!(metadata[6].4, true);  // optional_signer
}

#[test]
fn test_backward_compatible_constraint_names() {
    // Test that we still support shank's original constraint names
    #[derive(ShankAccounts)]
    pub struct BackwardCompatAccounts<'info> {
        #[account(writable)]
        pub writable_account: AccountInfo<'info>,
        
        #[account(write)]
        pub write_account: AccountInfo<'info>,
        
        #[account(writ)]
        pub writ_account: AccountInfo<'info>,
        
        #[account(w)]
        pub w_account: AccountInfo<'info>,
        
        #[account(sign)]
        pub sign_account: AccountInfo<'info>,
        
        #[account(sig)]
        pub sig_account: AccountInfo<'info>,
        
        #[account(s)]
        pub s_account: AccountInfo<'info>,
        
        #[account(optional)]
        pub opt_account: AccountInfo<'info>,
        
        #[account(option)]
        pub option_account: AccountInfo<'info>,
    }

    let metadata = BackwardCompatAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 9);
    
    // All writable variants should be marked as writable
    assert_eq!(metadata[0].2, true); // writable
    assert_eq!(metadata[1].2, true); // write
    assert_eq!(metadata[2].2, true); // writ
    assert_eq!(metadata[3].2, true); // w
    
    // All signer variants should be marked as signer
    assert_eq!(metadata[4].3, true); // sign
    assert_eq!(metadata[5].3, true); // sig  
    assert_eq!(metadata[6].3, true); // s
    
    // All optional variants should be marked as optional
    assert_eq!(metadata[7].5, true); // optional
    assert_eq!(metadata[8].5, true); // option
}

#[test]
fn test_empty_accounts_struct() {
    #[derive(ShankAccounts)]
    pub struct EmptyAccounts {
        // Empty struct to test edge case - no lifetime needed when no fields
    }

    let metadata = EmptyAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 0);
}

#[test]
fn test_single_account_struct() {
    #[derive(ShankAccounts)]
    pub struct SingleAccount<'info> {
        #[account(mut, signer, desc = "The only account")]
        pub only_account: AccountInfo<'info>,
    }

    let metadata = SingleAccount::__shank_accounts();
    assert_eq!(metadata.len(), 1);
    assert_eq!(metadata[0].1, "only_account");
    assert_eq!(metadata[0].2, true);  // writable
    assert_eq!(metadata[0].3, true);  // signer
    assert_eq!(metadata[0].6, Some("The only account".to_string())); // description
}

#[test] 
fn test_generic_lifetimes_work() {
    // Test that our macro correctly handles generic lifetime parameters like Anchor
    #[derive(ShankAccounts)]
    pub struct GenericAccounts<'info> {
        pub account1: AccountInfo<'info>,
        pub account2: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct DifferentLifetime<'a> {
        pub account: AccountInfo<'a>,
    }
    
    #[derive(ShankAccounts)]  
    pub struct MultipleGenerics<'info> {
        pub info_account: AccountInfo<'info>,
        pub data_account: AccountInfo<'info>, 
    }
    
    // All should compile successfully
    assert_eq!(GenericAccounts::__shank_accounts().len(), 2);
    assert_eq!(DifferentLifetime::__shank_accounts().len(), 1);
    assert_eq!(MultipleGenerics::__shank_accounts().len(), 2);
}