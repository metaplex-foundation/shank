//! Example demonstrating the new ShankAccounts derive macro
//! that provides an Anchor-like way to define instruction accounts

use shank::{ShankAccounts, ShankInstruction};

// Define accounts structs using the new ShankAccounts macro
// This is similar to how Anchor defines accounts
// Note: The actual field types don't matter for IDL generation -
// ShankAccounts only cares about the #[account(...)] attributes
// In a real Solana program, you would use AccountInfo<'info> from solana_program
#[derive(ShankAccounts)]
pub struct InitializeVaultAccounts {
    /// The vault account to initialize
    #[account(mut, desc = "Vault account to initialize")]
    pub vault: std::marker::PhantomData<()>, // Placeholder - use AccountInfo<'info> in real programs

    /// The authority that will control the vault
    #[account(signer, desc = "Authority that will control the vault")]
    pub authority: std::marker::PhantomData<()>,

    /// The token mint for the vault
    #[account(desc = "Token mint for the vault")]
    pub mint: std::marker::PhantomData<()>,

    /// The payer for account creation
    #[account(mut, signer, desc = "Payer for account creation")]
    pub payer: std::marker::PhantomData<()>,

    /// System program for account creation
    #[account(desc = "System program")]
    pub system_program: std::marker::PhantomData<()>,

    /// Token program
    #[account(desc = "Token program")]
    pub token_program: std::marker::PhantomData<()>,
}

#[derive(ShankAccounts)]
pub struct UpdateVaultAccounts {
    /// The vault account to update
    #[account(mut, desc = "Vault account to update")]
    pub vault: std::marker::PhantomData<()>,

    /// The authority that controls the vault
    #[account(signer, desc = "Authority that controls the vault")]
    pub authority: std::marker::PhantomData<()>,

    /// Optional new authority
    #[account(optional, desc = "Optional new authority")]
    pub new_authority: std::marker::PhantomData<()>,
}

#[derive(ShankAccounts)]
pub struct CloseVaultAccounts {
    /// The vault account to close
    #[account(mut, desc = "Vault account to close")]
    pub vault: std::marker::PhantomData<()>,

    /// The authority that controls the vault
    #[account(signer, desc = "Authority that controls the vault")]
    pub authority: std::marker::PhantomData<()>,

    /// The account to receive the rent
    #[account(mut, desc = "Account to receive the rent")]
    pub rent_receiver: std::marker::PhantomData<()>,
}

// Define the instruction enum using the new #[accounts(StructName)] attribute
// This references the account structs defined above
#[derive(Debug, Clone, ShankInstruction)]
pub enum VaultInstruction {
    /// Initialize a new vault
    #[accounts(InitializeVaultAccounts)]
    Initialize {
        /// The initial balance for the vault
        initial_balance: u64,
    },

    /// Update vault settings
    #[accounts(UpdateVaultAccounts)]
    Update {
        /// New settings for the vault
        new_settings: u8,
    },

    /// Close the vault and return rent
    #[accounts(CloseVaultAccounts)]
    Close,
}

// You can still use the old-style account attributes for backward compatibility
#[derive(Debug, Clone, ShankInstruction)]
pub enum LegacyInstruction {
    /// Old style instruction with inline account definitions
    #[account(
        0,
        writable,
        name = "data_account",
        desc = "Account to store data"
    )]
    #[account(1, signer, name = "authority", desc = "Authority")]
    #[account(2, name = "system_program", desc = "System program")]
    OldStyleInstruction { data: [u8; 32] },
}

fn main() {
    println!("This example demonstrates the new ShankAccounts derive macro.");
    println!("It provides an Anchor-like way to define instruction accounts.");
    println!();
    println!("Key points about ShankAccounts:");
    println!("1. The macro only cares about #[account(...)] attributes, not field types");
    println!("2. Field types can be anything - PhantomData, AccountInfo, etc.");
    println!("3. Provides cleaner separation of account definitions from instruction logic");
    println!("4. Account structs can be reused across multiple instructions");
    println!("5. More similar to Anchor's account definition style");
    println!(
        "6. Fully backward compatible - old #[account(...)] style still works"
    );
    println!();
    println!("ShankAccounts is designed to be a complete replacement for:");
    println!(
        "- Traditional #[account(...)] attributes on instruction variants"
    );
    println!("- ShankContext derive macro for context generation");
    println!();
    println!("In a real Solana program, you would typically:");
    println!("- Use AccountInfo<'info> from solana_program for field types");
    println!("- Import account_info::AccountInfo with proper lifetimes");
    println!("- Get generated context structs for type-safe account access");
    println!("- Use validation methods on the context structs");
    println!("- Benefit from automatic IDL generation");
    println!();
    println!("Example usage in a Solana program processor:");
    println!("");
    println!("pub fn process_init_vault(");
    println!("    program_id: &Pubkey,");
    println!("    accounts: &[AccountInfo],");
    println!("    data: &[u8],");
    println!(") -> ProgramResult {{");
    println!("    // This would be generated by ShankAccounts:");
    println!("    let ctx = InitializeVaultAccounts::context(program_id, accounts)?;");
    println!("    ");
    println!("    // Type-safe access to accounts by name:");
    println!("    msg!(\"Vault: {{}}\", ctx.vault.key);");
    println!("    msg!(\"Authority: {{}}\", ctx.authority.key);");
    println!("    ");
    println!("    // Handle optional accounts:");
    println!("    if let Some(new_auth) = ctx.new_authority {{");
    println!("        // Process optional account");
    println!("    }}");
    println!("    ");
    println!("    Ok(())");
    println!("}}");
    println!();
    println!("The ShankAccounts macro would generate both IDL metadata");
    println!("and runtime context handling code.");
}
