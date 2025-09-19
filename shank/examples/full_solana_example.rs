//! Complete example showing ShankAccounts in a real Solana program context

use shank::{ShankAccounts, ShankInstruction};

// Enable the solana-program feature for this example
#[cfg(feature = "solana-program")]
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Mock AccountInfo when solana-program feature is not enabled
#[cfg(not(feature = "solana-program"))]
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

// Define accounts using the new ShankAccounts derive macro
// This generates both IDL metadata AND context structs with AccountInfo
#[derive(ShankAccounts)]
pub struct CreateVaultAccounts<'info> {
    /// Vault account to initialize
    #[account(mut, desc = "The vault account to be initialized")]
    pub vault: &'info AccountInfo<'info>,
    
    /// Authority that controls the vault
    #[account(signer, desc = "The authority that will control this vault")]
    pub authority: &'info AccountInfo<'info>,
    
    /// Payer for account creation
    #[account(mut, signer, desc = "Account paying for the vault creation")]
    pub payer: &'info AccountInfo<'info>,
    
    /// Optional new authority
    #[account(optional, desc = "Optional new authority for the vault")]
    pub new_authority: &'info AccountInfo<'info>,
    
    /// System program
    #[account(desc = "System program")]
    pub system_program: &'info AccountInfo<'info>,
}

#[derive(ShankAccounts)]
pub struct UpdateVaultAccounts<'info> {
    /// Vault to update
    #[account(mut, desc = "The vault account to update")]
    pub vault: &'info AccountInfo<'info>,
    
    /// Current authority
    #[account(signer, desc = "Current vault authority")]
    pub authority: &'info AccountInfo<'info>,
}

// Arguments for instructions
pub struct CreateVaultArgs {
    pub initial_amount: u64,
}

pub struct UpdateVaultArgs {
    pub new_value: u64,
}

// Define instructions using the account structs
#[derive(ShankInstruction)]
pub enum VaultInstruction {
    /// Create a new vault
    #[accounts(CreateVaultAccounts)]
    CreateVault(CreateVaultArgs),
    
    /// Update an existing vault  
    #[accounts(UpdateVaultAccounts)]
    UpdateVault(UpdateVaultArgs),
}

// Example processor functions using the generated context structs
#[cfg(feature = "solana-program")]
pub fn process_create_vault<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    _data: &[u8],
) -> ProgramResult {
    // Use the generated context method for type-safe account access
    let ctx = CreateVaultAccounts::context(program_id, accounts)?;
    
    // Access accounts by name with compile-time guarantees
    msg!("Creating vault: {}", ctx.vault.key);
    msg!("Authority: {}", ctx.authority.key);
    msg!("Payer: {}", ctx.payer.key);
    
    // Handle optional accounts safely
    match ctx.new_authority {
        Some(new_auth) => {
            msg!("Setting new authority: {}", new_auth.key);
            // Process new authority
        }
        None => {
            msg!("No new authority provided");
        }
    }
    
    // Perform vault initialization logic here
    msg!("Vault created successfully");
    
    Ok(())
}

#[cfg(feature = "solana-program")]
pub fn process_update_vault<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    _data: &[u8],
) -> ProgramResult {
    // Use the generated context method
    let ctx = UpdateVaultAccounts::context(program_id, accounts)?;
    
    // Type-safe account access
    msg!("Updating vault: {}", ctx.vault.key);
    msg!("Authority: {}", ctx.authority.key);
    
    // Perform update logic here
    msg!("Vault updated successfully");
    
    Ok(())
}

#[cfg(not(feature = "solana-program"))]
fn main() {
    println!("This example demonstrates ShankAccounts with real AccountInfo types.");
    println!("To see the full functionality, enable the 'solana-program' feature:");
    println!("cargo run --features solana-program --example full_solana_example");
    println!();
    println!("Key benefits:");
    println!("1. Single source of truth for account definitions");
    println!("2. Type-safe account access by name instead of array indexing");
    println!("3. Automatic validation of account count and optional accounts");
    println!("4. Generated context structs work like Anchor's account contexts");
    println!("5. Both IDL generation and runtime functionality from one macro");
}

#[cfg(feature = "solana-program")]
fn main() {
    println!("Full Solana program example with ShankAccounts!");
    println!();
    println!("The ShankAccounts macro generates:");
    println!("1. CreateVaultAccountsContext<'a> struct with AccountInfo fields");
    println!("2. CreateVaultAccounts::context() method for validation");
    println!("3. IDL metadata for shank-idl extraction");
    println!();
    println!("Usage in processor:");
    println!("let ctx = CreateVaultAccounts::context(program_id, accounts)?;");
    println!("msg!(\"Vault: {{}}\", ctx.vault.key);");
    println!();
    println!("This provides the same functionality as Anchor's account contexts");
    println!("while remaining compatible with native Solana programs.");
}