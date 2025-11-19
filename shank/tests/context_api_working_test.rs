use shank::{AccountInfoRef, Context, ShankAccounts};

// Mock program ID - this simulates what declare_id! macro would create
pub const ID: [u8; 32] = [1; 32];

// Mock AccountInfo that implements AccountInfoRef
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

impl AccountInfoRef for AccountInfo<'_> {
    fn key(&self) -> &[u8; 32] {
        self.key
    }
}

#[derive(ShankAccounts)]
pub struct TestAccounts<'info> {
    #[account(mut, signer, desc = "The payer")]
    pub payer: &'info AccountInfo<'info>,

    #[account(mut, desc = "Data account")]
    pub data: &'info AccountInfo<'info>,

    #[account(optional, desc = "Optional account")]
    pub optional_account: Option<&'info AccountInfo<'info>>,
}

#[test]
fn test_context_api_works() {
    let payer_key = [2u8; 32];
    let data_key = [3u8; 32];
    let optional_key = [4u8; 32];
    let remaining_key = [5u8; 32];

    let payer = AccountInfo {
        key: &payer_key,
        data: &[],
        owner: &[0; 32],
    };

    let data = AccountInfo {
        key: &data_key,
        data: &[],
        owner: &[0; 32],
    };

    let optional = AccountInfo {
        key: &optional_key,
        data: &[],
        owner: &[0; 32],
    };

    let remaining = AccountInfo {
        key: &remaining_key,
        data: &[],
        owner: &[0; 32],
    };

    // Test with all accounts including optional and remaining
    let accounts = [payer, data, optional, remaining];

    let ctx: Context<TestAccounts, AccountInfo> =
        TestAccounts::context(&accounts);

    // Verify accounts struct
    assert_eq!(ctx.accounts.payer.key, &payer_key);
    assert_eq!(ctx.accounts.data.key, &data_key);
    assert!(ctx.accounts.optional_account.is_some());
    assert_eq!(ctx.accounts.optional_account.unwrap().key, &optional_key);

    // Verify remaining accounts
    assert_eq!(ctx.remaining_accounts.len(), 1);
    assert_eq!(ctx.remaining_accounts[0].key, &remaining_key);
}

#[test]
fn test_context_without_optional() {
    let payer_key = [2u8; 32];
    let data_key = [3u8; 32];

    let payer = AccountInfo {
        key: &payer_key,
        data: &[],
        owner: &[0; 32],
    };

    let data = AccountInfo {
        key: &data_key,
        data: &[],
        owner: &[0; 32],
    };

    // Use program ID as placeholder for optional account
    let optional_placeholder = AccountInfo {
        key: &ID, // This should make optional_account None
        data: &[],
        owner: &[0; 32],
    };

    let accounts = [payer, data, optional_placeholder];

    let ctx: Context<TestAccounts, AccountInfo> =
        TestAccounts::context(&accounts);

    // Verify accounts struct
    assert_eq!(ctx.accounts.payer.key, &payer_key);
    assert_eq!(ctx.accounts.data.key, &data_key);
    assert!(ctx.accounts.optional_account.is_none()); // Should be None because key == ID

    // No remaining accounts
    assert_eq!(ctx.remaining_accounts.len(), 0);
}

fn main() {
    println!("Context API working test");
}
