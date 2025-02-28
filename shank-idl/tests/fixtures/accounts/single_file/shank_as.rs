use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankAccount, ShankType};

/// An enum that will be used with the shank-as attribute
#[derive(ShankType)]
pub enum TestEnum {
    OptionA,
    OptionB,
    OptionC,
}

/// A wrapper type for u64
pub struct CustomU64Wrapper(pub u64);

/// Account with fields using the shank(as = "...") attribute
#[derive(ShankAccount)]
pub struct AccountWithShankAs {
    /// A regular field without any attribute
    pub regular_field: u32,

    /// A field stored as u8 but representing an enum
    #[shank(as = "TestEnum")]
    pub enum_as_byte: u8,

    /// A field with a wrapper type that should be treated as a simpler type
    #[shank(as = "u64")]
    pub wrapped_u64: CustomU64Wrapper,

    /// A direct enum field for comparison
    pub enum_as_enum: TestEnum,
}
