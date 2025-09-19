use shank::ShankAccounts;
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
fn main() {}
