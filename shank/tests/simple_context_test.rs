use shank::ShankAccounts;

// Mock program ID - this simulates what declare_id! macro would create
pub const ID: [u8; 32] = [1; 32];

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[derive(ShankAccounts)]
pub struct SimpleAccounts<'info> {
    #[account(mut, signer)]
    pub payer: &'info AccountInfo<'info>,

    #[account(mut)]
    pub data: &'info AccountInfo<'info>,

    #[account(optional)]
    pub optional_account: Option<&'info AccountInfo<'info>>,
}

#[test]
fn test_context_method() {
    // Test that the IDL generation works
    let accounts = SimpleAccounts::__shank_accounts();
    assert_eq!(accounts.len(), 3);
}

fn main() {
    println!("Simple context test");
}
