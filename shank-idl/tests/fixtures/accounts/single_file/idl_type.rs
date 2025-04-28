use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankAccount, ShankType};

/// An enum that will be used with the idl_type attribute
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

/// Account with fields using the idl_type(...) attribute
#[derive(ShankAccount)]
pub struct AccountWithIdlType {
    /// A regular field without any attribute
    pub regular_field: u32,

    /// A field stored as u8 but representing an enum (using string literal format)
    #[idl_type("TestEnum")]
    pub enum_as_byte_str: u8,

    /// A field with a wrapper type that should be treated as a simpler type (using string literal format)
    #[idl_type("u64")]
    pub wrapped_u64_str: CustomU64Wrapper,

    /// A field stored as u8 but representing an enum (using direct type format)
    #[idl_type(TestEnum)]
    pub enum_as_byte_direct: u8,

    /// A field with a wrapper type that should be treated as a simpler type (using direct type format)
    #[idl_type(u32)]
    pub wrapped_u32_direct: CustomU32Wrapper,
}

// Notes: This test does not check:
// - The ability to reference a path (like std::string::String)
// - Parsing failure when the direct type is not found
