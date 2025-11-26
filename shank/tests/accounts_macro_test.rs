use shank::ShankAccounts;

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

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
        pub payer: &'info AccountInfo<'info>,

        #[account(optional, desc = "Optional account")]
        pub optional_account: Option<&'info AccountInfo<'info>>,

        #[account(optional_signer, desc = "Optional signer")]
        pub optional_signer: Option<&'info AccountInfo<'info>>,

        #[account(writable)]
        pub data_account: &'info AccountInfo<'info>,
    }
}

// Test alternative attribute names compile
#[test]
fn test_alternative_attribute_names_compile() {
    #[derive(ShankAccounts)]
    pub struct AlternativeAccounts<'info> {
        #[account(write)]
        pub writable1: &'info AccountInfo<'info>,

        #[account(writ)]
        pub writable2: &'info AccountInfo<'info>,

        #[account(w)]
        pub writable3: &'info AccountInfo<'info>,

        #[account(sign)]
        pub signer1: &'info AccountInfo<'info>,

        #[account(sig)]
        pub signer2: &'info AccountInfo<'info>,

        #[account(s)]
        pub signer3: &'info AccountInfo<'info>,

        #[account(opt)]
        pub optional1: Option<&'info AccountInfo<'info>>,

        #[account(option)]
        pub optional2: Option<&'info AccountInfo<'info>>,
    }
}
