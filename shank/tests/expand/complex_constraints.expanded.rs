use shank::ShankAccounts;
pub const ID: [u8; 32] = [1; 32];
pub struct AccountInfo<'info> {
    pub key: &'info [u8; 32],
    pub data: &'info [u8],
    pub owner: &'info [u8; 32],
}
pub struct ComplexAccounts<'info> {
    #[account(mut, signer, desc = "Payer and authority")]
    pub payer: &'info AccountInfo<'info>,
    #[account(mut, desc = "Mutable data")]
    pub data: &'info AccountInfo<'info>,
    #[account(desc = "Read-only account")]
    pub read_only: &'info AccountInfo<'info>,
    #[account(optional, mut, desc = "Optional mutable account")]
    pub optional_mut: Option<&'info AccountInfo<'info>>,
    #[account(optional, signer, desc = "Optional signer")]
    pub optional_signer: Option<&'info AccountInfo<'info>>,
    #[account(optional, mut, signer, desc = "Optional mutable signer")]
    pub optional_mut_signer: Option<&'info AccountInfo<'info>>,
}
impl<'info> ComplexAccounts<'info> {
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
                    Some("Payer and authority".to_string()),
                ),
                (
                    1u32,
                    "data",
                    true,
                    false,
                    false,
                    false,
                    Some("Mutable data".to_string()),
                ),
                (
                    2u32,
                    "read_only",
                    false,
                    false,
                    false,
                    false,
                    Some("Read-only account".to_string()),
                ),
                (
                    3u32,
                    "optional_mut",
                    true,
                    false,
                    false,
                    true,
                    Some("Optional mutable account".to_string()),
                ),
                (
                    4u32,
                    "optional_signer",
                    false,
                    true,
                    false,
                    true,
                    Some("Optional signer".to_string()),
                ),
                (
                    5u32,
                    "optional_mut_signer",
                    true,
                    true,
                    false,
                    true,
                    Some("Optional mutable signer".to_string()),
                ),
            ]),
        )
    }
}
impl<'info> ComplexAccounts<'info> {
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
        if accounts.len() < 6usize {
            {
                ::std::rt::panic_fmt(
                    format_args!(
                        "Expected at least {0} accounts, got {1}", 6usize, accounts
                        .len(),
                    ),
                );
            };
        }
        let account_struct = Self {
            payer: &accounts[0usize],
            data: &accounts[1usize],
            read_only: &accounts[2usize],
            optional_mut: if accounts[3usize].key == &crate::ID {
                None
            } else {
                Some(&accounts[3usize])
            },
            optional_signer: if accounts[4usize].key == &crate::ID {
                None
            } else {
                Some(&accounts[4usize])
            },
            optional_mut_signer: if accounts[5usize].key == &crate::ID {
                None
            } else {
                Some(&accounts[5usize])
            },
        };
        ::shank::Context {
            accounts: account_struct,
            remaining_accounts: &accounts[6usize..],
        }
    }
}
fn main() {}
