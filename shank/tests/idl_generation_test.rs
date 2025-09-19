use shank::ShankAccounts;

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[test]
fn test_idl_generation_basic_accounts() {
    #[derive(ShankAccounts)]
    pub struct BasicAccounts<'info> {
        #[account(mut, signer, desc = "The payer account")]
        pub payer: &'info AccountInfo<'info>,
        
        #[account(mut, desc = "The data account")]
        pub data: &'info AccountInfo<'info>,
        
        #[account(desc = "The system program")]
        pub system_program: &'info AccountInfo<'info>,
    }

    let idl = BasicAccounts::__shank_accounts();
    
    // Verify we have 3 accounts
    assert_eq!(idl.len(), 3);
    
    // Check payer account
    assert_eq!(idl[0].0, 0); // index
    assert_eq!(idl[0].1, "payer"); // name
    assert_eq!(idl[0].2, true); // mut
    assert_eq!(idl[0].3, true); // signer
    assert_eq!(idl[0].4, false); // optional_signer (false for regular signer)
    assert_eq!(idl[0].5, false); // optional
    assert_eq!(idl[0].6, Some("The payer account".to_string())); // description
    
    // Check data account
    assert_eq!(idl[1].0, 1);
    assert_eq!(idl[1].1, "data");
    assert_eq!(idl[1].2, true); // mut
    assert_eq!(idl[1].3, false); // not signer
    assert_eq!(idl[1].4, false); // optional_signer
    assert_eq!(idl[1].5, false); // not optional
    assert_eq!(idl[1].6, Some("The data account".to_string()));
    
    // Check system program
    assert_eq!(idl[2].0, 2);
    assert_eq!(idl[2].1, "system_program");
    assert_eq!(idl[2].2, false); // not mut
    assert_eq!(idl[2].3, false); // not signer
    assert_eq!(idl[2].4, false); // optional_signer
    assert_eq!(idl[2].5, false); // not optional
    assert_eq!(idl[2].6, Some("The system program".to_string()));
}

#[test]
fn test_idl_generation_optional_accounts() {
    #[derive(ShankAccounts)]
    pub struct OptionalAccounts<'info> {
        #[account(signer, desc = "Required authority")]
        pub authority: &'info AccountInfo<'info>,
        
        #[account(optional, desc = "Optional data account")]
        pub optional_data: Option<&'info AccountInfo<'info>>,
        
        #[account(optional, signer, desc = "Optional authority")]
        pub optional_authority: Option<&'info AccountInfo<'info>>,
    }

    let idl = OptionalAccounts::__shank_accounts();
    
    assert_eq!(idl.len(), 3);
    
    // Required authority
    assert_eq!(idl[0].1, "authority");
    assert_eq!(idl[0].3, true); // signer
    assert_eq!(idl[0].5, false); // not optional
    
    // Optional data account
    assert_eq!(idl[1].1, "optional_data");
    assert_eq!(idl[1].3, false); // not signer
    assert_eq!(idl[1].5, true); // optional
    
    // Optional authority (optional + signer)
    assert_eq!(idl[2].1, "optional_authority");
    assert_eq!(idl[2].3, true); // signer
    assert_eq!(idl[2].4, false); // optional_signer is false (it's just optional + signer)
    assert_eq!(idl[2].5, true); // optional
}

#[test]
fn test_idl_generation_complex_constraints() {
    #[derive(ShankAccounts)]
    pub struct ComplexAccounts<'info> {
        #[account(mut, signer, desc = "Payer and authority")]
        pub payer: &'info AccountInfo<'info>,
        
        #[account(mut, desc = "Mutable data")]
        pub data: &'info AccountInfo<'info>,
        
        #[account(desc = "Read-only account")]
        pub read_only: &'info AccountInfo<'info>,
        
        #[account(optional, mut, desc = "Optional mutable account")]
        pub optional_mut: Option<&'info AccountInfo<'info>>,
        
        #[account(optional, signer, desc = "Optional signer")]
        pub optional_signer: Option<&'info AccountInfo<'info>>,
        
        #[account(optional, mut, signer, desc = "Optional mutable signer")]
        pub optional_mut_signer: Option<&'info AccountInfo<'info>>,
    }

    let idl = ComplexAccounts::__shank_accounts();
    
    assert_eq!(idl.len(), 6);
    
    // Payer (mut + signer)
    let payer = &idl[0];
    assert_eq!(payer.1, "payer");
    assert_eq!(payer.2, true); // mut
    assert_eq!(payer.3, true); // signer
    assert_eq!(payer.5, false); // not optional
    
    // Data (mut only)
    let data = &idl[1];
    assert_eq!(data.1, "data");
    assert_eq!(data.2, true); // mut
    assert_eq!(data.3, false); // not signer
    assert_eq!(data.5, false); // not optional
    
    // Read-only
    let read_only = &idl[2];
    assert_eq!(read_only.1, "read_only");
    assert_eq!(read_only.2, false); // not mut
    assert_eq!(read_only.3, false); // not signer
    assert_eq!(read_only.5, false); // not optional
    
    // Optional mut
    let optional_mut = &idl[3];
    assert_eq!(optional_mut.1, "optional_mut");
    assert_eq!(optional_mut.2, true); // mut
    assert_eq!(optional_mut.3, false); // not signer
    assert_eq!(optional_mut.5, true); // optional
    
    // Optional signer
    let optional_signer = &idl[4];
    assert_eq!(optional_signer.1, "optional_signer");
    assert_eq!(optional_signer.2, false); // not mut
    assert_eq!(optional_signer.3, true); // signer
    assert_eq!(optional_signer.5, true); // optional
    
    // Optional mut signer
    let optional_mut_signer = &idl[5];
    assert_eq!(optional_mut_signer.1, "optional_mut_signer");
    assert_eq!(optional_mut_signer.2, true); // mut
    assert_eq!(optional_mut_signer.3, true); // signer
    assert_eq!(optional_mut_signer.5, true); // optional
}

