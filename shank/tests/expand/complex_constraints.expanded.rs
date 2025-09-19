use shank::ShankAccounts;
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
fn main() {}
