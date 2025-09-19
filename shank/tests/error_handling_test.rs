use shank::ShankAccounts;

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8], 
    pub owner: &'info [u8; 32],
}

#[test]
fn test_conflicting_constraints_compile_time_error() {
    // Note: These would cause compile-time errors, but we can't easily test 
    // compile-time failures in unit tests. However, we can document the expected behavior.
    
    // This SHOULD cause an error (but we can't test it in a unit test):
    // #[derive(ShankAccounts)]
    // pub struct ConflictingAccounts<'info> {
    //     #[account(signer, optional_signer)] // Should error: can't be both
    //     pub bad_account: AccountInfo<'info>,
    // }
    
    // Instead, we test valid non-conflicting combinations
    #[derive(ShankAccounts)]
    pub struct ValidAccounts<'info> {
        #[account(signer)]
        pub signer_account: AccountInfo<'info>,
        
        #[account(optional_signer)]  
        pub optional_signer_account: AccountInfo<'info>,
        
        #[account(mut, signer)] // This combination is valid
        pub mut_signer: AccountInfo<'info>,
    }
    
    let metadata = ValidAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 3);
    
    // Verify signer
    assert_eq!(metadata[0].3, true);  // signer
    assert_eq!(metadata[0].4, false); // not optional_signer
    
    // Verify optional_signer  
    assert_eq!(metadata[1].3, false); // not regular signer
    assert_eq!(metadata[1].4, true);  // optional_signer
    
    // Verify combined constraints work
    assert_eq!(metadata[2].2, true);  // writable
    assert_eq!(metadata[2].3, true);  // signer
}

#[test]
fn test_description_variants() {
    #[derive(ShankAccounts)]
    pub struct DescriptionVariants<'info> {
        #[account(desc = "Using desc")]
        pub desc_account: AccountInfo<'info>,
        
        // Test that we properly handle all description attribute names
        // (This tests our parsing logic)
        pub no_desc_account: AccountInfo<'info>,
    }
    
    let metadata = DescriptionVariants::__shank_accounts();
    assert_eq!(metadata.len(), 2);
    
    // Check description is captured
    assert_eq!(metadata[0].6, Some("Using desc".to_string()));
    
    // Check no description
    assert_eq!(metadata[1].6, None);
}

#[test]
fn test_field_order_preservation() {
    #[derive(ShankAccounts)]
    pub struct OrderedAccounts<'info> {
        pub first: AccountInfo<'info>,
        pub second: AccountInfo<'info>, 
        pub third: AccountInfo<'info>,
        pub fourth: AccountInfo<'info>,
    }
    
    let metadata = OrderedAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 4);
    
    // Verify order is preserved
    assert_eq!(metadata[0].0, 0);
    assert_eq!(metadata[0].1, "first");
    
    assert_eq!(metadata[1].0, 1); 
    assert_eq!(metadata[1].1, "second");
    
    assert_eq!(metadata[2].0, 2);
    assert_eq!(metadata[2].1, "third");
    
    assert_eq!(metadata[3].0, 3);
    assert_eq!(metadata[3].1, "fourth");
}

