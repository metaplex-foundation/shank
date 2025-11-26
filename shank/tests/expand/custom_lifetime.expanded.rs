use shank::ShankAccounts;
pub const ID: [u8; 32] = [1; 32];
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}
pub struct CustomLifetime<'a> {
    #[account(signer, desc = "Authority with custom lifetime")]
    pub authority: &'a AccountInfo<'a>,
    #[account(mut, desc = "Data with custom lifetime")]
    pub data: &'a AccountInfo<'a>,
}
impl<'a> CustomLifetime<'a> {
    #[doc(hidden)]
    pub fn __shank_accounts() -> Vec<
        (u32, &'static str, bool, bool, bool, bool, Option<String>),
    > {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                (
                    0u32,
                    "authority",
                    false,
                    true,
                    false,
                    false,
                    Some("Authority with custom lifetime".to_string()),
                ),
                (
                    1u32,
                    "data",
                    true,
                    false,
                    false,
                    false,
                    Some("Data with custom lifetime".to_string()),
                ),
            ]),
        )
    }
}
impl<'a> CustomLifetime<'a> {
    /// Create a context from a slice of accounts
    ///
    /// This method parses the accounts according to the struct definition
    /// and returns a Context containing the account struct.
    ///
    /// Optional accounts are determined by checking if the account key
    /// equals the program ID (crate::ID). If so, they are set to None, otherwise Some.
    pub fn context(
        accounts: &'a [AccountInfo<'a>],
    ) -> ::shank::Context<'a, Self, AccountInfo<'a>> {
        if accounts.len() < 2usize {
            {
                ::std::rt::panic_fmt(
                    format_args!(
                        "Expected at least {0} accounts, got {1}", 2usize, accounts
                        .len(),
                    ),
                );
            };
        }
        let account_struct = Self {
            authority: &accounts[0usize],
            data: &accounts[1usize],
        };
        ::shank::Context {
            accounts: account_struct,
            remaining_accounts: &accounts[2usize..],
        }
    }
}
fn main() {}
