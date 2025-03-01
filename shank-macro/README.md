# shank_macro

Provides macros used to annotate Solana Rust programs in order to extract an IDL with the shank
CLI.

### ShankAccount

Annotates a _struct_ that shank will consider an account containing de/serializable data.

```rs
use shank::ShankAccount;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct Metadata {
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub primary_sale_happened: bool,
}
```

### Field Attributes

#### `#[idl_type(...)]` attribute

This attribute allows you to override how Shank interprets a field's type when generating the IDL. This is useful for:

1. Fields with wrapper types that should be treated as their inner types in the IDL
2. Fields storing enum values as primitives (like `u8`) that should be recognized as enums
3. Fields with complex types that need simpler representations in the IDL

The attribute supports two formats:

1. **String literal format**: `#idl_type("TypeName")]`
2. **Direct type format**: `#[idl_type(TypeName)]`

The difference between these is that the direct type format will error at runtime if the referenced type cannot be found in the Rust type system.
In the future, ideally the direct type format would perform checks on the given type.

```rs
use shank::ShankAccount;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct MyAccount {
    // Regular field
    pub regular_field: u32,

    // Field stored as u8 but representing an enum (string literal format)
    #[idl_type("MyEnum")]
    pub enum_as_byte_str: u8,

    // Field with a wrapper type that should be treated as a simpler type (string literal format)
    #[idl_type("u64")]
    pub wrapped_u64_str: CustomU64Wrapper,

    // Field stored as u8 but representing an enum (direct type format)
    #[idl_type(MyEnum)]
    pub enum_as_byte_direct: u8,

    // Field with a wrapper type that should be treated as a simpler type (direct type format)
    #[idl_type(u64)]
    pub wrapped_u32_direct: CustomU32Wrapper,
}
```

The type specified in the `as` parameter must be a valid Rust type that Shank can recognize. If the type is a custom type (like an enum), make sure it's defined in your codebase and is accessible to Shank during IDL generation.

#### `#[padding]` attribute

Indicates that a field is used for padding and should be marked as such in the IDL.

### Note

The fields of a _ShankAccount_ struct can reference other types as long as they are annotated
with `BorshSerialize`, `BorshDeserialize`, or `ShankType`.

## ShankInstruction

Annotates the program _Instruction_ `Enum` in order to include `#[account]` attributes.

The `#[account]` attributes indicate for each instruction _variant_ which accounts it expects
and how they should be configured.

### `#[account]` attribute

This attribute allows you to configure each account that is provided to the particular
instruction. These annotations need to follow the order in which the accounts are provided.
They take the following general form:

```rs
#[account(index?, (writable|signer)?, name="<account_name>", desc?="optional description")]
```

- `index`: optionally provides the account index in the provided accounts array which needs to
  match its position of `#[account]` attributes
- `signer` | `sign` | `sig`: indicates that the account is _signer_
- `writable` | `write` | `writ` | `mut`: indicates that the account is _writable_ which means it may be
  mutated as part of processing the particular instruction
- `name`: (required) provides the name for the account
- `desc` | `description`: allows to provide a description of the account

### Known Accounts

If an account `name` matches either of the a _known_ accounts indicated below then
[solita](https://github.com/metaplex-foundation/solita) generated SDK code won't require providing
it as the program id is known.

- `token_program` uses `TOKEN_PROGRAM_ID`
- `ata_program` uses `ASSOCIATED_TOKEN_PROGRAM_ID`
- `system_program` uses `SystemProgram.programId`
- `rent` uses `SYSVAR_RENT_PUBKEY`

```rs
use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum VaultInstruction {
    /// Initialize a token vault, starts inactivate. Add tokens in subsequent instructions, then activate.
    #[account(0, writable, name="fraction_mint",
              desc="Initialized fractional share mint with 0 tokens in supply, authority on mint must be pda of program with seed [prefix, programid]")]
    #[account(1, writable, name="redeem_treasury",
            desc = "Initialized redeem treasury token account with 0 tokens in supply, owner of account must be pda of program like above")]
    #[account(2, writable, name="fraction_treasury",
            desc = "Initialized fraction treasury token account with 0 tokens in supply, owner of account must be pda of program like above")]
    #[account(3, writable, name="vault",
            desc = "Uninitialized vault account")]
    #[account(4, name="authority",
            desc = "Authority on the vault")]
    #[account(5, name="pricing_lookup_address",
            desc = "Pricing Lookup Address")]
    #[account(6, name="token_program",
            desc = "Token program")]
    #[account(7, name="rent",
            desc = "Rent sysvar")]
    InitVault(InitVaultArgs),

    /// Activates the vault, distributing initial shares into the fraction treasury.
    /// Tokens can no longer be removed in this state until Combination.
    #[account(0, writable, name="vault", desc = "Initialized inactivated fractionalized token vault")]
    #[account(1, writable, name="fraction_mint", desc = "Fraction mint")]
    #[account(2, writable, name="fraction_treasury", desc = "Fraction treasury")]
    #[account(3, name="fraction_mint_authority", desc = "Fraction mint authority for the program - seed of [PREFIX, program_id]")]
    #[account(4, signer, name="vault_authority", desc = "Authority on the vault")]
    #[account(5, name="token_program", desc = "Token program")]
    ActivateVault(NumberOfShareArgs)
}
```

## LICENSE

Apache-2.0
