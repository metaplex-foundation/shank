use shank::ShankAccounts;

// Mock program ID
pub const ID: [u8; 32] = [1; 32];

// Mock AccountInfo for testing
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}

#[derive(ShankAccounts)]
pub struct NoConstraints<'info> {
    pub read_only1: &'info AccountInfo<'info>,
    pub read_only2: &'info AccountInfo<'info>,
    pub read_only3: &'info AccountInfo<'info>,
}

fn main() {}