#[test]
fn test_complex_constraint_combinations() {
    #[derive(ShankAccounts)]
    pub struct ComplexAccounts<'info> {
        // All basic combinations that make sense
        #[account(mut)]
        pub mut_only: AccountInfo<'info>,
        
        #[account(signer)]
        pub signer_only: AccountInfo<'info>,
        
        #[account(optional)]
        pub optional_only: AccountInfo<'info>,
        
        #[account(optional_signer)]
        pub optional_signer_only: AccountInfo<'info>,
        
        #[account(mut, signer)]
        pub mut_signer: AccountInfo<'info>,
        
        #[account(mut, optional)]
        pub mut_optional: AccountInfo<'info>,
        
        #[account(mut, signer, desc = "Complex account")]
        pub mut_signer_desc: AccountInfo<'info>,
    }
    
    let metadata = ComplexAccounts::__shank_accounts();
    assert_eq!(metadata.len(), 7);
    
    // Test mut_only
    assert!(metadata[0].2);   // writable
    assert!(!metadata[0].3);  // not signer
    assert!(!metadata[0].4);  // not optional_signer
    assert!(!metadata[0].5);  // not optional
    
    // Test signer_only
    assert!(!metadata[1].2);  // not writable
    assert!(metadata[1].3);   // signer
    assert!(!metadata[1].4);  // not optional_signer
    assert!(!metadata[1].5);  // not optional
    
    // Test optional_only
    assert!(!metadata[2].2);  // not writable
    assert!(!metadata[2].3);  // not signer
    assert!(!metadata[2].4);  // not optional_signer
    assert!(metadata[2].5);   // optional
    
    // Test optional_signer_only
    assert!(!metadata[3].2);  // not writable
    assert!(!metadata[3].3);  // not regular signer
    assert!(metadata[3].4);   // optional_signer
    assert!(!metadata[3].5);  // not optional (optional_signer is different)
    
    // Test mut_signer combination
    assert!(metadata[4].2);   // writable
    assert!(metadata[4].3);   // signer
    assert!(!metadata[4].4);  // not optional_signer
    assert!(!metadata[4].5);  // not optional
    
    // Test mut_optional combination
    assert!(metadata[5].2);   // writable
    assert!(!metadata[5].3);  // not signer
    assert!(!metadata[5].4);  // not optional_signer
    assert!(metadata[5].5);   // optional
    
    // Test all combined with description
    assert!(metadata[6].2);   // writable
    assert!(metadata[6].3);   // signer
    assert!(!metadata[6].4);  // not optional_signer
    assert!(!metadata[6].5);  // not optional
    assert_eq!(metadata[6].6, Some("Complex account".to_string()));
}

#[test]
fn test_different_lifetime_names() {
    // Test that our macro works with different lifetime parameter names
    #[derive(ShankAccounts)]
    pub struct InfoLifetime<'info> {
        pub account: AccountInfo<'info>,
    }
    
    #[derive(ShankAccounts)]
    pub struct ALifetime<'a> {
        pub account: AccountInfo<'a>,
    }
    
    #[derive(ShankAccounts)]
    pub struct CustomLifetime<'my_lifetime> {
        pub account: AccountInfo<'my_lifetime>,
    }
    
    // All should compile and work
    assert_eq!(InfoLifetime::__shank_accounts().len(), 1);
    assert_eq!(ALifetime::__shank_accounts().len(), 1);
    assert_eq!(CustomLifetime::__shank_accounts().len(), 1);
}

#[test]
fn test_underscore_and_special_field_names() {
    #[derive(ShankAccounts)]
    pub struct SpecialNames<'info> {
        pub _underscore: AccountInfo<'info>,
        pub camelCase: AccountInfo<'info>,
        pub snake_case: AccountInfo<'info>,
        pub UPPER_CASE: AccountInfo<'info>,
        pub account123: AccountInfo<'info>,
    }
    
    let metadata = SpecialNames::__shank_accounts();
    assert_eq!(metadata.len(), 5);
    
    assert_eq!(metadata[0].1, "_underscore");
    assert_eq!(metadata[1].1, "camelCase");
    assert_eq!(metadata[2].1, "snake_case");
    assert_eq!(metadata[3].1, "UPPER_CASE");
    assert_eq!(metadata[4].1, "account123");
}

#[test]
fn test_long_descriptions() {
    #[derive(ShankAccounts)]
    pub struct LongDescriptions<'info> {
        #[account(desc = "This is a very long description that goes on and on and explains exactly what this account is used for in great detail with multiple sentences and lots of information.")]
        pub detailed_account: AccountInfo<'info>,
        
        #[account(desc = "")]
        pub empty_desc: AccountInfo<'info>,
        
        #[account(desc = "Short")]
        pub short_desc: AccountInfo<'info>,
    }
    
    let metadata = LongDescriptions::__shank_accounts();
    assert_eq!(metadata.len(), 3);
    
    // Verify long description is preserved
    assert!(metadata[0].6.as_ref().unwrap().len() > 100);
    assert!(metadata[0].6.as_ref().unwrap().contains("very long description"));
    
    // Verify empty description
    assert_eq!(metadata[1].6, Some("".to_string()));
    
    // Verify short description  
    assert_eq!(metadata[2].6, Some("Short".to_string()));
}