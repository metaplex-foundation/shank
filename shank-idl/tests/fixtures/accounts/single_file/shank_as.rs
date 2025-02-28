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

/// Another wrapper type for u32
pub struct CustomU32Wrapper(pub u32);

/// Account with fields using the shank(as = "...") attribute
#[derive(ShankAccount)]
pub struct AccountWithShankAs {
    /// A regular field without any attribute
    pub regular_field: u32,

    /// A field stored as u8 but representing an enum (using string literal format)
    #[shank(as = "TestEnum")]
    pub enum_as_byte_str: u8,

    /// A field with a wrapper type that should be treated as a simpler type (using string literal format)
    #[shank(as = "u64")]
    pub wrapped_u64_str: CustomU64Wrapper,

    /// A field stored as u8 but representing an enum (using direct type format)
    #[shank(as = TestEnum)]
    pub enum_as_byte_direct: u8,

    /// A field with a wrapper type that should be treated as a simpler type (using direct type format)
    #[shank(as = u32)]
    pub wrapped_u32_direct: CustomU32Wrapper,

    /// A direct enum field for comparison
    pub enum_as_enum: TestEnum,
}
