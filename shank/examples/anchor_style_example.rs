//! Example showing ShankAccounts with proper Anchor-style account references

use shank::{ShankAccounts, ShankInstruction};

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

// Enable the solana-program feature for this example
#[cfg(feature = "solana-program")]
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Mock AccountInfo when solana-program feature is not enabled
#[cfg(not(feature = "solana-program"))]
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

// CORRECT: Using references to AccountInfo like Anchor
#[derive(ShankAccounts)]
pub struct CreateMachineV1Accounts<'info> {
    #[account(mut, signer, desc = "The new machine asset account")]
    pub machine: &'info AccountInfo<'info>,

    #[account(mut, desc = "The Core machine collection")]
    pub machine_collection: &'info AccountInfo<'info>,

    #[account(desc = "The account paying for the storage fees")]
    pub owner: &'info AccountInfo<'info>,

    #[account(mut, signer, desc = "The account paying for the storage fees")]
    pub payer: &'info AccountInfo<'info>,

    #[account(
        optional,
        signer,
        desc = "The authority signing for account creation"
    )]
    pub authority: Option<&'info AccountInfo<'info>>,

    #[account(desc = "The mpl core program")]
    pub mpl_core_program: &'info AccountInfo<'info>,

    #[account(desc = "The system program")]
    pub system_program: &'info AccountInfo<'info>,
}

#[derive(ShankAccounts)]
pub struct UpdateMachineV1Accounts<'info> {
    #[account(mut, desc = "The machine asset account")]
    pub machine: &'info AccountInfo<'info>,

    #[account(signer, desc = "The machine authority")]
    pub authority: &'info AccountInfo<'info>,

    #[account(optional, desc = "Optional new authority")]
    pub new_authority: Option<&'info AccountInfo<'info>>,
}

// Use the account structs in instruction definitions
#[derive(ShankInstruction)]
pub enum MachineInstruction {
    /// Create a new machine
    #[accounts(CreateMachineV1Accounts)]
    CreateMachineV1 {
        name: String,
        uri: String,
        plugins: Vec<u8>,
    },

    /// Update machine configuration
    #[accounts(UpdateMachineV1Accounts)]
    UpdateMachineV1 {
        new_name: Option<String>,
        new_uri: Option<String>,
    },
}

// Example processor functions using the generated context structs
#[cfg(feature = "solana-program")]
pub fn process_create_machine_v1<'info>(
    accounts: &'info [AccountInfo<'info>],
    name: String,
    uri: String,
    plugins: Vec<u8>,
) -> ProgramResult {
    // Use the generated context method for type-safe account access
    let ctx = CreateMachineV1Accounts::context(accounts)?;

    // Access accounts by name with compile-time guarantees
    msg!("Creating machine: {}", name);
    msg!("Machine account: {:?}", ctx.machine.key);
    msg!("Collection: {:?}", ctx.machine_collection.key);
    msg!("Owner: {:?}", ctx.owner.key);
    msg!("Payer: {:?}", ctx.payer.key);

    // Handle optional accounts safely
    match ctx.authority {
        Some(authority) => {
            msg!("Authority provided: {:?}", authority.key);
        }
        None => {
            msg!("No authority provided");
        }
    }

    // Machine creation logic would go here
    msg!("Machine '{}' created successfully", name);

    Ok(())
}

#[cfg(not(feature = "solana-program"))]
fn main() {
    println!("Anchor-style ShankAccounts example!");
    println!("This demonstrates proper usage with &'info AccountInfo<'info> references");
    println!();
    println!("Key points:");
    println!("- Account fields use &'info AccountInfo<'info> (references, like Anchor)");
    println!("- Lifetime parameter is typically 'info (Anchor convention)");
    println!("- Generated context provides type-safe access: ctx.machine.key");
    println!("- Optional accounts are handled as Option<&AccountInfo>");
    println!();
    println!("The ShankAccounts macro generates:");
    println!("1. IDL metadata via __shank_accounts() method");
    println!("2. Context struct with proper AccountInfo references");
    println!("3. Context validation methods for runtime use");
}

#[cfg(feature = "solana-program")]
fn main() {
    println!("Anchor-style ShankAccounts with full Solana program support!");
}
