use shank::ShankAccounts;

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

#[derive(ShankAccounts)]
pub struct EmptyAccounts {}

fn main() {}