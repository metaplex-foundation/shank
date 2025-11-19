use shank::ShankAccounts;
pub const ID: [u8; 32] = [1; 32];
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}
pub struct OptionalAccounts<'info> {
    #[account(signer, desc = "Required authority")]
    pub authority: &'info AccountInfo<'info>,
    #[account(optional, desc = "Optional data account")]
    pub optional_data: Option<&'info AccountInfo<'info>>,
    #[account(optional, signer, desc = "Optional authority")]
    pub optional_authority: Option<&'info AccountInfo<'info>>,
}
impl<'info> OptionalAccounts<'info> {
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
                    Some("Required authority".to_string()),
                ),
                (
                    1u32,
                    "optional_data",
                    false,
                    false,
                    false,
                    true,
                    Some("Optional data account".to_string()),
                ),
                (
                    2u32,
                    "optional_authority",
                    false,
                    true,
                    false,
                    true,
                    Some("Optional authority".to_string()),
                ),
            ]),
        )
    }
}
impl<'info> OptionalAccounts<'info> {
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
            authority: &accounts[0usize],
            optional_data: if accounts[1usize].key == &crate::ID {
                None
            } else {
                Some(&accounts[1usize])
            },
            optional_authority: if accounts[2usize].key == &crate::ID {
                None
            } else {
                Some(&accounts[2usize])
            },
        };
        ::shank::Context {
            accounts: account_struct,
            remaining_accounts: &accounts[3usize..],
        }
    }
}
fn main() {}
