pub mod account;
pub mod converters;
pub mod custom_type;
pub mod error;
pub mod instruction;
pub mod krate;
pub mod macros;
pub mod parsed_enum;
pub mod parsed_macro;
pub mod parsed_struct;
pub mod parsers;
pub mod types;

pub const DERIVE_INSTRUCTION_ATTR: &str = "ShankInstruction";
pub const DERIVE_ACCOUNT_ATTR: &str = "ShankAccount";
