#[cfg(feature = "solana-program")]
mod solana_program_tests {
    use shank::ShankAccounts;
    use solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey,
        program_error::ProgramError,
    };

    #[derive(ShankAccounts)]
    pub struct CreateTokenAccounts<'info> {
        #[account(mut, signer, desc = "Payer and authority")]
        pub payer: &'info AccountInfo<'info>,
        
        #[account(mut, desc = "Token mint account")]
        pub mint: &'info AccountInfo<'info>,
        
        #[account(desc = "System program")]
        pub system_program: &'info AccountInfo<'info>,
        
        #[account(optional, desc = "Optional metadata account")]
        pub metadata: Option<&'info AccountInfo<'info>>,
    }

    #[test]
    fn test_context_with_required_accounts() {
        // Mock accounts
        let payer_key = Pubkey::new_unique();
        let mint_key = Pubkey::new_unique();
        let system_program_key = solana_program::system_program::id();
        let program_id = Pubkey::new_unique();
        
        let mut payer_lamports = 1000000;
        let mut mint_lamports = 0;
        let mut system_lamports = 1;
        
        let payer_data = vec![];
        let mint_data = vec![];
        let system_data = vec![];
        
        let payer_info = AccountInfo::new(
            &payer_key,
            true, // is_signer
            true, // is_writable  
            &mut payer_lamports,
            &mut payer_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let mint_info = AccountInfo::new(
            &mint_key,
            false,
            true, // is_writable
            &mut mint_lamports,
            &mut mint_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let system_info = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data.clone(),
            &system_program_key, // owned by itself
            false,
            0,
        );
        
        let accounts = &[payer_info, mint_info, system_info];
        
        // Test context creation with minimum required accounts
        let ctx = CreateTokenAccounts::context(accounts, &program_id).unwrap();
        
        assert_eq!(ctx.payer.key, &payer_key);
        assert_eq!(ctx.mint.key, &mint_key);
        assert_eq!(ctx.system_program.key, &system_program_key);
        assert!(ctx.metadata.is_none());
    }

    #[test]
    fn test_context_with_optional_account() {
        // Mock accounts including optional account
        let payer_key = Pubkey::new_unique();
        let mint_key = Pubkey::new_unique();
        let system_program_key = solana_program::system_program::id();
        let metadata_key = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();
        
        let mut payer_lamports = 1000000;
        let mut mint_lamports = 0;
        let mut system_lamports = 1;
        let mut metadata_lamports = 0;
        
        let payer_data = vec![];
        let mint_data = vec![];
        let system_data = vec![];
        let metadata_data = vec![];
        
        let payer_info = AccountInfo::new(
            &payer_key,
            true, // is_signer
            true, // is_writable  
            &mut payer_lamports,
            &mut payer_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let mint_info = AccountInfo::new(
            &mint_key,
            false,
            true, // is_writable
            &mut mint_lamports,
            &mut mint_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let system_info = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data.clone(),
            &system_program_key, // owned by itself
            false,
            0,
        );
        
        let metadata_info = AccountInfo::new(
            &metadata_key,
            false,
            false,
            &mut metadata_lamports,
            &mut metadata_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let accounts = &[payer_info, mint_info, system_info, metadata_info];
        
        // Test context creation with optional account provided
        let ctx = CreateTokenAccounts::context(accounts, &program_id).unwrap();
        
        assert_eq!(ctx.payer.key, &payer_key);
        assert_eq!(ctx.mint.key, &mint_key);
        assert_eq!(ctx.system_program.key, &system_program_key);
        assert!(ctx.metadata.is_some());
        assert_eq!(ctx.metadata.unwrap().key, &metadata_key);
    }

    #[test]
    fn test_context_with_program_id_placeholder() {
        // Test when optional account is the program ID (should be None)
        let payer_key = Pubkey::new_unique();
        let mint_key = Pubkey::new_unique();
        let system_program_key = solana_program::system_program::id();
        let program_id = Pubkey::new_unique();
        
        let mut payer_lamports = 1000000;
        let mut mint_lamports = 0;
        let mut system_lamports = 1;
        let mut program_lamports = 1;
        
        let payer_data = vec![];
        let mint_data = vec![];
        let system_data = vec![];
        let program_data = vec![];
        
        let payer_info = AccountInfo::new(
            &payer_key,
            true,
            true,
            &mut payer_lamports,
            &mut payer_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let mint_info = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut mint_lamports,
            &mut mint_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let system_info = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data.clone(),
            &system_program_key,
            false,
            0,
        );
        
        // Use program_id as placeholder for optional account
        let program_info = AccountInfo::new(
            &program_id, // This should make metadata None
            false,
            false,
            &mut program_lamports,
            &mut program_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let accounts = &[payer_info, mint_info, system_info, program_info];
        
        let ctx = CreateTokenAccounts::context(accounts, &program_id).unwrap();
        
        assert_eq!(ctx.payer.key, &payer_key);
        assert_eq!(ctx.mint.key, &mint_key);
        assert_eq!(ctx.system_program.key, &system_program_key);
        assert!(ctx.metadata.is_none()); // Should be None because key == program_id
    }

    #[test]
    fn test_context_error_not_enough_accounts() {
        let program_id = Pubkey::new_unique();
        
        // Only provide 2 accounts when 3 are required
        let payer_key = Pubkey::new_unique();
        let mint_key = Pubkey::new_unique();
        
        let mut payer_lamports = 1000000;
        let mut mint_lamports = 0;
        
        let payer_data = vec![];
        let mint_data = vec![];
        
        let payer_info = AccountInfo::new(
            &payer_key,
            true,
            true,
            &mut payer_lamports,
            &mut payer_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let mint_info = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut mint_lamports,
            &mut mint_data.clone(),
            &program_id,
            false,
            0,
        );
        
        let accounts = &[payer_info, mint_info];
        
        // Should fail because we need 3 required accounts but only provided 2
        let result = CreateTokenAccounts::context(accounts, &program_id);
        assert!(matches!(result, Err(ProgramError::NotEnoughAccountKeys)));
    }
}

// Mock tests without solana-program feature
#[cfg(not(feature = "solana-program"))]
mod mock_tests {
    use shank::ShankAccounts;

    // Mock AccountInfo for testing
    pub struct AccountInfo<'info> {
        pub key: &'info [u8; 32],
        pub data: &'info [u8],
        pub owner: &'info [u8; 32],
    }

    #[test]
    fn test_idl_generation_without_solana_program() {
        #[derive(ShankAccounts)]
        pub struct TestAccounts<'info> {
            #[account(mut, signer, desc = "Payer")]
            pub payer: &'info AccountInfo<'info>,
            
            #[account(optional, desc = "Optional account")]
            pub optional: Option<&'info AccountInfo<'info>>,
        }

        let idl = TestAccounts::__shank_accounts();
        assert_eq!(idl.len(), 2);
        assert_eq!(idl[0].1, "payer");
        assert_eq!(idl[1].1, "optional");
        assert_eq!(idl[1].5, true); // optional
    }
}