#[test]
fn test_idl_generation_no_descriptions() {
    #[derive(ShankAccounts)]
    pub struct NoDescAccounts<'info> {
        #[account(mut, signer)]
        pub payer: &'info AccountInfo<'info>,
        
        #[account]
        pub data: &'info AccountInfo<'info>,
    }

    let idl = NoDescAccounts::__shank_accounts();
    
    assert_eq!(idl.len(), 2);
    
    // Should have None for descriptions when not provided
    assert_eq!(idl[0].6, None);
    assert_eq!(idl[1].6, None);
}

#[test]
fn test_idl_generation_different_lifetimes() {
    #[derive(ShankAccounts)]
    pub struct CustomLifetime<'a> {
        #[account(signer, desc = "Authority with custom lifetime")]
        pub authority: &'a AccountInfo<'a>,
        
        #[account(mut, desc = "Data with custom lifetime")]
        pub data: &'a AccountInfo<'a>,
    }

    let idl = CustomLifetime::__shank_accounts();
    
    assert_eq!(idl.len(), 2);
    assert_eq!(idl[0].1, "authority");
    assert_eq!(idl[1].1, "data");
    
    // Descriptions should be preserved regardless of lifetime name
    assert_eq!(idl[0].6, Some("Authority with custom lifetime".to_string()));
    assert_eq!(idl[1].6, Some("Data with custom lifetime".to_string()));
}

#[test]
fn test_idl_generation_empty_struct() {
    #[derive(ShankAccounts)]
    pub struct EmptyAccounts {}

    let idl = EmptyAccounts::__shank_accounts();
    assert_eq!(idl.len(), 0);
}

#[test]
fn test_idl_generation_single_account() {
    #[derive(ShankAccounts)]
    pub struct SingleAccount<'info> {
        #[account(mut, signer, desc = "The only account")]
        pub only: &'info AccountInfo<'info>,
    }

    let idl = SingleAccount::__shank_accounts();
    
    assert_eq!(idl.len(), 1);
    assert_eq!(idl[0].0, 0); // index 0
    assert_eq!(idl[0].1, "only"); // name
    assert_eq!(idl[0].2, true); // mut
    assert_eq!(idl[0].3, true); // signer
    assert_eq!(idl[0].5, false); // not optional
    assert_eq!(idl[0].6, Some("The only account".to_string()));
}

#[test]
fn test_idl_accounts_indexing() {
    #[derive(ShankAccounts)]
    pub struct IndexedAccounts<'info> {
        #[account(desc = "First account")]
        pub first: &'info AccountInfo<'info>,
        
        #[account(desc = "Second account")]
        pub second: &'info AccountInfo<'info>,
        
        #[account(desc = "Third account")]
        pub third: &'info AccountInfo<'info>,
        
        #[account(desc = "Fourth account")]
        pub fourth: &'info AccountInfo<'info>,
    }

    let idl = IndexedAccounts::__shank_accounts();
    
    assert_eq!(idl.len(), 4);
    
    // Verify indices are sequential and start from 0
    for (expected_idx, account) in idl.iter().enumerate() {
        assert_eq!(account.0, expected_idx as u32);
    }
    
    // Verify names match field names
    assert_eq!(idl[0].1, "first");
    assert_eq!(idl[1].1, "second");
    assert_eq!(idl[2].1, "third");
    assert_eq!(idl[3].1, "fourth");
}

#[test]
fn test_idl_generation_preserves_field_order() {
    #[derive(ShankAccounts)]
    pub struct OrderedAccounts<'info> {
        #[account(desc = "Z account")]
        pub z_account: &'info AccountInfo<'info>,
        
        #[account(desc = "A account")]
        pub a_account: &'info AccountInfo<'info>,
        
        #[account(desc = "M account")]
        pub m_account: &'info AccountInfo<'info>,
    }

    let idl = OrderedAccounts::__shank_accounts();
    
    // Should preserve declaration order, not alphabetical
    assert_eq!(idl[0].1, "z_account");
    assert_eq!(idl[1].1, "a_account"); 
    assert_eq!(idl[2].1, "m_account");
    
    // Indices should match declaration order
    assert_eq!(idl[0].0, 0);
    assert_eq!(idl[1].0, 1);
    assert_eq!(idl[2].0, 2);
}