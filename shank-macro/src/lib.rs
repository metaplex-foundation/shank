use account::derive_account;
use accounts::derive_accounts;
use builder::derive_builder;
use context::derive_context;
use instruction::derive_instruction;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error as ParseError};

mod account;
mod accounts;
mod builder;
mod context;
mod instruction;

// -----------------
// #[derive(ShankAccount)]
// -----------------

/// Annotates a _struct_ that shank will consider an account containing de/serializable data.
///
/// # Field Attributes
///
/// ## `#[idl_type(...)]` attribute
///
/// This attribute allows you to override how Shank interprets a field's type when generating the IDL.
/// This is useful for:
///
/// 1. Fields with wrapper types that should be treated as their inner types in the IDL
/// 2. Fields storing enum values as primitives (like `u8`) that should be recognized as enums
/// 3. Fields with complex types that need simpler representations in the IDL
///
/// The attribute supports two formats:
///
/// 1. **String literal format**: `#[idl_type("TypeName")]`
/// 2. **Direct type format**: `#[idl_type(TypeName)]`
///
/// ```
/// use shank::ShankAccount;
///
/// #[derive(ShankAccount)]
/// pub struct MyAccount {
///     // Field stored as u8 but representing an enum
///     #[idl_type("MyEnum")]
///     pub enum_as_byte: u8,
///
///     // Field with a wrapper type that should be treated as a simpler type
///     #[idl_type("u64")]
///     pub wrapped_u64: CustomU64Wrapper,
/// }
/// ```
///
/// ## `#[padding]` attribute
///
/// Indicates that a field is used for padding and should be marked as such in the IDL.
///
/// ## `#[idl_name("name")]` attribute
///
/// Allows you to override the field name that appears in the IDL while keeping the original Rust field name.
///
/// ```
/// #[derive(ShankAccount)]
/// pub struct MyAccount {
///     #[idl_name("displayName")]
///     pub internal_name: String,
/// }
/// ```
///
/// ## `#[skip]` attribute
///
/// Excludes the field from the IDL entirely. The field will not appear in the generated IDL.
///
/// ```
/// #[derive(ShankAccount)]
/// pub struct MyAccount {
///     pub public_field: u64,
///     #[skip]
///     pub internal_only_field: String,
/// }
/// ```
///
/// # Example
///
/// ```
/// use shank::ShankAccount;
/// use borsh::{BorshDeserialize, BorshSerialize};
///
/// #[derive(Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
/// pub struct Metadata {
///     pub update_authority: Pubkey,
///     pub mint: Pubkey,
///     pub primary_sale_happened: bool,
/// }
/// ```
///
/// # Seeds
///
/// You can include a `#[seeds]` annotation which allows shank to generate the following `impl`
/// methods for the particular account.
///
/// A seed takes one of the following patterns:
///
/// - `"literal"` this will be hardcoded into the seed/pda methods and does not need to be passed
/// via an argument
/// - `program_id` (known pubkey) this is the program id of the program which is passed to methods
/// - `label("description"[, type])` a seed of name _label_ with the provided description and an
/// optional type (if no type is provided `Pubkey` is assumed); this will be passed as an argument
///
/// Below is an example of each:
///
/// ```
/// #[derive(ShankAccount)]
/// #[seeds(
///     "lit:prefix",                        // a string literal which will be hard coded
///     program_id                           // the public key of the program which needs to be provided
///     pub_key_implicit("desc of the key"), // a public key which needs to be provided
///     pub_key("desc of the key", Pubkey),  // same as the above, explicitly declaring as pubkey
///     id("desc of byte", u8),              // a byte
///     name("desc of name", String)         // a string
/// )]
/// struct AccountStructWithSeeds {
///     count: u8,
/// }
/// ```
/// When seeds are specified for an account it will derive the following _static_ methods for that
/// account:
///
/// ```
/// AccountName::shank_seeds<'a>(..) -> [&'a [u8]; Nusize]
/// AccountName::shank_seeds_with_bump<'a>(.., bump: &'a [u8; 1]) -> [&'a [u8]; Nusize]
///
/// AccountName::shank_pda(program_id: Pubkey, ..) -> (Pubkey, u8)
/// AccountName::shank_pda_with_bump(program_id: Pubkey, bump: u8, ..) -> (Pubkey, u8)
/// ```
///
///# Note
///
/// The fields of a _ShankAccount_ struct can reference other types as long as they are annotated
/// with `ShankType`, `BorshSerialize` or `BorshDeserialize`.
#[proc_macro_derive(
    ShankAccount,
    attributes(padding, seeds, idl_type, idl_name, skip)
)]
pub fn shank_account(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_account(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

// -----------------
// #[derive(ShankInstruction)]
// -----------------

/// Annotates the program _Instruction_ `Enum` in order to include `#[account]` attributes.
///
/// The `#[account]` attributes indicate for each instruction _variant_ which accounts it expects
/// and how they should be configured.
///
/// # `#[account]` attribute
///
/// This attribute allows you to configure each account that is provided to the particular
/// instruction. These annotations need to follow the order in which the accounts are provided.
/// They take the following general form:
///
/// ```
/// #[account(index?, writable?, (signer|optional_signer)?, optional?, name="<account_name>", desc?="optional description")]
/// ```
///
/// - `index`: optionally provides the account index in the provided accounts array which needs to
///   match its position of `#[account]` attributes
/// - `signer` | `sign` | `sig`: indicates that the account is _signer_
/// - `optional_signer`: indicates that the account is _optional_signer_
/// - `writable` | `write` | `writ` | `mut`: indicates that the account is _writable_ which means it may be
///   mutated as part of processing the particular instruction
/// - `optional | option | opt`: indicates that this account is optional
/// - `name`: (required) provides the name for the account
/// - `desc` | `description` | `docs`: allows to provide a description of the account
///
/// When the `optional` attribute is added to an account, shank will mark it such that its value should default
/// to the `progam_id` if it is not provided by the client. Thus the position of optional accounts is static and
/// optional accounts that are set can follow ones that are not.
///
/// # Known Accounts
///
/// If an account `name` matches either of the a _known_ accounts indicated below then
/// [solita](https://github.com/metaplex-foundation/solita) generated SDK code won't require providing
/// it as the program id is known.
///
/// - `token_program` uses `TOKEN_PROGRAM_ID`
/// - `ata_program` uses `ASSOCIATED_TOKEN_PROGRAM_ID`
/// - `system_program` uses `SystemProgram.programId`
/// - `rent` uses `SYSVAR_RENT_PUBKEY`
///
/// # Optional Accounts Strategies
///
/// The default strategy (without `#[legacy_optional_accounts_strategy]`) is to set the `program_id` in place
/// of an optional account not set by the client. When the `#[legacy_optional_accounts_strategy]` is added,
/// shank will instead omit unset optional accounts from the accounts array.
///
/// **NOTE**: shank doesn't do anything different here aside from setting a flag for the
/// particular instruction. Thus adding that strategy to an instruction enum is merely advisory and
/// will is expected to be properly respected by code generator tools like
/// [kinobi](https://github.com/metaplex-foundation/kinobi) and [solita](https://github.com/metaplex-foundation/solita).
///
/// # Examples
///
/// ```
/// use borsh::{BorshDeserialize, BorshSerialize};
/// use shank::ShankInstruction;
/// #[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
/// #[rustfmt::skip]
/// pub enum VaultInstruction {
///     /// Initialize a token vault, starts inactivate. Add tokens in subsequent instructions, then activate.
///     #[account(0, writable, name="fraction_mint",
///               desc="Initialized fractional share mint with 0 tokens in supply, authority on mint must be pda of program with seed [prefix, programid]")]
///     #[account(1, writable, name="redeem_treasury",
///             desc = "Initialized redeem treasury token account with 0 tokens in supply, owner of account must be pda of program like above")]
///     #[account(2, writable, name="fraction_treasury",
///             desc = "Initialized fraction treasury token account with 0 tokens in supply, owner of account must be pda of program like above")]
///     #[account(3, writable, name="vault",
///             desc = "Uninitialized vault account")]
///     #[account(4, optional_signer, name="authority",
///             desc = "Authority on the vault")]
///     #[account(5, name="pricing_lookup_address",
///             desc = "Pricing Lookup Address")]
///     #[account(6, name="token_program",
///             desc = "Token program")]
///     #[account(7, name="rent",
///             desc = "Rent sysvar")]
///     InitVault(InitVaultArgs),
///
///     /// Activates the vault, distributing initial shares into the fraction treasury.
///     /// Tokens can no longer be removed in this state until Combination.
///     #[account(0, writable, name="vault", desc = "Initialized inactivated fractionalized token vault")]
///     #[account(1, writable, name="fraction_mint", desc = "Fraction mint")]
///     #[account(2, writable, name="fraction_treasury", desc = "Fraction treasury")]
///     #[account(3, name="fraction_mint_authority", desc = "Fraction mint authority for the program - seed of [PREFIX, program_id]")]
///     #[account(4, signer, name="vault_authority", desc = "Authority on the vault")]
///     #[account(5, name="token_program", desc = "Token program")]
///     ActivateVault(NumberOfShareArgs)
/// }
/// ```
#[proc_macro_derive(
    ShankInstruction,
    attributes(account, accounts, legacy_optional_accounts_strategy)
)]
pub fn shank_instruction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_instruction(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

// -----------------
// #[derive(ShankBuilder)]
// -----------------

/// Generates instruction builders for each annotated instruction.
///
/// An instruction builder is an _struct_ that contains all the accounts for an instruction. You can
/// also include `#[args]` attributes to specify additional arguments that are passed to the builder.
///
///
/// # Example
///
/// When you annotate your instruction with `#[derive(ShankBuilder)]`:
///
/// ```
/// use borsh::{BorshDeserialize, BorshSerialize};
/// use shank::ShankBuilder;
/// #[derive(Debug, Clone, ShankBuilder, BorshSerialize, BorshDeserialize)]
/// #[rustfmt::skip]
/// pub enum Instruction {
///     /// This instruction stores an amout in the vault.
///     #[account(0, writable, name="vault", desc="Vault account")]
///     #[account(1, signer, name="authority", desc = "Authority of the vault")]
///     #[account(2, signer, writable, name = "payer", desc = "Payer")]
///     #[account(3, name = "system_program", desc = "System program")]
///     #[args(additional_accounts: Vec<AccountMeta>)]
///     Create(CreateArgs)
/// }
/// ```
///
/// Shank will generate a `CreateBuilder` _struct_ in a submodule called `builders`. The builder can be used
/// to define the accounts and arguments for the instruction:
///
/// ```
/// let create_ix = CreateBuilder::new()
///    .vault(vault_pubkey)
///    .authority(authority_pubkey)
///    .payer(payer_pubkey)
///    .build(additional_accounts)
///    .instruction();
/// ```
#[proc_macro_derive(ShankBuilder, attributes(account, args))]
pub fn shank_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_builder(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

// -----------------
// #[derive(ShankContext)]
// -----------------

/// Generates an accounts _struct_ for each instruction.
///
/// The _struct_ will contain all shank annotated accounts and the _impl_ block
/// will initialize them from the accounts array. This struct can be used in combination
/// with a `Context` to provide access to accounts by name. The accounts _strct_ supports
///  the use of optional accounts, which would generate an account field with an
/// `Option<AccountInfo<'a>>` type.
///
/// # Example
///
/// When you annotate your instruction with `#[derive(ShankContext)]`:
///
/// ```
/// use borsh::{BorshDeserialize, BorshSerialize};
/// use shank::ShankContext;
/// #[derive(Debug, Clone, ShankContext, BorshSerialize, BorshDeserialize)]
/// #[rustfmt::skip]
/// pub enum Instruction {
///     /// This instruction stores an amout in the vault.
///     #[account(0, writable, name="vault", desc="Vault account")]
///     #[account(1, signer, name="authority", desc = "Authority of the vault")]
///     #[account(2, signer, writable, name = "payer", desc = "Payer")]
///     #[account(3, name = "system_program", desc = "System program")]
///     #[args(amount: u64)]
///     Create(CreateOrUpdateArgs)
/// }
/// ```
///
/// A `CreateAccounts` and a generic `Context` _structs_ will be generated, which can be used to
/// access each account by name in your processor implementation:
///
/// ```
/// pub fn process_create<'a>(
///     program_id: &Pubkey,
///     accounts: &'a [AccountInfo<'a>],
///     instruction_data: &[u8],
/// ) -> ProgramResult {
///     let context = CreateAccounts::context(accounts)?;
///
///     msg!("{}", context.accounts.vault.key);
///     msg!("{}", context.accounts.authority.key);
///     msg!("{}", context.accounts.payer.key);
///     ...
/// }
/// ```
#[proc_macro_derive(ShankContext, attributes(account))]
pub fn shank_context(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_context(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

// -----------------
// #[derive(ShankAccounts)]
// -----------------

/// Annotates a _struct_ that defines accounts for an instruction in a similar way to Anchor.
///
/// This is designed as a complete replacement for both:
/// - The `#[account]` attribute system on instruction enums
/// - The `ShankContext` derive macro for context generation
///
/// Instead of annotating instruction variants directly, you define a separate struct
/// that contains all accounts with their constraints. This generates both IDL metadata
/// and runtime context handling code for type-safe account access.
///
/// # Field Attributes
///
/// Each field in the struct represents an account and can be annotated with attributes:
///
/// - `#[account(writable)]` or `#[account(mut)]` - The account is writable
/// - `#[account(signer)]` - The account must sign the transaction
/// - `#[account(optional_signer)]` - The account may optionally sign
/// - `#[account(optional)]` - The account is optional (defaults to program_id when not provided)
/// - `#[account(desc = "...")]` - Description of the account for documentation
///
/// # Example
///
/// ```ignore
/// use shank::ShankAccounts;
///
/// #[derive(ShankAccounts)]
/// pub struct CreateVaultAccounts {
///     #[account(mut, desc = "Initialized fractional share mint")]
///     pub fraction_mint: std::marker::PhantomData<()>,
///     
///     #[account(mut, desc = "Initialized redeem treasury")]
///     pub redeem_treasury: std::marker::PhantomData<()>,
///     
///     #[account(mut, desc = "Fraction treasury")]
///     pub fraction_treasury: std::marker::PhantomData<()>,
///     
///     #[account(mut, desc = "Uninitialized vault account")]
///     pub vault: std::marker::PhantomData<()>,
///     
///     #[account(optional_signer, desc = "Authority on the vault")]
///     pub authority: std::marker::PhantomData<()>,
///     
///     #[account(desc = "Token program")]
///     pub token_program: std::marker::PhantomData<()>,
/// }
///
/// // Then reference it in your instruction enum:
/// #[derive(ShankInstruction)]
/// pub enum VaultInstruction {
///     #[accounts(CreateVaultAccounts)]
///     InitVault(InitVaultArgs),
/// }
/// ```
///
/// # Generated Code
///
/// ShankAccounts generates:
/// 1. **IDL Metadata Methods** - For shank-idl to extract account information
/// 2. **Context Structs** - `{StructName}Context<'a>` with `AccountInfo<'a>` fields  
/// 3. **Context Methods** - `{StructName}::context(program_id, accounts)` for validation
///
/// # Usage in Solana Programs
///
/// ```ignore
/// pub fn process_init_vault(
///     program_id: &Pubkey,
///     accounts: &[AccountInfo],
///     data: &[u8],
/// ) -> ProgramResult {
///     let ctx = CreateVaultAccounts::context(program_id, accounts)?;
///     
///     // Type-safe access by name:
///     msg!("Vault: {}", ctx.vault.key);
///     msg!("Authority: {}", ctx.authority.key);
///     
///     Ok(())
/// }
/// ```
///
/// Note: The field types don't affect IDL generation - ShankAccounts only processes
/// the `#[account(...)]` attributes. In real Solana programs, use `AccountInfo<'info>`
/// from `solana_program::account_info` for field types.
#[proc_macro_derive(ShankAccounts, attributes(account))]
pub fn shank_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_accounts(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

// -----------------
// #[derive(ShankType)]
// -----------------

/// Annotates a _struct_ or _enum_ that shank will consider a type containing de/serializable data.
///
/// The macro does not generate any code. The annotation is used to indicate to shank-idl that the
/// the type should be included in the program's IDL.
///
/// # Example
///
/// ```
/// use shank::ShankType;
///
/// #[derive(ShankType)]
/// pub struct Metadata {
///     pub update_authority: Pubkey,
///     pub mint: Pubkey,
///     pub primary_sale_happened: bool,
/// }
/// ```
///
///# Note
///
/// The fields of a _ShankType_ struct or enum can reference other types as long as they are annotated
/// with `ShankType`, `BorshSerialize` or `BorshDeserialize`.
#[proc_macro_derive(ShankType, attributes(idl_name, idl_type, skip))]
pub fn shank_type(_input: TokenStream) -> TokenStream {
    // returns the token stream that was passed in (the macro is only an annotation for shank-idl
    // to export the type in the program's IDL)
    quote! {}.into()
}

fn to_compile_error(error: ParseError) -> proc_macro2::TokenStream {
    let compile_error = ParseError::to_compile_error(&error);
    quote!(#compile_error)
}
