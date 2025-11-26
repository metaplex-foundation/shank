use shank::ShankAccounts;
pub const ID: [u8; 32] = [1; 32];
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}
pub struct BasicAccounts<'info> {
    #[account(mut, signer, desc = "The payer account")]
    pub payer: &'info AccountInfo<'info>,
    #[account(mut, desc = "The data account")]
    pub data: &'info AccountInfo<'info>,
    #[account(desc = "The system program")]
    pub system_program: &'info AccountInfo<'info>,
}
impl<'info> BasicAccounts<'info> {
    #[doc(hidden)]
    pub fn __shank_accounts() -> Vec<
        (u32, &'static str, bool, bool, bool, bool, Option<String>),
    > {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                (
                    0u32,
                    "payer",
                    true,
                    true,
                    false,
                    false,
                    Some("The payer account".to_string()),
                ),
                (
                    1u32,
                    "data",
                    true,
                    false,
                    false,
                    false,
                    Some("The data account".to_string()),
                ),
                (
                    2u32,
                    "system_program",
                    false,
                    false,
                    false,
                    false,
                    Some("The system program".to_string()),
                ),
            ]),
        )
    }
}
impl<'info> BasicAccounts<'info> {
    /// Create a context from a slice of accounts
    ///
    /// This method parses the accounts according to the struct definition
    /// and returns a Context containing the account struct.
    ///
    /// Optional accounts are determined by checking if the account key
    /// equals the program ID (crate::ID). If so, they are set to None, otherwise Some.
    pub fn context(
        accounts: &'info [AccountInfo<'info>],
    ) -> ::shank::Context<'info, Self, AccountInfo<'info>> {
        if accounts.len() < 3usize {
            {
                ::std::rt::panic_fmt(
                    format_args!(
                        "Expected at least {0} accounts, got {1}", 3usize, accounts
                        .len(),
                    ),
                );
            };
        }
        let account_struct = Self {
            payer: &accounts[0usize],
            data: &accounts[1usize],
            system_program: &accounts[2usize],
        };
        ::shank::Context {
            accounts: account_struct,
            remaining_accounts: &accounts[3usize..],
        }
    }
}
fn main() {}
