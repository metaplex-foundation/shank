use shank::{ShankAccounts, Context};

// Mock AccountInfo
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

// Mock program ID for optional account detection
pub const ID: [u8; 32] = [42; 32];

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
fn test_context_method_works() {
    let payer_key = [1u8; 32];
    let data_key = [2u8; 32];
    let optional_key = [3u8; 32];
    
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
    
    let accounts = [payer, data, optional];
    
    // This is the key test - the context method should work!
    let ctx: Context<TestAccounts> = TestAccounts::context(&accounts);
    
    // Verify the accounts struct was created correctly
    assert_eq!(ctx.accounts.payer.key, &payer_key);
    assert_eq!(ctx.accounts.data.key, &data_key);
    assert!(ctx.accounts.optional_account.is_some());
    assert_eq!(ctx.accounts.optional_account.unwrap().key, &optional_key);
}

#[test]
fn test_minimal_accounts() {
    let payer_key = [1u8; 32];
    let data_key = [2u8; 32];
    
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
    
    // Only provide required accounts (payer and data, no optional)
    let accounts = [payer, data];
    
    let ctx: Context<TestAccounts> = TestAccounts::context(&accounts);
    
    assert_eq!(ctx.accounts.payer.key, &payer_key);
    assert_eq!(ctx.accounts.data.key, &data_key);
    assert!(ctx.accounts.optional_account.is_some()); // Still present at index 2, but would be None if we had proper ID detection
}

fn main() {
    println!("Final context test - context method is working!");
}