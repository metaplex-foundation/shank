use shank::ShankAccounts;
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
fn main() {}
