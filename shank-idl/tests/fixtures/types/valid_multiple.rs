#[derive(BorshSerialize)]
pub struct OneCustomType {
    pub field: u8,
}

#[derive(BorshDeserialize)]
pub struct OtherCustomType {
    pub field: Option<String>,
}

#[derive(BorshDeserialize)]
pub enum EnumCustomType {
    Up(u8),
    Down(u8),
}

/// Misses serialization attrs
pub struct NotCustomType {
    pub field: u8,
}

/// Has serialization attr, but also ShankInstruction
#[derive(BorshDeserialize, ShankInstruction)]
pub struct AlsoNotCustomType {
    pub field: u8,
}

/// Has serialization attr, but also ShankAccount
#[derive(BorshDeserialize, ShankAccount)]
pub struct AccountType {
    pub field: u8,
}
