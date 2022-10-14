use account::derive_account;
use instruction::derive_instruction;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error as ParseError};

mod account;
mod instruction;

// -----------------
// #[derive(ShankAccount)]
// -----------------

/// Annotates a _struct_ that shank will consider an account containing de/serializable data.
///
/// # Examples
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
///# Note
///
/// The fields of a _ShankAccount_ struct can reference other types as long as they are annotated
/// with `BorshSerialize` or `BorshDeserialize`.
#[proc_macro_derive(ShankAccount, attributes(padding))]
pub fn shank_account(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_account(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

// -----------------
// #[derive(ShankInstructions)]
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
/// #[account(index?, (writable|signer)?, optional?, name="<account_name>", desc?="optional description")]
/// ```
///
/// - `index`: optionally provides the account index in the provided accounts array which needs to
///   match its position of `#[account]` attributes
/// - `signer` | `sign` | `sig`: indicates that the account is _signer_
/// - `writable` | `write` | `writ` | `mut`: indicates that the account is _writable_ which means it may be
///   mutated as part of processing the particular instruction
/// - `optional | option | opt`: indicates that this account is optional
/// - `name`: (required) provides the name for the account
/// - `desc` | `description`: allows to provide a description of the account
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
/// # Strategies
///
/// ## Defaulting Optional Accounts
///
/// When the `#[default_optional_accounts]` attribute is added to an Instruction enum, shank will mark it
/// such that optional accounts should default to the `progam_id` if they are not provided by the client.
/// Thus their position is static and optional accounts that are set can follow ones that are not.
///
/// The default strategy (without `#[default_optional_accounts]`) is to just omit unset optional
/// accounts from the accounts array.
///
/// **NOTE**: shank doesn't do anything different here aside from setting a flag for the
/// particular instruction. Thus adding that strategy to an instruction enum is merely advisory and
/// will is expected to be properly respected by code generator tools like
/// [solita](https://github.com/metaplex-foundation/solita).
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
///     #[account(4, name="authority",
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
    attributes(account, default_optional_accounts)
)]
pub fn shank_instruction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_instruction(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

fn to_compile_error(error: ParseError) -> proc_macro2::TokenStream {
    let compile_error = ParseError::to_compile_error(&error);
    quote!(#compile_error)
}
