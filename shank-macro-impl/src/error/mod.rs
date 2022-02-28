mod program_error;
mod this_error;

pub use program_error::*;
pub use this_error::*;

pub const DERIVE_THIS_ERROR_ATTR: &str = "Error";
