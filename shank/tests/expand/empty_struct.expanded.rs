use shank::ShankAccounts;
pub struct EmptyAccounts {}
impl EmptyAccounts {
    #[doc(hidden)]
    pub fn __shank_accounts() -> Vec<
        (u32, &'static str, bool, bool, bool, bool, Option<String>),
    > {
        ::alloc::vec::Vec::new()
    }
}
fn main() {}
