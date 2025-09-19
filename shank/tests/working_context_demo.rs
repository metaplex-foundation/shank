use shank::{ShankAccounts, Context};

// Mock AccountInfo for demo
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

#[derive(ShankAccounts)]
pub struct DemoAccounts<'info> {
    #[account(mut, signer, desc = "Payer account")]
    pub payer: &'info AccountInfo<'info>,
    
    #[account(mut, desc = "Data account")]
    pub data: &'info AccountInfo<'info>,
}

fn main() {
    println!("Working context demo");
    println!("The context() method should be available on DemoAccounts");
    println!("It should return Context<DemoAccounts> with accounts and remaining_accounts fields");
    
    // For now, let's just test that the IDL generation works
    let accounts_idl = DemoAccounts::__shank_accounts();
    println!("Generated {} accounts in IDL", accounts_idl.len());
    
    // In a real program, this would work:
    // let ctx = DemoAccounts::context(&accounts);
    // ctx.accounts.payer.key // access payer
    // ctx.remaining_accounts // access any extra accounts
}