use shank::ShankAccounts;
pub const ID: [u8; 32] = [1; 32];
pub struct EmptyAccounts {}
impl EmptyAccounts {
    #[doc(hidden)]
    pub fn __shank_accounts() -> Vec<
        (u32, &'static str, bool, bool, bool, bool, Option<String>),
    > {
        ::alloc::vec::Vec::new()
    }
}
impl EmptyAccounts {}
fn main() {}
