use shank::ShankAccounts;

// Mock AccountInfo for testing (in real programs, import from solana_program)
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32], // Mock pubkey
    pub data: &'info [u8],
    pub owner: &'info [u8; 32], // Mock pubkey
}

// Test basic ShankAccounts struct compilation
#[test]
fn test_basic_accounts_struct_compiles() {
    #[derive(ShankAccounts)]
    pub struct CreateVaultAccounts<'info> {
        #[account(mut, desc = "Vault account")]
        pub vault: &'info AccountInfo<'info>,

        #[account(signer, desc = "Authority")]
        pub authority: &'info AccountInfo<'info>,

        #[account(desc = "System program")]
        pub system_program: &'info AccountInfo<'info>,
    }

    // The macro generates both IDL metadata and context structs
    // This test ensures the macro compiles successfully with AccountInfo types
}

// Test all account attributes compile
#[test]
fn test_all_account_attributes_compile() {
    #[derive(ShankAccounts)]
    pub struct ComplexAccounts<'info> {
        #[account(mut, signer, desc = "Payer account")]
        pub payer: AccountInfo<'info>,

        #[account(optional, desc = "Optional account")]
        pub optional_account: AccountInfo<'info>,

        #[account(optional_signer, desc = "Optional signer")]
        pub optional_signer: AccountInfo<'info>,

        #[account(writable)]
        pub data_account: AccountInfo<'info>,
    }
}

// Test alternative attribute names compile
#[test]
fn test_alternative_attribute_names_compile() {
    #[derive(ShankAccounts)]
    pub struct AlternativeAccounts<'info> {
        #[account(write)]
        pub writable1: AccountInfo<'info>,

        #[account(writ)]
        pub writable2: AccountInfo<'info>,

        #[account(w)]
        pub writable3: AccountInfo<'info>,

        #[account(sign)]
        pub signer1: AccountInfo<'info>,

        #[account(sig)]
        pub signer2: AccountInfo<'info>,

        #[account(s)]
        pub signer3: AccountInfo<'info>,

        #[account(opt)]
        pub optional1: AccountInfo<'info>,

        #[account(option)]
        pub optional2: AccountInfo<'info>,
    }
}
