use shank::{ShankAccount, ShankType};

// Placeholder for Pubkey type (normally from solana_program)
pub struct Pubkey([u8; 32]);

// These types would normally come from the podded crate or similar
/// Represents an optional i64 using sentinel values instead of borsh tag
pub struct OptionalI64(pub i64);

/// Represents an optional u64 using sentinel values
pub struct OptionalU64(pub u64);

/// Represents an optional pubkey using sentinel values instead of borsh tag
pub struct OptionalPubkey(pub Pubkey);

/// Generic PodOption type (normally from podded crate)
pub struct PodOption<T>(pub T);

/// Custom type with a pod_sentinel for use with PodOption
#[derive(ShankType)]
#[pod_sentinel(0xFF, 0xFF, 0xFF, 0xFF)]
pub struct CustomU32Wrapper {
    pub value: u32,
}

/// Account demonstrating podded/bytemuck types
#[derive(ShankAccount)]
pub struct AccountWithPoddedTypes {
    /// Regular borsh optional field for comparison
    pub regular_option: Option<u32>,

    /// Fixed-width optional i64 (uses sentinel value, not tag byte)
    pub claim_start_time: OptionalI64,

    /// Fixed-width optional u64
    pub optional_amount: OptionalU64,

    /// Pod pubkey type
    pub optional_pubkey: OptionalPubkey,

    /// Regular field
    pub counter: u64,

    /// Generic PodOption with i64
    pub pod_option_i64: PodOption<i64>,

    /// Generic PodOption with u32
    pub pod_option_u32: PodOption<u32>,

    /// Generic PodOption with Pubkey
    pub pod_option_pubkey: PodOption<Pubkey>,

    /// Generic PodOption with custom type
    pub pod_option_custom: PodOption<CustomU32Wrapper>,

    /// Testing that idl_type override still works
    #[idl_type("Option<i64>")]
    pub manual_override: OptionalI64,
}
