//! Example showing how to use the context() method with ShankAccounts

use shank::ShankAccounts;

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

// The context() method is only available when the solana-program feature is enabled
#[cfg(feature = "solana-program")]
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

// Mock AccountInfo when solana-program feature is not enabled
#[cfg(not(feature = "solana-program"))]
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[derive(ShankAccounts)]
pub struct CreateTokenAccounts<'info> {
    #[account(mut, signer, desc = "Payer and authority")]
    pub payer: &'info AccountInfo<'info>,

    #[account(mut, desc = "Token mint account")]
    pub mint: &'info AccountInfo<'info>,

    #[account(desc = "System program")]
    pub system_program: &'info AccountInfo<'info>,

    #[account(optional, desc = "Optional metadata account")]
    pub metadata: Option<&'info AccountInfo<'info>>,
}

#[cfg(feature = "solana-program")]
pub fn process_create_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Use the generated context method to parse accounts
    let ctx = CreateTokenAccounts::context(accounts, program_id)?;

    // Now you can access accounts with type safety and validation
    msg!("Payer: {:?}", ctx.payer.key);
    msg!("Mint: {:?}", ctx.mint.key);
    msg!("System program: {:?}", ctx.system_program.key);

    // Handle optional accounts safely
    if let Some(metadata) = ctx.metadata {
        msg!("Metadata provided: {:?}", metadata.key);
    } else {
        msg!("No metadata account provided");
    }

    // Your program logic here...

    Ok(())
}

#[cfg(feature = "solana-program")]
pub fn process_instruction_with_error_handling(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    match CreateTokenAccounts::context(accounts, program_id) {
        Ok(ctx) => {
            // Successfully parsed accounts
            msg!("Successfully parsed {} accounts", 4);

            // Your program logic here...

            Ok(())
        }
        Err(ProgramError::NotEnoughAccountKeys) => {
            msg!("Error: Not enough accounts provided");
            Err(ProgramError::NotEnoughAccountKeys)
        }
        Err(ProgramError::InvalidAccountData) => {
            msg!("Error: Too many accounts provided");
            Err(ProgramError::InvalidAccountData)
        }
        Err(e) => {
            msg!("Error parsing accounts: {:?}", e);
            Err(e)
        }
    }
}

#[cfg(not(feature = "solana-program"))]
fn main() {
    println!("Context usage example for ShankAccounts!");
    println!();
    println!("Key features:");
    println!("1. Type-safe account access with ctx.payer, ctx.mint, etc.");
    println!("2. Automatic validation of required vs optional accounts");
    println!("3. Proper error handling for missing or extra accounts");
    println!("4. Integration with Solana program entry points");
    println!();
    println!("Usage in your Solana program:");
    println!(
        "  let ctx = CreateTokenAccounts::context(accounts, program_id)?;"
    );
    println!("  // Now access accounts safely: ctx.payer.key, ctx.mint, etc.");
    println!();
    println!("The context() method provides these benefits:");
    println!("- Validates minimum required account count");
    println!("- Handles optional accounts correctly");
    println!("- Returns structured access to all accounts");
    println!("- Integrates with existing Solana program patterns");
}

#[cfg(feature = "solana-program")]
fn main() {
    println!("Context usage example with full Solana program support!");
    println!(
        "Use the process_create_token() function as your instruction handler."
    );
}
