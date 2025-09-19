use shank::ShankAccounts;
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
fn main() {}
