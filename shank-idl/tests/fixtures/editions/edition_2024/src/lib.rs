//! Fixture crate opting into the Rust 2024 edition.
//!
//! Besides the shank annotations in `state.rs` it uses syntax that only
//! parses with the 2024 edition to make sure shank's source parser copes
//! with it.
use shank::{ShankAccount, ShankInstruction, ShankType};

solana_program::declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

mod state;

pub use state::*;

// Unsafe extern blocks with safe items (RFC 3484)
unsafe extern "C" {
    pub safe fn abs(input: i32) -> i32;
}

// Unsafe attributes (RFC 3325)
#[unsafe(no_mangle)]
pub extern "C" fn exported_symbol() {}

// Precise capturing in `impl Trait` (RFC 3617)
pub fn indices(items: &[u8]) -> impl Iterator<Item = u8> + use<'_> {
    items.iter().copied()
}

// let chains (RFC 2497)
pub fn both(a: Option<u8>, b: Option<u8>) -> Option<u8> {
    if let Some(a) = a
        && let Some(b) = b
    {
        Some(a + b)
    } else {
        None
    }
}

#[derive(ShankInstruction)]
pub enum CounterInstruction {
    /// Creates a new counter account.
    #[account(0, writable, signer, name = "payer", desc = "Pays for the counter")]
    #[account(1, writable, name = "counter", desc = "The counter PDA")]
    #[account(2, name = "system_program", desc = "System program")]
    Create { initial: u64 },

    /// Increments the counter.
    #[account(0, mut, name = "counter", desc = "The counter PDA")]
    #[account(1, signer, name = "authority")]
    #[account(2, optional, name = "log", desc = "Optional log account")]
    Increment(IncrementArgs),
}

#[derive(ShankType)]
pub struct IncrementArgs {
    pub amount: u64,
    pub reason: Reason,
}

#[derive(ShankType)]
pub enum Reason {
    Manual,
    Scheduled { at: i64 },
}
