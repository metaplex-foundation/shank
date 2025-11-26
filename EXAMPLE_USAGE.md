# ShankAccounts with Optional Account Support

This demonstrates the improved ShankAccounts implementation with proper optional account handling.

## Example Program Structure

```rust
use shank::ShankAccounts;

// Program ID - normally created by declare_id! macro
pub const ID: [u8; 32] = [1; 32];

#[derive(ShankAccounts)]
pub struct CreateVaultAccounts<'info> {
    #[account(mut, signer, desc = "The payer and authority")]
    pub payer: &'info AccountInfo<'info>,
    
    #[account(mut, desc = "The vault account to create")]
    pub vault: &'info AccountInfo<'info>,
    
    #[account(optional, desc = "Optional new authority")]
    pub optional_authority: Option<&'info AccountInfo<'info>>,
    
    #[account(desc = "System program")]
    pub system_program: &'info AccountInfo<'info>,
}
```

## Usage

```rust
// When optional account is provided:
let accounts = [payer, vault, authority_account, system_program];
let ctx = CreateVaultAccounts::context(&accounts);
assert!(ctx.accounts.optional_authority.is_some());

// When optional account is NOT provided (pass program ID as placeholder):
let accounts = [payer, vault, program_id_placeholder, system_program];
let ctx = CreateVaultAccounts::context(&accounts);
assert!(ctx.accounts.optional_authority.is_none());
```

## Key Features

1. **Ergonomic API**: No need to pass program ID parameter - uses `crate::ID` automatically
2. **Type Safety**: Optional accounts use `Option<&AccountInfo>` types
3. **Runtime Detection**: Checks if `account.key == crate::ID` to determine None/Some
4. **IDL Generation**: Proper `"isOptional": true` flags in generated IDL
5. **Remaining Accounts**: Automatically handles extra accounts beyond the struct definition

## IDL Output

```json
{
  "accounts": [
    {"name": "payer", "isMut": true, "isSigner": true, "docs": ["The payer and authority"]},
    {"name": "vault", "isMut": true, "isSigner": false, "docs": ["The vault account to create"]}, 
    {"name": "optionalAuthority", "isMut": false, "isSigner": false, "isOptional": true, "docs": ["Optional new authority"]},
    {"name": "systemProgram", "isMut": false, "isSigner": false, "docs": ["System program"]}
  ]
}
```

This follows Solana's modern optional accounts pattern where missing optional accounts are represented by the program ID.