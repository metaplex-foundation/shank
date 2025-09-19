// This test file validates context struct generation with conditional compilation
// It tests both with and without the solana-program feature

use shank::ShankAccounts;

// Mock types when solana-program feature is not available
#[cfg(not(feature = "solana-program"))]
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[cfg(not(feature = "solana-program"))]
pub struct Pubkey([u8; 32]);

#[cfg(not(feature = "solana-program"))]
#[derive(Debug, PartialEq)]
pub enum ProgramError {
    NotEnoughAccountKeys,
}

// Use real types when solana-program feature is available
#[cfg(feature = "solana-program")]
use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    program_error::ProgramError,
};

#[test]
fn test_context_struct_compilation() {
    // Define accounts that should generate context structs
    #[derive(ShankAccounts)]
    pub struct TestContextAccounts<'info> {
        #[account(mut, desc = "Mutable account")]
        pub mutable_account: AccountInfo<'info>,
        
        #[account(signer, desc = "Signer account")]
        pub signer_account: AccountInfo<'info>,
        
        #[account(optional, desc = "Optional account")]
        pub optional_account: AccountInfo<'info>,
        
        pub regular_account: AccountInfo<'info>,
    }

    // Test IDL metadata generation (always available)
    let metadata = TestContextAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 4);
    
    // Test that basic metadata is correct
    assert_eq!(metadata[0].1, "mutable_account");
    assert_eq!(metadata[0].2, true);  // writable
    assert_eq!(metadata[0].3, false); // not signer
    
    assert_eq!(metadata[1].1, "signer_account");
    assert_eq!(metadata[1].2, false); // not writable
    assert_eq!(metadata[1].3, true);  // signer
    
    assert_eq!(metadata[2].1, "optional_account");
    assert_eq!(metadata[2].5, true);  // optional
}

// This test only runs when solana-program feature is enabled
#[cfg(feature = "solana-program")]
#[test]
fn test_context_struct_with_real_solana_types() {
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
    
    #[derive(ShankAccounts)]
    pub struct RealSolanaAccounts<'info> {
        #[account(mut, signer)]
        pub authority: AccountInfo<'info>,
        
        #[account(mut)]
        pub data_account: AccountInfo<'info>,
        
        #[account(optional)]
        pub optional_account: AccountInfo<'info>,
    }
    
    // Test that IDL metadata works
    let metadata = RealSolanaAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 3);
    
    // The context struct generation and methods should be available
    // but we can't easily test them without creating real AccountInfo instances
    // This test mainly verifies that the compilation works with real Solana types
}

#[test]
fn test_multiple_account_structures() {
    #[derive(ShankAccounts)]
    pub struct SimpleAccounts<'info> {
        #[account(signer)]
        pub user: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct ComplexAccounts<'info> {
        #[account(mut, signer, desc = "Complex authority")]
        pub authority: AccountInfo<'info>,
        
        #[account(mut, desc = "Data storage")]
        pub data: AccountInfo<'info>,
        
        #[account(optional_signer, desc = "Optional signer")]
        pub optional_signer: AccountInfo<'info>,
        
        #[account(optional, desc = "Optional account")]
        pub optional_account: AccountInfo<'info>,
    }
    
    // Both should generate independent metadata
    let simple_metadata = SimpleAccounts::__shank_accounts();
    let complex_metadata = ComplexAccounts::__shank_accounts();
    
    assert_eq!(simple_metadata.len(), 1);
    assert_eq!(complex_metadata.len(), 4);
    
    // Verify independence (changing one doesn't affect the other)
    assert_eq!(simple_metadata[0].1, "user");
    assert_eq!(complex_metadata[0].1, "authority");
}

#[test]
fn test_context_with_different_lifetimes() {
    // Test that different lifetime names work
    #[derive(ShankAccounts)]
    pub struct InfoLifetime<'info> {
        pub account: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct ALifetime<'a> {
        pub account: AccountInfo<'a>,
    }
    
    #[derive(ShankAccounts)]
    pub struct DataLifetime<'data> {
        pub account: AccountInfo<'data>,
    }
    
    // All should compile and generate metadata
    assert_eq!(InfoLifetime::__shank_accounts().len(), 1);
    assert_eq!(ALifetime::__shank_accounts().len(), 1);
    assert_eq!(DataLifetime::__shank_accounts().len(), 1);
}

#[test]
fn test_accounts_with_no_constraints() {
    // Test that accounts without any constraints work
    #[derive(ShankAccounts)]
    pub struct NoConstraintsAccounts<'info> {
        pub account1: AccountInfo<'info>,
        pub account2: AccountInfo<'info>,
        pub account3: AccountInfo<'info>,
    }
    
    let metadata = NoConstraintsAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 3);
    
    // All should have default constraint values
    for (i, account) in metadata.iter().enumerate() {
        assert_eq!(account.0, i as u32); // correct index
        assert_eq!(account.2, false);    // not writable
        assert_eq!(account.3, false);    // not signer
        assert_eq!(account.4, false);    // not optional_signer
        assert_eq!(account.5, false);    // not optional
        assert_eq!(account.6, None);     // no description
    }
}

#[test]
fn test_context_generation_consistency() {
    // Create the same account structure multiple times to ensure consistency
    #[derive(ShankAccounts)]
    pub struct ConsistencyTest1<'info> {
        #[account(mut, signer, desc = "Test account")]
        pub test_account: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct ConsistencyTest2<'info> {
        #[account(mut, signer, desc = "Test account")]
        pub test_account: AccountInfo<'info>,
    }
    
    let metadata1 = ConsistencyTest1::__shank_accounts();
    let metadata2 = ConsistencyTest2::__shank_accounts();
    
    // Both should generate identical metadata
    assert_eq!(metadata1.len(), metadata2.len());
    assert_eq!(metadata1[0].1, metadata2[0].1); // same name
    assert_eq!(metadata1[0].2, metadata2[0].2); // same writable
    assert_eq!(metadata1[0].3, metadata2[0].3); // same signer
    assert_eq!(metadata1[0].6, metadata2[0].6); // same description
}

// This test verifies that our macro-generated code doesn't conflict 
// with other macro-generated code in the same compilation unit
#[test]
fn test_multiple_macros_no_conflict() {
    #[derive(ShankAccounts)]
    pub struct Macro1<'info> {
        pub account: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct Macro2<'info> {
        pub account: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct Macro3<'info> {
        pub account: AccountInfo<'info>,
    }
    
    // All should be independent and not conflict
    let m1 = Macro1::__shank_accounts();
    let m2 = Macro2::__shank_accounts();
    let m3 = Macro3::__shank_accounts();
    
    assert_eq!(m1.len(), 1);
    assert_eq!(m2.len(), 1);
    assert_eq!(m3.len(), 1);
    
    // Each should have its own implementation
    assert_eq!(m1[0].1, "account");
    assert_eq!(m2[0].1, "account");
    assert_eq!(m3[0].1, "account");
}