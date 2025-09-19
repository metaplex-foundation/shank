use shank::ShankAccounts;
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
fn main() {}
