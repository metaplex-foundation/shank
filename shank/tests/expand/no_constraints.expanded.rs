use shank::ShankAccounts;
pub const ID: [u8; 32] = [1; 32];
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}
pub struct NoConstraints<'info> {
    pub read_only1: &'info AccountInfo<'info>,
    pub read_only2: &'info AccountInfo<'info>,
    pub read_only3: &'info AccountInfo<'info>,
}
impl<'info> NoConstraints<'info> {
    #[doc(hidden)]
    pub fn __shank_accounts() -> Vec<
        (u32, &'static str, bool, bool, bool, bool, Option<String>),
    > {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                (0u32, "read_only1", false, false, false, false, None),
                (1u32, "read_only2", false, false, false, false, None),
                (2u32, "read_only3", false, false, false, false, None),
            ]),
        )
    }
}
impl<'info> NoConstraints<'info> {
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
            read_only1: &accounts[0usize],
            read_only2: &accounts[1usize],
            read_only3: &accounts[2usize],
        };
        ::shank::Context {
            accounts: account_struct,
            remaining_accounts: &accounts[3usize..],
        }
    }
}
fn main() {}
