use shank::ShankAccounts;

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

// This test shows the current state and the issue you're encountering

#[cfg(feature = "solana-program")]
mod with_solana_program {
    use super::*;
    use solana_program::{
        account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
    };

    #[derive(ShankAccounts)]
    pub struct TestAccounts<'info> {
        #[account(mut, signer)]
        pub payer: &'info AccountInfo<'info>,

        #[account(optional)]
        pub optional_account: Option<&'info AccountInfo<'info>>,
    }

    #[test]
    fn test_context_method_exists() {
        // This should compile when solana-program feature is enabled
        let accounts = vec![];
        let program_id = Pubkey::new_unique();

        // The context method should be available
        let result = TestAccounts::context(&accounts, &program_id);
        assert!(result.is_err());
    }
}

#[cfg(not(feature = "solana-program"))]
mod without_solana_program {
    use super::*;

    // Mock AccountInfo for testing
    pub struct AccountInfo<'info> {
        pub key: &'info [u8; 32],
        pub data: &'info [u8],
        pub owner: &'info [u8; 32],
    }

    #[derive(ShankAccounts)]
    pub struct TestAccounts<'info> {
        #[account(mut, signer)]
        pub payer: &'info AccountInfo<'info>,

        #[account(optional)]
        pub optional_account: Option<&'info AccountInfo<'info>>,
    }

    #[test]
    fn test_context_method_not_available() {
        // Currently the context() method is not available without solana-program feature
        // This is what you want to change

        let _accounts = TestAccounts::__shank_accounts();
        // TestAccounts::context() is not available here

        // This is the limitation you want to remove
        println!("Context method not available without solana-program feature");
    }
}

fn main() {
    println!("Testing context method availability");
}
