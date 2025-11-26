use shank::ShankAccounts;

// Mock AccountInfo for testing without solana-program
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

pub const ID: [u8; 32] = [1; 32];

#[test]
fn test_context_method_basic() {
    #[derive(ShankAccounts)]
    pub struct TestAccounts<'info> {
        #[account(mut, signer)]
        pub payer: &'info AccountInfo<'info>,

        #[account(mut)]
        pub data: &'info AccountInfo<'info>,

        pub system_program: &'info AccountInfo<'info>,
    }

    // This should compile but won't work without solana-program feature
    // Just testing that the macro expands correctly
    let idl = TestAccounts::__shank_accounts();
    assert_eq!(idl.len(), 3);
}

#[test]
fn test_context_method_with_optional() {
    #[derive(ShankAccounts)]
    pub struct TestOptionalAccounts<'info> {
        #[account(signer)]
        pub authority: &'info AccountInfo<'info>,

        #[account(optional)]
        pub optional_data: Option<&'info AccountInfo<'info>>,

        pub system_program: &'info AccountInfo<'info>,
    }

    let idl = TestOptionalAccounts::__shank_accounts();
    assert_eq!(idl.len(), 3);

    // Check that optional field is marked correctly
    assert_eq!(idl[1].5, true); // optional
}
