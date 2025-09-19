#[cfg(feature = "solana-program")]
mod with_solana_program {
    use shank::ShankAccounts;
    use solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey,
        program_error::ProgramError,
    };

    #[derive(ShankAccounts)]
    pub struct DemoAccounts<'info> {
        #[account(mut, signer, desc = "Payer account")]
        pub payer: &'info AccountInfo<'info>,
        
        #[account(mut, desc = "Data account")]
        pub data_account: &'info AccountInfo<'info>,
        
        #[account(desc = "System program")]
        pub system_program: &'info AccountInfo<'info>,
        
        #[account(optional, desc = "Optional metadata")]
        pub metadata: Option<&'info AccountInfo<'info>>,
    }

    // This test demonstrates how to use the context method
    // but won't actually run because creating AccountInfo is complex
    fn example_usage() {
        // In a real program, you would get accounts from the runtime:
        // 
        // pub fn process_instruction(
        //     program_id: &Pubkey,
        //     accounts: &[AccountInfo],
        //     instruction_data: &[u8],
        // ) -> ProgramResult {
        //     let ctx = DemoAccounts::context(accounts, program_id)?;
        //     
        //     // Now you can access accounts safely:
        //     msg!("Payer: {:?}", ctx.payer.key);
        //     msg!("Data: {:?}", ctx.data_account.key);
        //     msg!("System: {:?}", ctx.system_program.key);
        //     
        //     if let Some(metadata) = ctx.metadata {
        //         msg!("Metadata: {:?}", metadata.key);
        //     }
        //     
        //     Ok(())
        // }
    }

    #[test]
    fn test_idl_generation() {
        let accounts = DemoAccounts::__shank_accounts();
        assert_eq!(accounts.len(), 4);
        
        // Check payer
        assert_eq!(accounts[0].1, "payer");
        assert_eq!(accounts[0].2, true); // mut
        assert_eq!(accounts[0].3, true); // signer
        assert_eq!(accounts[0].5, false); // not optional
        
        // Check optional metadata
        assert_eq!(accounts[3].1, "metadata");
        assert_eq!(accounts[3].5, true); // optional
    }
}

#[cfg(not(feature = "solana-program"))]
mod without_solana_program {
    use shank::ShankAccounts;

    // Mock AccountInfo when solana-program is not available
    pub struct AccountInfo<'info> {
        pub key: &'info [u8; 32],
        pub data: &'info [u8],
        pub owner: &'info [u8; 32],
    }

    #[derive(ShankAccounts)]
    pub struct DemoAccounts<'info> {
        #[account(mut, signer, desc = "Payer account")]
        pub payer: &'info AccountInfo<'info>,
        
        #[account(mut, desc = "Data account")]
        pub data_account: &'info AccountInfo<'info>,
        
        #[account(desc = "System program")]
        pub system_program: &'info AccountInfo<'info>,
        
        #[account(optional, desc = "Optional metadata")]
        pub metadata: Option<&'info AccountInfo<'info>>,
    }

    #[test]
    fn test_idl_generation() {
        let accounts = DemoAccounts::__shank_accounts();
        assert_eq!(accounts.len(), 4);
        
        // Check payer
        assert_eq!(accounts[0].1, "payer");
        assert_eq!(accounts[0].2, true); // mut
        assert_eq!(accounts[0].3, true); // signer
        assert_eq!(accounts[0].5, false); // not optional
        
        // Check optional metadata  
        assert_eq!(accounts[3].1, "metadata");
        assert_eq!(accounts[3].5, true); // optional
    }
}