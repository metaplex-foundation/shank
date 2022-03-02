/// Errors that may be returned by the Vault program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum VaultError {
    /// Invalid instruction data passed in.
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError,

    /// Lamport balance below rent-exempt threshold.
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,

    /// Already initialized
    #[error("Already initialized")]
    AlreadyInitialized,

    /// Uninitialized
    #[error("Uninitialized")]
    Uninitialized,

    /// Account does not have correct owner
    #[error("Account does not have correct owner")]
    IncorrectOwner,

    /// NumericalOverflowError
    #[error("NumericalOverflowError")]
    NumericalOverflowError,
}
