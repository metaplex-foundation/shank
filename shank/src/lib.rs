extern crate shank_macro;
pub use shank_macro::*;

pub mod context;
pub use context::Context;

/// Trait for types that can provide account information
pub trait AccountInfoRef {
    /// Get the account's public key
    fn key(&self) -> &[u8; 32];
}
