use shank::{Context, ShankAccounts};

// Mock AccountInfo
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[derive(ShankAccounts)]
pub struct BasicTestAccounts<'info> {
    #[account(mut, signer)]
    pub payer: &'info AccountInfo<'info>,

    #[account(mut)]
    pub data: &'info AccountInfo<'info>,
}

#[test]
fn test_basic_context() {
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

    let accounts = [payer, data];

    let ctx: Context<BasicTestAccounts, AccountInfo> =
        BasicTestAccounts::context(&accounts);

    // This should compile and work
    assert_eq!(ctx.accounts.payer.key, &payer_key);
    assert_eq!(ctx.accounts.data.key, &data_key);
}

fn main() {
    println!("Basic context test");
}
